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
extern crate users;

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
    use users::{get_current_uid, get_user_by_uid};

    #[test]
    fn connect() {
        simple_logger::init().unwrap();

        let mut core = Core::new().unwrap();
        let handle = core.handle();

        // Mesos message
        let mut framework_info = mesos::FrameworkInfo::new();
        framework_info.set_user(String::from("nobody"));
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

    #[derive(Clone, Debug)]
    struct State {
        pub framework_id: String,
        pub stream_id: String,
    }

    fn log_response_body(
        chunks: Result<Vec<hyper::Chunk>, hyper::Error>,
    ) -> Result<(), failure::Error> {
        chunks
            .map_err(failure::Error::from)
            .map(|chunks: Vec<hyper::Chunk>| {
                for chunk in chunks {
                    debug!("{}", String::from_utf8_lossy(&chunk));
                }
                ()
            })
    }

    #[test]
    fn task_launch() {
        simple_logger::init().unwrap();

        let mut core = Core::new().unwrap();
        let handle = core.handle();

        // Mesos message
        let user = get_user_by_uid(get_current_uid()).unwrap();
        let mut framework_info = mesos::FrameworkInfo::new();
        framework_info.set_user(String::from(user.name()));
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
                let state = stream::repeat::<_, failure::Error>(State {
                    framework_id: client.framework_id.clone(),
                    stream_id: client.stream_id.clone(),
                });
                client.events.zip(state)
            })
            .flatten()
            .for_each(
                |(mut event, state)| -> Box<Future<Item = _, Error = failure::Error>> {
                    match event.get_field_type() {
                        scheduler::Event_Type::OFFERS => {
                            info!("Received offer.");

                            // Create task for offer.
                            // TODO: Launch task only once!
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
                                Client::accept(state.framework_id, vec![offer_id], vec![operation]);

                            // Make call
                            let uri = "http://localhost:5050/api/v1/scheduler"
                                .parse::<Uri>()
                                .unwrap();
                            let request = Client::request_for(uri, call, Some(state.stream_id));
                            let http_client = hyper::Client::new(&handle);
                            let s = http_client
                                .request(request)
                                .map_err(|error| {
                                    error!("Accept call failed");
                                    failure::Error::from(error)
                                })
                                .and_then(|res| {
                                    debug!("Mesos accept offer response status: {}", res.status());
                                    res.body().collect().then(log_response_body)
                                });
                            Box::new(s)
                        }
                        scheduler::Event_Type::UPDATE => {
                            info!("Received task update.");
                            let status = event.get_update().get_status();
                            let task_id = status.get_task_id();
                            let task_state = status.get_state();
                            debug!(
                                "Task {} is {:?}: {}",
                                task_id.get_value(),
                                task_state,
                                status.get_message()
                            );

                            // TODO: Stop connection.
                            let call = Client::teardown(state.framework_id);

                            // Make call
                            let uri = "http://localhost:5050/api/v1/scheduler"
                                .parse::<Uri>()
                                .unwrap();
                            let request = Client::request_for(uri, call, Some(state.stream_id));
                            let http_client = hyper::Client::new(&handle);
                            let s = http_client
                                .request(request)
                                .map_err(|error| {
                                    error!("Teardown call failed");
                                    failure::Error::from(error)
                                })
                                .and_then(|res| {
                                    debug!("Mesos teardown response status: {}", res.status());
                                    res.body().collect().then(log_response_body)
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
