extern crate futures;
extern crate hyper;
extern crate tokio_core;

#[cfg(test)]
mod tests {

    use futures::Future;
	use hyper::Client;
    use hyper::Uri;
	use tokio_core::reactor::Core;

    #[test]
    fn it_works() {
		let mut core = Core::new().unwrap();
        let handle = core.handle();
        let client = Client::new(&handle);

		let uri = "http://httpbin.org/ip".parse::<Uri>().unwrap();
		let work = client.get(uri).map(|res| {
			println!("Response: {}", res.status());
		}).map(|_| {
            println!("\nDone.");
        });

        core.run(work).unwrap();

        assert_eq!(2 + 2, 4);
    }
}
