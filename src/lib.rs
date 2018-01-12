extern crate futures;
extern crate hyper;
extern crate tokio_core;

mod mesos {

    use hyper;
    use futures::{Poll, Stream};

    pub struct RecordIoConnection {
        pub body: hyper::Body,
    }

    impl Stream for RecordIoConnection {
        type Item=hyper::Chunk;
        type Error=hyper::error::Error;

        fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
            self.body.poll()
        }
    }
}

#[cfg(test)]
mod tests {

    use std::io::{self, Write};

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
			println!("Response: {}", res.status());
            mesos::RecordIoConnection { body: res.body() }
		});

        let s: mesos::RecordIoConnection = core.run(work).unwrap();
        let w = s.for_each(|chunk| {
            io::stdout().write_all(&chunk).map_err(From::from)
        });

        core.run(w).unwrap();

        assert_eq!(2 + 2, 4);
    }
}
