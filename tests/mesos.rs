extern crate async_mesos;
extern crate futures;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate mime;
extern crate protobuf;
extern crate simple_logger;
extern crate spectral;
extern crate tokio_core;

#[cfg(test)]
mod integration {

    use futures::Future;
    use futures::Stream;
    use hyper::Uri;
    use async_mesos::client::{Client, Events};
    use async_mesos::mesos;
    use async_mesos::scheduler;
    use simple_logger;
    use spectral::prelude::*;
    use tokio_core::reactor::Core;

    #[test]
    fn connect() {
        simple_logger::init().unwrap();

        let mut core = Core::new().unwrap();
        let handle = core.handle();

        // Mesos message
        let mut framework_info = mesos::FrameworkInfo::new();
        framework_info.set_user(String::from("foo"));
        framework_info.set_name(String::from("Example FOO Framework"));

        // Create client
        let uri = "http://localhost:5050/api/v1/scheduler"
            .parse::<Uri>()
            .unwrap();
        let client = Client::connect(&handle, uri, framework_info);

        let work = client
            .into_stream()
            .map(|client| client.events)
            .flatten()
            .map(|event| event.get_field_type())
            .take(1)
            .collect();

        let result = core.run(work).unwrap();
        assert_that(&result).is_equal_to(vec![scheduler::Event_Type::HEARTBEAT]);
    }

    struct State {
        framework_id: String,
    }

    #[test]
    fn task_launch() {
        simple_logger::init().unwrap();

        let mut core = Core::new().unwrap();
        let handle = core.handle();

        // Mesos message
        let mut framework_info = mesos::FrameworkInfo::new();
        framework_info.set_user(String::from("foo"));
        framework_info.set_name(String::from("Example FOO Framework"));

        // Create client
        let uri = "http://localhost:5050/api/v1/scheduler"
            .parse::<Uri>()
            .unwrap();
        let future_client = Client::connect(&handle, uri, framework_info);

        let mut state = None;

        // Process events and start and stop a simple task.
        let work = future_client
            .into_stream()
            .map(|client| {
                // TODO: We should capture a client session or something similar.
                state = Some(State {
                    framework_id: client.framework_id.clone(),
                });
                client.events
            })
            .flatten()
            .for_each(|event| match event.get_field_type() {
                scheduler::Event_Type::OFFERS => {
                    info!("Received offer.");
                    // Client::accept(state.framework_id, ...) -> Future
                    Ok(())
                }
                other => {
                    debug!("Ignore event {:?}", other);
                    Ok(())
                }
            });

        core.run(work).unwrap();
    }
}
