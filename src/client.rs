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
            Ok(Async::Ready(None))
        }
    }
}

#[derive(Clone, Debug)]
pub struct Client {
    pub uri: hyper::Uri,
    pub framework_id: String,
    pub stream_id: String,
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
    ) -> Box<Future<Item = (Self, Events), Error = failure::Error>> {
        // Mesos subscribe call
        let call = Self::subscribe(framework_info);
        let request = Self::request_for(uri.clone(), call, None);

        // Call Mesos
        let http_client = hyper::Client::configure().keep_alive(false).build(&handle);
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
                let maybe_client = Self::new(subscribed_event, stream_id, uri);
                future::result(maybe_client.map(|client| (client, events)))
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

    fn mesos_framework_id(&self) -> mesos::FrameworkID {
        let mut id = mesos::FrameworkID::new();
        id.set_value(self.framework_id.clone());
        id
    }

    pub fn teardown(&self) -> scheduler::Call {
        let mut call = scheduler::Call::new();
        call.set_framework_id(self.mesos_framework_id());
        call.set_field_type(scheduler::Call_Type::TEARDOWN);
        call
    }

    /// Construct Accept call.
    ///
    /// # Arguments
    ///
    ///  * `offer_ids` - Vector over ids of offers that are accepted.
    ///  * `operations` - The operations to perform on offers.
    pub fn accept(
        &self,
        offer_ids: Vec<mesos::OfferID>,
        operations: Vec<mesos::Offer_Operation>,
    ) -> scheduler::Call {
        let mut call = scheduler::Call::new();
        let mut accept = scheduler::Call_Accept::new();

        let mut filters = mesos::Filters::new();
        filters.set_refuse_seconds(5.0);

        accept.set_offer_ids(RepeatedField::from_vec(offer_ids));
        accept.set_operations(RepeatedField::from_vec(operations));
        accept.set_filters(filters);

        call.set_framework_id(self.mesos_framework_id());
        call.set_accept(accept);
        call.set_field_type(scheduler::Call_Type::ACCEPT);
        call
    }

    /// Construct Acknowledge Call
    ///
    /// # Arguments
    ///
    /// * `update_status` - The task status sent along with the update event.
    pub fn acknowledge(&self, mut update_status: mesos::TaskStatus) -> scheduler::Call {
        let mut call = scheduler::Call::new();
        let mut ack = scheduler::Call_Acknowledge::new();

        ack.set_agent_id(update_status.take_agent_id());
        ack.set_task_id(update_status.take_task_id());
        ack.set_uuid(update_status.take_uuid());

        call.set_framework_id(self.mesos_framework_id());
        call.set_acknowledge(ack);
        call.set_field_type(scheduler::Call_Type::ACKNOWLEDGE);
        call
    }

    /// Construct a Hyper Request object for given URI and Mesos scheduler call.
    fn request_for(
        uri: hyper::Uri,
        call: scheduler::Call,
        maybe_stream_id: Option<String>,
    ) -> hyper::Request {
        let mut request = hyper::Request::new(hyper::Method::Post, uri);
        request.headers_mut().set(hyper::header::Accept(vec![
            hyper::header::qitem(PROTOBUF_MEDIA_TYPE.clone()),
        ]));
        request
            .headers_mut()
            .set(hyper::header::ContentType(PROTOBUF_MEDIA_TYPE.clone()));
        if let Some(stream_id) = maybe_stream_id {
            request.headers_mut().set(MesosStreamIdHeader(stream_id));
        }

        // TODO: Handle error
        let body = call.write_to_bytes().unwrap();
        request.set_body(body);
        request
    }

    fn log_response_body(
        chunks: Result<Vec<hyper::Chunk>, hyper::Error>,
    ) -> Result<(), failure::Error> {
        chunks
            .map_err(failure::Error::from)
            .map(|chunks: Vec<hyper::Chunk>| {
                for chunk in chunks {
                    debug!("{}", String::from_utf8_lossy(&chunk));
                }
            })
    }

    pub fn call(
        &self,
        handle: &Handle,
        call: scheduler::Call,
    ) -> Box<Future<Item = (), Error = failure::Error>> {
        let request = Client::request_for(self.uri.clone(), call, Some(self.stream_id.clone()));

        // TODO: Construct only once.
        let http_client = hyper::Client::new(&handle);
        let s = http_client
            .request(request)
            .map_err(|error| {
                error!("Teardown call failed");
                failure::Error::from(error)
            })
            .and_then(|res| {
                debug!("Mesos teardown response status: {}", res.status());
                res.body().collect().then(Self::log_response_body)
            });
        // TODO: Do not return boxed future.
        Box::new(s)
    }

    /// Construct a new Mesos client from subcribe event, remaining Mesos events and Mesos streamd id.
    fn new(
        maybe_event: Option<scheduler::Event>,
        stream_id: String,
        uri: hyper::Uri,
    ) -> Result<Self, Error> {
        if let Some(event) = maybe_event {
            if event.has_subscribed() {
                let framework_id = event.get_subscribed().get_framework_id().clone();

                Ok(Self {
                    uri,
                    framework_id: framework_id.get_value().into(),
                    stream_id,
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
    use client::{Client, Decoder};
    use hyper;
    use mesos;
    use model;
    use protobuf::Message;
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
        let uri = "http://localhost:5050/api/v1/scheduler"
            .parse::<hyper::Uri>()
            .unwrap();
        let result = Client::new(None, String::from("some stream id"), uri);

        assert_that(&result).is_err();
    }

    #[test]
    fn wrong_event() {
        let mut event = scheduler::Event::new();
        event.set_field_type(scheduler::Event_Type::HEARTBEAT);
        let uri = "http://localhost:5050/api/v1/scheduler"
            .parse::<hyper::Uri>()
            .unwrap();
        let result = Client::new(Some(event), String::from("some stream id"), uri);

        assert_that(&result).is_err();
    }

    #[test]
    fn accept_call() {
        let framework_id = String::from("my_framework");
        let stream_id = String::from("teh_stream");
        let uri = "http://localhost:5050/api/v1/scheduler"
            .parse::<hyper::Uri>()
            .expect("Test URI was not parsable.");
        let client = Client {
            uri: uri.clone(),
            framework_id,
            stream_id,
        };

        let mut offer_id = mesos::OfferID::new();
        offer_id.set_value(String::from("some_offer"));

        let mut agent_id = mesos::AgentID::new();
        agent_id.set_value(String::from("an_agent"));

        let mut task_id = mesos::TaskID::new();
        task_id.set_value(String::from("my_task"));

        let resource_cpu = model::ScalarResourceBuilder::default()
            .name("cpu")
            .value(0.1)
            .build()
            .expect("CPU resource was not complete");

        let resource_mem = model::ScalarResourceBuilder::default()
            .name("mem")
            .value(32.0)
            .build()
            .expect("Memory resource was not complete");

        let command = model::ShellCommandBuilder::default()
            .command("sleep 100000")
            .build()
            .expect("Shell command was not complete");

        let task_info = model::TaskInfoBuilder::default()
            .name("my_task")
            .task_id(task_id)
            .agent_id(agent_id)
            .resource(resource_cpu)
            .resource(resource_mem)
            .command(command)
            .build()
            .expect("Task info was not complete");

        let operation = model::OfferLaunchOperationBuilder::default()
            .task_info(task_info)
            .build()
            .expect("Offer operation was not complete");

        let call = client.accept(vec![offer_id], vec![operation]);
        asserting(&"Accept call is initialized")
            .that(&call.is_initialized())
            .is_true();

        Client::request_for(uri, call, None);
    }
}
