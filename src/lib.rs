extern crate bytes;
#[macro_use]
extern crate futures;
extern crate hyper;
extern crate tokio_core;

mod mesos {

    use bytes::BytesMut;
    use bytes::buf::BufMut;
    use hyper;
    use futures::{Async, Poll, Stream};

    use std::str;

    trait Decoder {
        fn decode(&mut self, buf: &mut BytesMut) -> Option<String>;
    }

    pub struct LineDecoder;
    impl Decoder for LineDecoder {

        fn decode(&mut self, buf: &mut BytesMut) -> Option<String> {
            // Parse line for now.
            if let Some(i) = buf.iter().position(|&b| b == b'\n') {
                let line = buf.split_to(i);
                buf.split_to(1);

                match str::from_utf8(&line) {
                    Ok(s) => return Some(s.to_string()),
                    Err(e) => {
                        println!("got error");
                        return None
                    },
                };
            }
            None
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

    use bytes::BytesMut;

    use futures::{Future, Stream};
	use hyper::Client;
    use hyper::Uri;
    use mesos;
	use tokio_core::reactor::Core;

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
