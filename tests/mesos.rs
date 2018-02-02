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
    use hyper::header::{qitem, Accept, ContentType};
    use mime;
    use async_mesos::client;
    use async_mesos::client::{Decoder, Events};
    use async_mesos::mesos;
    use async_mesos::scheduler;
    use protobuf::core::{parse_from_bytes, Message};
    use protobuf::repeated::RepeatedField;
    use protobuf::stream::CodedInputStream;
    use spectral::prelude::*;
    use tokio_core::reactor::Core;

    #[test]
    fn connect() {
        let mut core = Core::new().unwrap();
        let handle = core.handle();

        // Mesos message
        let mut framework_info = mesos::FrameworkInfo::new();
        framework_info.set_user(String::from("foo"));
        framework_info.set_name(String::from("Example FOO Framework"));
        //framework_info.set_roles(RepeatedField::from_vec(vec![String::from("test")]));

        // Create client
        let uri = "http://localhost:5050/api/v1/scheduler"
            .parse::<Uri>()
            .unwrap();
        let client = Client.connect(&handle, uri, framework_info);

        let events: Events = core.run(client).unwrap().events;
        let w = events.map(|event| event.get_field_type()).take(2).collect();

        let result = core.run(w).unwrap();
        assert_that(&result).is_equal_to(vec![
            scheduler::Event_Type::SUBSCRIBED,
            scheduler::Event_Type::HEARTBEAT,
        ]);
    }
}
