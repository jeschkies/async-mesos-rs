extern crate bytes;
#[macro_use] extern crate failure;
extern crate futures;
extern crate hyper;
extern crate spectral;
extern crate tokio_core;

mod mesos;

#[cfg(test)]
mod experiments {

    use bytes::{BufMut, BytesMut};
    use futures::{Future, Stream};
    use hyper::Client;
    use hyper::Uri;
    use mesos;
    use mesos::{Decoder, RecordIoDecoderState};
    use spectral::prelude::*;
    use tokio_core::reactor::Core;

    #[test]
    fn decode_lines() {
        let mut buffer = BytesMut::with_capacity(1024);
        buffer.put(&b"hello\nworld\n"[..]);
        let mut decoder = mesos::LineDecoder {};

        let first = decoder.decode(&mut buffer);
        assert_that(&first).is_ok().is_some();

        let second = decoder.decode(&mut buffer);
        assert_that(&second).is_ok().is_some();

        let third = decoder.decode(&mut buffer);
        assert_that(&third).is_ok().is_none();
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
            return mesos::RecordIoConnection::new(res.body());
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
