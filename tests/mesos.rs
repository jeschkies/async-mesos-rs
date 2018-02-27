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
    use futures::{future, Future, Stream};
    use hyper::Uri;
    use async_mesos::client::Client;
    use async_mesos::mesos;
    use async_mesos::model;
    use async_mesos::scheduler;
    use simple_logger;
    use spectral::prelude::*;
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
            .map(|(_, events)| events)
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
            pub client: Client,
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
        let work = future_client.and_then(|(client, events)| {
            let state = State {
                client: client,
                task_id: None,
            };
            events.fold(
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

                            let cpu = model::ScalarResourceBuilder::default()
                                .name("cpu")
                                .value(0.1)
                                .build()?;

                            let mem = model::ScalarResourceBuilder::default()
                                .name("mem")
                                .value(32.0)
                                .build()?;

                            let command = model::ShellCommandBuilder::default()
                                .command("sleep 100000")
                                .build()?;

                            let task_info = model::TaskInfoBuilder::default()
                                .name("sleep_task")
                                .task_id(task_id.clone())
                                .agent_id(agent_id)
                                .resource(cpu)
                                .resource(mem)
                                .command(command)
                                .build();

                            let operation = model::OfferLaunchOperationBuilder::default()
                                .task_info(task_info)
                                .build()?;
                            let call = state.client.accept(vec![offer_id], vec![operation]);
                            state.task_id = Some(task_id);

                            // Make call
                            let s = state.client.call(&handle, call).map(|()| state);
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

                            let ack_call = state.client.acknowledge(status);
                            // Fire and forget acknowledge call.
                            let s = state.client.call(&handle, ack_call).map_err(|error| {
                                error!("Could not make acknowledge request: {}", error);
                                ()
                            });
                            handle.spawn(s);

                            if task_state == mesos::TaskState::TASK_RUNNING {
                                // Stop framework.
                                let call = state.client.teardown();
                                let s = state.client.call(&handle, call).map(|()| state);
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
