extern crate async_mesos;
extern crate futures;
extern crate hyper;
extern crate spectral;
extern crate tokio_core;

#[cfg(test)]
mod integration {

    use futures::{Future, Stream};
    use hyper::Client;
    use hyper::Uri;
    use async_mesos::client;
    use async_mesos::client::{Decoder, RecordIoConnection};
    use spectral::prelude::*;
    use std::str;
    use tokio_core::reactor::Core;

    #[test]
    fn connect() {
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let client = Client::new(&handle);

        let uri = "http://localhost:5050".parse::<Uri>().unwrap();
        let work = client.get(uri).map(|res| {
            println!("Response status: {}", res.status());
            return RecordIoConnection::new(res.body());
        });

        let records: RecordIoConnection = core.run(work).unwrap();
        let w = records.for_each(|bytes| {
            println!("{}", str::from_utf8(&bytes)?);
            Ok(())
        });

        core.run(w).unwrap();
    }
}
