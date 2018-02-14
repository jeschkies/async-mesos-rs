extern crate bytes;
extern crate futures;
extern crate spectral;
extern crate tokio_core;

use bytes::{Bytes, BytesMut};
use bytes::buf::BufMut;
use failure;
use failure::Error;
use futures::{future, Async, Future, Poll, Stream};
use hyper;
use mesos;
use mime;
use protobuf::core::{parse_from_bytes, Message};
use protobuf::repeated::RepeatedField;
use scheduler;
use tokio_core::reactor::Handle;

use std::{fmt, str};

#[derive(Fail, Debug)]
pub enum DecoderError {
    #[fail(display = "Could not decode RecordIO frame.")] Frame,
}

pub trait Decoder {
    type Item;
    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Error>;
}

#[derive(Debug, PartialEq)]
pub enum RecordIoDecoderState {
    TrimWhitespaces,
    ReadLength,
    ReadRecord { len: u64 },
}

/// Decoder for [RecordIO](http://mesos.apache.org/documentation/latest/scheduler-http-api/#recordio-response-format-1)
/// format.
pub struct RecordIoDecoder {
    state: RecordIoDecoderState,
}
impl RecordIoDecoder {
    pub fn new() -> Self {
        Self {
            state: RecordIoDecoderState::TrimWhitespaces,
        }
    }

    fn is_whitespace(&self, b: &u8) -> bool {
        (*b == b' ' || *b == b'\n' || *b == b'\r' || *b == b'\t')
    }

    pub fn trim_whitespaces(&mut self, buf: &mut BytesMut) -> RecordIoDecoderState {
        let whitespaces: usize = buf.iter().take_while(|&b| self.is_whitespace(b)).count();
        buf.split_to(whitespaces);
        RecordIoDecoderState::ReadLength
    }

    pub fn decode_length(
        &mut self,
        buf: &mut BytesMut,
    ) -> Result<RecordIoDecoderState, failure::Error> {
        if let Some(i) = buf.iter().position(|&b| b == b'\n') {
            let length = buf.split_to(i);
            buf.split_to(1);

            let s = str::from_utf8(&length)?;
            let length: u64 = s.parse()?;
            Ok(RecordIoDecoderState::ReadRecord { len: length })
        } else {
            Ok(RecordIoDecoderState::ReadLength)
        }
    }

    pub fn decode_record(
        &mut self,
        length: u64,
        buf: &mut BytesMut,
    ) -> (RecordIoDecoderState, Option<Bytes>) {
        if (buf.len() as u64) < length {
            return (RecordIoDecoderState::ReadRecord { len: length }, None);
        } else {
            let record_buf = buf.split_to(length as usize);
            return (
                RecordIoDecoderState::TrimWhitespaces,
                Some(record_buf.freeze()),
            );
        }
    }
}
impl Decoder for RecordIoDecoder {
    type Item = Bytes;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Bytes>, Error> {
        while buf.len() > 0 {
            match self.state {
                RecordIoDecoderState::TrimWhitespaces => {
                    self.state = self.trim_whitespaces(buf);
                }
                RecordIoDecoderState::ReadLength => {
                    self.state = self.decode_length(buf)?;
                }
                RecordIoDecoderState::ReadRecord { len } => {
                    let (new_state, record) = self.decode_record(len, buf);
                    self.state = new_state;
                    return Ok(record);
                }
            }
        }
        return Ok(None);
    }
}

pub struct RecordIoConnection {
    pub buf: BytesMut,
    pub body: hyper::Body,
    decoder: RecordIoDecoder,
}

impl RecordIoConnection {
    pub fn new(body: hyper::Body) -> Self {
        Self {
            buf: BytesMut::with_capacity(8192),
            body: body,
            decoder: RecordIoDecoder::new(),
        }
    }

    /// Process all bytes in RecordIoConnection::buf.
    fn drain(&mut self) -> Poll<Option<Bytes>, Error> {
        if let Some(record) = self.decoder.decode(&mut self.buf)? {
            Ok(Async::Ready(Some(record)))
        } else {
            Ok(Async::Ready(None))
        }
    }
}

impl Stream for RecordIoConnection {
    type Item = Bytes; // TODO: RecordIoDecoder::Item
    type Error = failure::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self.body.poll() {
            Ok(Async::Ready(Some(chunk))) => {
                if chunk.len() <= self.buf.remaining_mut() {
                    // Potential deadlock. If we cannot parse and the buffer is
                    // full we should give an error.
                    self.buf.put(chunk.as_ref());
                } else {
                    error!(
                        "Not enough buffer space left {}, {}",
                        self.buf.remaining_mut(),
                        chunk.len()
                    );
                }
                if let Some(record) = try!(self.decoder.decode(&mut self.buf)) {
                    return Ok(Async::Ready(Some(record)));
                } else {
                    return Ok(Async::NotReady);
                }
            }
            Err(e) => return Err(From::from(e)),
            Ok(Async::Ready(None)) => return self.drain(),
            Ok(Async::NotReady) => return Ok(Async::NotReady),
        }
    }
}

pub struct Events {
    records: RecordIoConnection,
}

impl Events {
    pub fn new(body: hyper::Body) -> Self {
        Self {
            records: RecordIoConnection::new(body),
        }
    }
}

impl Stream for Events {
    type Item = scheduler::Event;
    type Error = failure::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if let Some(record) = try_ready!(self.records.poll()) {
            let event: scheduler::Event = parse_from_bytes(&record)?;
            Ok(Async::Ready(Some(event)))
        } else {
            Ok(Async::NotReady)
        }
    }
}

pub struct Client {
    pub framework_id: String,
    stream_id: String,
    pub events: Events,
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Client {{ framework_id: {}, stream_id: {} }}",
            self.framework_id, self.stream_id
        )
    }
}

header! { (MesosStreamIdHeader, "Mesos-Stream-Id") => [String] }
lazy_static! {
    static ref PROTOBUF_MEDIA_TYPE: mime::Mime = "application/x-protobuf".parse::<mime::Mime>().unwrap();
}

impl Client {
    pub fn connect(
        handle: &Handle,
        uri: hyper::Uri,
        framework_info: mesos::FrameworkInfo,
    ) -> Box<Future<Item = Self, Error = failure::Error>> {
        // Mesos subscribe call
        let call = Self::subscribe(framework_info);
        let request = Self::request_for(uri, call);

        // Call Mesos
        let http_client = hyper::Client::new(&handle);
        let s = http_client
            .request(request)
            .map_err(failure::Error::from)
            .and_then(move |res: hyper::Response| {
                debug!("Mesos subscribe response status: {}", res.status());

                let stream_id = if let Some(header) = res.headers().get::<MesosStreamIdHeader>() {
                    future::ok(header.0.clone())
                } else {
                    // TODO: Use different error type.
                    future::err(format_err!("Missing Mesos-Stream-Id header."))
                };
                let events = Events::new(res.body())
                    .into_future()
                    .map_err(|(err, _)| failure::Error::from(err));
                events.join(stream_id)
            })
            .and_then(|((subscribed_event, events), stream_id)| {
                future::result(Self::new(subscribed_event, events, stream_id))
            });
        Box::new(s)
    }

    pub fn subscribe(framework_info: mesos::FrameworkInfo) -> scheduler::Call {
        let mut call = scheduler::Call::new();
        let mut subscribe = scheduler::Call_Subscribe::new();
        subscribe.set_framework_info(framework_info);
        call.set_subscribe(subscribe);
        call.set_field_type(scheduler::Call_Type::SUBSCRIBE);
        call
    }

    /// Construct Accept call.
    ///
    /// # Arguments
    ///
    ///  * `framework_id` - Id of this registered client.
    ///  * `offer_ids` - Vector over ids of offers that are accepted.
    ///  * `operations` - The operations to perform on offers.
    pub fn accept(
        framework_id: String,
        offer_ids: Vec<mesos::OfferID>,
        operations: Vec<mesos::Offer_Operation>,
    ) -> scheduler::Call {
        let mut call = scheduler::Call::new();
        let mut accept = scheduler::Call_Accept::new();
        accept.set_offer_ids(RepeatedField::from_vec(offer_ids));
        accept.set_operations(RepeatedField::from_vec(operations));

        let mut id = mesos::FrameworkID::new();
        id.set_value(framework_id);
        call.set_framework_id(id);
        call.set_accept(accept);
        call.set_field_type(scheduler::Call_Type::ACCEPT);
        call
    }

    /// Construct offer launch operation.
    ///
    /// # Argument
    ///
    /// * `task_infos` - Description of the tasks to launch.
    pub fn launch_operation(task_infos: Vec<mesos::TaskInfo>) -> mesos::Offer_Operation {
        let mut operation = mesos::Offer_Operation::new();
        let mut launch = mesos::Offer_Operation_Launch::new();
        launch.set_task_infos(RepeatedField::from_vec(task_infos));

        operation.set_launch(launch);
        operation.set_field_type(mesos::Offer_Operation_Type::LAUNCH);
        operation
    }

    /// Builds a Hyper Request object for given URI and Mesos scheduler call.
    pub fn request_for(uri: hyper::Uri, call: scheduler::Call) -> hyper::Request {
        let mut request = hyper::Request::new(hyper::Method::Post, uri);
        request.headers_mut().set(hyper::header::Accept(vec![
            hyper::header::qitem(PROTOBUF_MEDIA_TYPE.clone()),
        ]));
        request
            .headers_mut()
            .set(hyper::header::ContentType(PROTOBUF_MEDIA_TYPE.clone()));

        // TODO: Handle error
        let body = call.write_to_bytes().unwrap();
        request.set_body(body);
        request
    }

    /// Builds a new Mesos client from subcribe event, remaining Mesos events and Mesos streamd id.
    fn new(
        maybe_event: Option<scheduler::Event>,
        events: Events,
        stream_id: String,
    ) -> Result<Self, Error> {
        if let Some(event) = maybe_event {
            if event.has_subscribed() {
                let framework_id = event.get_subscribed().get_framework_id().clone();

                Ok(Self {
                    framework_id: framework_id.get_value().into(),
                    stream_id,
                    events,
                })
            } else {
                Err(format_err!(
                    "Mesos {:?} event was not a SUBSCRIBED event",
                    event.get_field_type()
                ))
            }
        } else {
            Err(format_err!("Did not receive Mesos SUBSCRIBED event."))
        }
    }
}

#[cfg(test)]
mod tests {

    use bytes::{BufMut, Bytes, BytesMut};
    use client;
    use client::{Client, Decoder, Events};
    use hyper;
    use scheduler;
    use spectral::prelude::*;

    #[test]
    fn trim_whitespaces() {
        let mut buffer = BytesMut::with_capacity(1024);
        buffer.put(&b"\t\n \r121\n{\"type\":\"HEARTBEAT\"}"[..]);
        let mut decoder = client::RecordIoDecoder::new();

        let state = decoder.trim_whitespaces(&mut buffer);
        assert_eq!(state, client::RecordIoDecoderState::ReadLength);
        assert_eq!(buffer, "121\n{\"type\":\"HEARTBEAT\"}");
    }

    #[test]
    fn decode_length() {
        let mut buffer = BytesMut::with_capacity(1024);
        buffer.put(&b"121\n"[..]);
        let mut decoder = client::RecordIoDecoder::new();

        let state = decoder.decode_length(&mut buffer);
        assert_that(&state)
            .is_ok()
            .is_equal_to(client::RecordIoDecoderState::ReadRecord { len: 121 });
    }

    #[test]
    fn decode_length_error() {
        let mut buffer = BytesMut::with_capacity(1024);
        buffer.put(&b"1f1\n"[..]);
        let mut decoder = client::RecordIoDecoder::new();

        let state = decoder.decode_length(&mut buffer);
        assert_that(&state).is_err();
    }

    #[test]
    fn decode_length_invalid() {
        let mut buffer = BytesMut::with_capacity(1024);
        buffer.put(&b"-42\n"[..]);
        let mut decoder = client::RecordIoDecoder::new();

        let state = decoder.decode_length(&mut buffer);
        assert_that(&state).is_err();
    }

    #[test]
    fn decode_record() {
        let mut buffer = BytesMut::with_capacity(1024);
        buffer.put(&b"{\"type\":\"HEARTBEAT\"}"[..]);
        let mut decoder = client::RecordIoDecoder::new();

        let (state, record) = decoder.decode_record(20, &mut buffer);
        assert_eq!(state, client::RecordIoDecoderState::TrimWhitespaces);
        assert_that(&record)
            .is_some()
            .is_equal_to(Bytes::from("{\"type\":\"HEARTBEAT\"}"));
    }

    #[test]
    fn not_decode_record() {
        let mut buffer = BytesMut::with_capacity(1024);
        buffer.put(&b"{\"type\":\"HEARTBEAT\"}"[..]);
        let mut decoder = client::RecordIoDecoder::new();

        let (state, record) = decoder.decode_record(42, &mut buffer);
        assert_eq!(state, client::RecordIoDecoderState::ReadRecord { len: 42 });
        assert_that(&record).is_none();
    }

    #[test]
    fn decode_multiple_records() {
        let mut buffer = BytesMut::with_capacity(1024);
        let body = b"\t  \r\n121\n{\"type\": \"SUBSCRIBED\",\"subscribed\": {\"framework_id\": {\"value\":\"12220-3440-12532-2345\"},\"heartbeat_interval_seconds\":15.0}\
            \t \r\n 20\n{\"type\":\"HEARTBEAT\"}";
        buffer.put(&body[..]);
        let mut decoder = client::RecordIoDecoder::new();

        let first = decoder.decode(&mut buffer);
        let expected = Bytes::from(
            "{\"type\": \"SUBSCRIBED\",\"subscribed\": {\"framework_id\": {\"value\":\"12220-3440-12532-2345\"},\"heartbeat_interval_seconds\":15.0}",
        );
        assert_that(&first).is_ok().is_some().is_equal_to(expected);

        let second = decoder.decode(&mut buffer);
        assert_that(&second)
            .is_ok()
            .is_some()
            .is_equal_to(Bytes::from("{\"type\":\"HEARTBEAT\"}"));

        let third = decoder.decode(&mut buffer);
        assert_that(&third).is_ok().is_none();
    }

    #[test]
    fn no_subscribe_event() {
        let events = Events::new(hyper::Body::empty());
        let result = Client::new(None, events, String::from("some stream id"));

        assert_that(&result).is_err();
    }

    #[test]
    fn wrong_event() {
        let events = Events::new(hyper::Body::empty());
        let mut event = scheduler::Event::new();
        event.set_field_type(scheduler::Event_Type::HEARTBEAT);
        let result = Client::new(Some(event), events, String::from("some stream id"));

        assert_that(&result).is_err();
    }
}
