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
    use async_mesos::client::{Client, ClientSession, Events};
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

    #[test]
    fn task_launch() {
        simple_logger::init();

        #[derive(Debug)]
        pub struct State {
            pub framework_id: String,
            pub stream_id: String,
            pub task_id: Option<mesos::TaskID>,
            pub session: ClientSession,
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
                framework_id: client.session.framework_id.clone(),
                stream_id: client.session.stream_id.clone(),
                task_id: None,
                session: client.session.clone(),
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
                            let s = Client::call(&handle, &state.session, call).map(|()| state);
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
                            // Fire and forget acknowledge call.
                            let s =
                                Client::call(&handle, &state.session, ack_call).map_err(|error| {
                                    error!("Could not make acknowledge request: {}", error);
                                    ()
                                });
                            handle.spawn(s);

                            if task_state == mesos::TaskState::TASK_RUNNING {
                                // Stop framework.
                                let call = Client::teardown(state.framework_id.clone());
                                let s = Client::call(&handle, &state.session, call).map(|()| state);
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
