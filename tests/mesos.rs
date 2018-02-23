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
        simple_logger::init();

        let mut core = Core::new().expect("Could not create Core.");
        let handle = core.handle();

        // Mesos message
        let user = get_user_by_uid(get_current_uid()).expect("No system user found.");
        let mut framework_info = mesos::FrameworkInfo::new();
        framework_info.set_user(String::from(user.name()));
        framework_info.set_name(String::from("Example FOO Framework"));

        // Create client
        let uri = "http://localhost:5050/api/v1/scheduler"
            .parse::<Uri>()
            .expect("Could not parse Uri.");
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
        simple_logger::init();

        #[derive(Clone, Debug)]
        pub struct State {
            pub framework_id: String,
            pub stream_id: String,
            pub task_id: Option<mesos::TaskID>,
        }

        let mut core = Core::new().expect("Could not create Core.");
        let handle = core.handle();

        // Mesos message
        let user = get_user_by_uid(get_current_uid()).expect("No system user found.");
        let mut framework_info = mesos::FrameworkInfo::new();
        framework_info.set_user(String::from(user.name()));
        framework_info.set_name(String::from("Example Rust Framework"));
        framework_info.set_role(String::from("*"));

        // Create client
        let uri = "http://localhost:5050/api/v1/scheduler"
            .parse::<Uri>()
            .expect("Could not parse Uri.");
        let future_client = Client::connect(&handle, uri, framework_info);

        // Process events and start and stop a simple task.
        let work = future_client.and_then(|client| {
            let state = State {
                framework_id: client.framework_id.clone(),
                stream_id: client.stream_id.clone(),
                task_id: None,
            };
            client.events.fold(
                state,
                |mut state, mut event| -> Box<Future<Item = State, Error = failure::Error>> {
                    match event.get_field_type() {
                        scheduler::Event_Type::OFFERS => {
                            info!("Received offer.");

                            // We already launched a task.
                            if state.task_id.is_some() {
                                info!("Ignoring offer because task is already launching.");
                                return Box::new(future::result(Ok(state)));
                            }

                            // Create task for offer.
                            let mut offer = event.take_offers().take_offers()[0].clone();
                            let offer_id = offer.take_id();
                            let agent_id = offer.take_agent_id();

                            let mut task_id = mesos::TaskID::new();
                            task_id.set_value(String::from("my_task"));

                            let resource_cpu = Client::resource_cpu(0.1);
                            let resource_mem = Client::resource_mem(32.0);
                            let resources = vec![resource_cpu, resource_mem];

                            let command = Client::command_shell(String::from("sleep 100000"));
                            let task_info = Client::task_info(
                                String::from("sleep_task"),
                                task_id.clone(),
                                agent_id,
                                resources,
                                command,
                            );
                            let operation = Client::launch_operation(vec![task_info]);
                            let call = Client::accept(
                                state.framework_id.clone(),
                                vec![offer_id],
                                vec![operation],
                            );
                            state.task_id = Some(task_id);

                            // Make call
                            let uri = "http://localhost:5050/api/v1/scheduler"
                                .parse::<Uri>()
                                .unwrap();
                            let request =
                                Client::request_for(uri, call, Some(state.stream_id.clone()));
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
                                })
                                .map(|()| state);
                            Box::new(s)
                        }
                        scheduler::Event_Type::UPDATE => {
                            info!("Received task update.");
                            let status = event.take_update().take_status();
                            let task_id = status.get_task_id().clone();
                            let task_state = status.get_state().clone();
                            debug!(
                                "Task {} is {:?}: {}",
                                task_id.get_value(),
                                task_state,
                                status.get_message()
                            );

                            let ack_call = Client::acknowledge(state.framework_id.clone(), status);
                            let ack_uri = "http://localhost:5050/api/v1/scheduler"
                                .parse::<Uri>()
                                .unwrap();
                            let ack_request =
                                    Client::request_for(ack_uri, ack_call, Some(state.stream_id.clone()));
                            let ack_http_client = hyper::Client::new(&handle);
                            // Fire and forget acknowledge call.
                            handle.spawn_fn(move ||{
                                let s = ack_http_client
                                    .request(ack_request)
                                    .map_err(|error| {
                                        error!("Teardown call failed");
                                        failure::Error::from(error)
                                    })
                                    .and_then(|res| {
                                        debug!("Mesos teardown response status: {}", res.status());
                                        res.body().collect().then(log_response_body)
                                    });
                                s.map_err(|error|{
                                    error!("Could not make acknowledge request: {}", error);
                                    ()
                                })
                            });

                            if task_state == mesos::TaskState::TASK_RUNNING {
                                // Stop framework.
                                let call = Client::teardown(state.framework_id.clone());

                                // Make call
                                let uri = "http://localhost:5050/api/v1/scheduler"
                                    .parse::<Uri>()
                                    .unwrap();
                                let request =
                                    Client::request_for(uri, call, Some(state.stream_id.clone()));
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
                                    })
                                    .map(|()| state);

                                return Box::new(s);
                            } else {
                                Box::new(future::result(Ok(state)))
                            }
                        }
                        other => {
                            debug!("Ignore event {:?}", other);
                            Box::new(future::result(Ok(state)))
                        }
                    }
                },
            )
        });

        core.run(work).unwrap();
    }
}
