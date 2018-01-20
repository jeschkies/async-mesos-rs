extern crate bytes;
extern crate futures;
extern crate hyper;
extern crate tokio_core;

mod mesos {

    use bytes::{Bytes, BytesMut};
    use bytes::buf::BufMut;
    use hyper;
    use futures::{Async, Poll, Stream};

    use std::str;

    pub trait Decoder<T> {
        fn decode(&mut self, buf: &mut BytesMut) -> Option<T>;
    }

    /// Decodes lines of body stream.
    pub struct LineDecoder;
    impl Decoder<String> for LineDecoder {

        fn decode(&mut self, buf: &mut BytesMut) -> Option<String> {
            // Parse line for now.
            if let Some(i) = buf.iter().position(|&b| b == b'\n') {
                let line = buf.split_to(i);
                buf.split_to(1);

                match str::from_utf8(&line) {
                    Ok(s) => return Some(s.to_string()),
                    Err(_) => {
                        println!("got error");
                        return None
                    },
                };
            }
            None
        }
    }

    #[derive(Debug)]
    #[derive(PartialEq)]
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
            Self { state: RecordIoDecoderState::TrimWhitespaces }
        }

        fn is_whitespace(&mut self, b: &u8) -> bool {
            (*b == b' ' || *b == b'\n' || *b == b'\r' || *b == b'\t')
        }

        pub fn trim_whitespaces(&mut self, buf: &mut BytesMut) -> RecordIoDecoderState {
            let whitespaces: usize = buf.iter().take_while(|&b| self.is_whitespace(b)).count();
            buf.split_to(whitespaces);
            RecordIoDecoderState::ReadLength
        }

        pub fn decode_length(&mut self, buf: &mut BytesMut) -> RecordIoDecoderState {
            if let Some(i) = buf.iter().position(|&b| b == b'\n') {
                let length = buf.split_to(i);
                buf.split_to(1);

                match str::from_utf8(&length) {
                    Ok(s) => {
                        let length: u64 = s.parse().unwrap();
                        return RecordIoDecoderState::ReadRecord { len: length }
                    },
                    Err(_) => {
                        println!("got error");
                        // TODO: Return error.
                        return RecordIoDecoderState::ReadLength
                    },
                };
            }
            RecordIoDecoderState::ReadLength
        }

        pub fn decode_record(&mut self, length: u64, buf: &mut BytesMut) -> (RecordIoDecoderState, Option<Bytes>) {
            if (buf.len() as u64)  < length {
                return (RecordIoDecoderState::ReadRecord { len: length}, None)
            } else {
                let record_buf = buf.split_to(length as usize);
                return (RecordIoDecoderState::TrimWhitespaces, Some(record_buf.freeze()))
            }
        }
    }
    impl Decoder<Bytes> for RecordIoDecoder {

        fn decode(&mut self, buf: &mut BytesMut) -> Option<Bytes> {
            match self.state {
                RecordIoDecoderState::TrimWhitespaces => {
                    self.state = self.trim_whitespaces(buf);
                    return None
                },
                RecordIoDecoderState::ReadLength => {
                    self.state = self.decode_length(buf);
                    return None
                },
                RecordIoDecoderState::ReadRecord { len } => {
                    let (new_state, record) = self.decode_record(len, buf);
                    self.state = new_state;
                    return record
                },
            }
        }
    }


    pub struct RecordIoConnection {
        pub buf: BytesMut,
        pub body: hyper::Body,
        decoder: LineDecoder,
    }

    impl RecordIoConnection {

        pub fn new(body: hyper::Body) -> Self {
            Self {
                buf: BytesMut::with_capacity(4096),
                body: body,
                decoder: LineDecoder{}
            }
        }

        /// Process all bytes in RecordIoConnection::buf.
        fn drain(&mut self) -> Poll<Option<String>, hyper::error::Error> {
          if let Some(line) = self.decoder.decode(&mut self.buf) {
              Ok(Async::Ready(Some(line)))
          } else {
              Ok(Async::Ready(None))
          }
        }
    }

    impl Stream for RecordIoConnection {
        type Item=String;
        type Error=hyper::error::Error;

        fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
            match self.body.poll() {
                Ok(Async::Ready(Some(chunk))) => {
                    self.buf.put(chunk.as_ref());
                    if let Some(line) = self.decoder.decode(&mut self.buf) {
                        return Ok(Async::Ready(Some(line)))
                    } else {
                        return Ok(Async::NotReady)
                    }
                },
                Err(e) => return Err(From::from(e)),
                Ok(Async::Ready(None)) => return self.drain(),
                Ok(Async::NotReady) => return Ok(Async::NotReady),
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use bytes::{BytesMut, BufMut};

    use futures::{Future, Stream};
	use hyper::Client;
    use hyper::Uri;
    use mesos;
    use mesos::{Decoder, RecordIoDecoderState};
	use tokio_core::reactor::Core;

    #[test]
    fn trim_whitespaces() {
        let mut buffer = BytesMut::with_capacity(1024);
        buffer.put(&b"\t\n \r121\n{\"type\":\"HEARTBEAT\"}"[..]);
        let mut decoder = mesos::RecordIoDecoder::new();

        let state = decoder.trim_whitespaces(&mut buffer);
        assert_eq!(state, mesos::RecordIoDecoderState::ReadLength);
        assert_eq!(buffer, "121\n{\"type\":\"HEARTBEAT\"}");
    }

    #[test]
    fn decode_length() {
        let mut buffer = BytesMut::with_capacity(1024);
        buffer.put(&b"121\n"[..]);
        let mut decoder = mesos::RecordIoDecoder::new();

        let state = decoder.decode_length(&mut buffer);
        assert_eq!(state, mesos::RecordIoDecoderState::ReadRecord { len: 121 });
    }

    #[test]
    fn decode_record() {
        let mut buffer = BytesMut::with_capacity(1024);
        buffer.put(&b"{\"type\":\"HEARTBEAT\"}"[..]);
        let mut decoder = mesos::RecordIoDecoder::new();

        let (state, record) = decoder.decode_record(20, &mut buffer);
        assert_eq!(state, mesos::RecordIoDecoderState::TrimWhitespaces);
        assert_eq!(record.expect("Record was not read."), "{\"type\":\"HEARTBEAT\"}")
    }

    #[test]
    fn not_decode_record() {
        let mut buffer = BytesMut::with_capacity(1024);
        buffer.put(&b"{\"type\":\"HEARTBEAT\"}"[..]);
        let mut decoder = mesos::RecordIoDecoder::new();

        let (state, record) = decoder.decode_record(42, &mut buffer);
        assert_eq!(state, mesos::RecordIoDecoderState::ReadRecord { len: 42 });
        assert_eq!(record.is_some(), false)
    }

    #[test]
    fn decode_lines() {
        let mut buffer = BytesMut::with_capacity(1024);
        buffer.put(&b"hello\nworld\n"[..]);
        let mut decoder = mesos::LineDecoder{};

        let first = decoder.decode(&mut buffer);
        assert_eq!(first.is_some(), true);

        let second = decoder.decode(&mut buffer);
        assert_eq!(second.is_some(), true);

        let third = decoder.decode(&mut buffer);
        assert_eq!(third.is_some(), false);
        assert_eq!(buffer.len(), 0);
    }

    #[test]
    fn it_works() {
		let mut core = Core::new().unwrap();
        let handle = core.handle();
        let client = Client::new(&handle);

		let uri = "http://httpbin.org/ip".parse::<Uri>().unwrap();
		let work = client.get(uri).map(|res| {
			println!("Response status: {}", res.status());
            return mesos::RecordIoConnection::new(res.body())
		});

        let s: mesos::RecordIoConnection = core.run(work).unwrap();
        let w = s.for_each(|line: String| {
            println!("{}", line);
            Ok(())
        });

        core.run(w).unwrap();

        assert_eq!(2 + 2, 4);
    }
}
