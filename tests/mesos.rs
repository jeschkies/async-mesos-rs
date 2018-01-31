extern crate async_mesos;
extern crate futures;
extern crate hyper;
extern crate mime;
extern crate protobuf;
extern crate spectral;
extern crate tokio_core;

#[cfg(test)]
mod integration {

    use futures::{Future, Stream};
    use hyper::{Client, Method, Request, Uri};
    use hyper::header::{Accept, ContentType, qitem};
    use mime;
    use async_mesos::client;
    use async_mesos::client::{Decoder, RecordIoConnection};
    use async_mesos::mesos;
    use async_mesos::scheduler;
    use protobuf::core::{parse_from_bytes, Message};
    use protobuf::repeated::RepeatedField;
    use protobuf::stream::CodedInputStream;
    use spectral::prelude::*;
    use std::str;
    use tokio_core::reactor::Core;

    #[test]
    fn connect() {
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let client = Client::new(&handle);

        // Mesos message
        let mut framework_info = mesos::FrameworkInfo::new();
        framework_info.set_user(String::from("foo"));
        framework_info.set_name(String::from("Example FOO Framework"));
        framework_info.set_roles(RepeatedField::from_vec(vec![String::from("test")]));
        let mut call = scheduler::Call::new();
        let mut subscribe = scheduler::Call_Subscribe::new();
        subscribe.set_framework_info(framework_info);
        call.set_subscribe(subscribe);
        call.set_field_type(scheduler::Call_Type::SUBSCRIBE);

        // Build request
        let uri = "http://localhost:5050/api/v1/scheduler".parse::<Uri>().unwrap();
        let mut request = Request::new(Method::Post, uri);
        let protobuf_media_type = "application/x-protobuf".parse::<mime::Mime>().unwrap();
        request.headers_mut().set(Accept(vec![qitem(protobuf_media_type.clone())]));
        request.headers_mut().set(ContentType(protobuf_media_type));
        request.set_body(call.write_to_bytes().unwrap());

        // Fire off request
        let work = client.request(request).map(|res| {
            println!("Response status: {}", res.status());
            return RecordIoConnection::new(res.body());
        });

        let records: RecordIoConnection = core.run(work).unwrap();
        let w = records.for_each(|bytes| {
            let event: scheduler::Event = parse_from_bytes(&bytes)?;
            println!("Got event {:?}", event.get_field_type());
            Ok(())
        });

        core.run(w).unwrap();
    }
}
