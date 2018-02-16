extern crate async_mesos;
#[macro_use]
extern crate failure;
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

    use failure;
    use futures::{future, stream, Future, Stream};
    use hyper;
    use hyper::Uri;
    use async_mesos::client::{Client, Events};
    use async_mesos::mesos;
    use async_mesos::scheduler;
    use simple_logger;
    use spectral::prelude::*;
    use std;
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

        // Process events and start and stop a simple task.
        let work = future_client
            .into_stream()
            .map(|client| {
                let ids = stream::repeat::<_, failure::Error>(client.framework_id.clone());
                client.events.zip(ids)
            })
            .flatten()
            .for_each(
                |(mut event, framework_id)| -> Box<Future<Item = _, Error = failure::Error>> {
                    match event.get_field_type() {
                        scheduler::Event_Type::OFFERS => {
                            info!("Received offer.");

                            // Create task for offer.
                            let mut offer = event.take_offers().take_offers()[0].clone();
                            let offer_id = offer.take_id();
                            let agent_id = offer.take_agent_id();

                            let mut task_id = mesos::TaskID::new();
                            task_id.set_value(String::from("my_task"));

                            let resource_cpu = Client::resource_cpu(0.1);
                            let resource_mem = Client::resource_mem(32.0);
                            let resources = vec![resource_cpu, resource_mem];

                            let executor = Client::executor_shell(
                                String::from("default"),
                                String::from("sleep 100000"),
                            );
                            let task_info = Client::task_info(
                                String::from("sleep_task"),
                                task_id,
                                agent_id,
                                resources,
                                executor,
                            );
                            let operation = Client::launch_operation(vec![task_info]);
                            let call =
                                Client::accept(framework_id, vec![offer_id], vec![operation]);

                            // Make call
                            let uri = "http://localhost:5050/api/v1/scheduler"
                                .parse::<Uri>()
                                .unwrap();
                            let request = Client::request_for(uri, call);
                            let http_client = hyper::Client::new(&handle);
                            let s = http_client
                                .request(request)
                                .map_err(|error| {
                                    error!("Accept call failed");
                                    failure::Error::from(error)
                                })
                                .and_then(|res| {
                                    debug!("Mesos accept offer response status: {}", res.status());
                                    res.body().collect()
                                        .map_err(failure::Error::from)
                                        .map(|chunks: Vec<hyper::Chunk>|{
                                            for chunk in chunks {
                                                debug!("{}", String::from_utf8_lossy(&chunk));
                                            }
                                        ()
                                    })
                                });
                            Box::new(s)
                        }
                        other => {
                            debug!("Ignore event {:?}", other);
                            Box::new(future::result(Ok(())))
                        }
                    }
                },
            );

        core.run(work).unwrap();
    }
}
