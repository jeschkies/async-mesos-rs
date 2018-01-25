
extern crate bytes;
extern crate futures;
extern crate hyper;
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
    use tokio_core::reactor::Core;

    #[test]
    fn decode_lines() {
        let mut buffer = BytesMut::with_capacity(1024);
        buffer.put(&b"hello\nworld\n"[..]);
        let mut decoder = mesos::LineDecoder {};

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
