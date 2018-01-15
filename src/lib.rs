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

    pub struct RecordIoConnection {
        pub buf: BytesMut,
        pub body: hyper::Body,
    }

    impl Stream for RecordIoConnection {
        type Item=String;
        type Error=hyper::error::Error;

        fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
            if let Some(chunk) = try_ready!(self.body.poll()) {
                self.buf.put(chunk.as_ref());
                // Parse line for now.
                println!("New line?");
                if let Some(i) = self.buf.iter().position(|&b| b == b'\n') {
                    println!("...yes");
                    let line = self.buf.split_to(i);
                    match str::from_utf8(&line) {
                        Ok(s) => {
                            return Ok(Async::Ready(Some(s.to_string())))
                        },
                        Err(e) => {
                            println!("got error");
                            return Err(hyper::Error::Utf8(e))
                        },
                    };
                } else {
                    println!("...no");
                    return Ok(Async::NotReady)
                }
            }
            Ok(Async::NotReady)
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
            mesos::RecordIoConnection { buf: BytesMut::with_capacity(4096), body: res.body() }
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
