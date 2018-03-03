//! Model builders for Mesos Protobufs.
//!
//! These stateful builders ease the creation of protobuf models. The `build` method runs a
//! minimal verification of the model and resturns `Result<mesos::Model, failure::Error>`.
//!
//! # Example
//!
//! This example is taken from `tests/mesos.rs` in the main repository.
//!
//! ```rust
//! # extern crate async_mesos;
//! # extern crate failure;
//! # extern crate hyper;
//! #
//! # use async_mesos::scheduler;
//! #
//! # fn build_accept_call() -> Result<scheduler::Call, failure::Error> {
//! #
//! # use async_mesos::client::Client;
//! # use async_mesos::mesos;
//! # use hyper::Uri;
//! use async_mesos::model;
//! #
//! # let uri = "http://localhost:5050/api/v1/scheduler"
//! #    .parse::<Uri>()
//! #    .unwrap();
//! # let framework_id = String::from("my_framework");
//! # let stream_id = String::from("my_stream");
//! # let client = Client { uri, framework_id, stream_id };
//! # let mut task_id = mesos::TaskID::new();
//! # task_id.set_value(String::from("my_task"));
//! # let mut agent_id =  mesos::AgentID::new();
//! # agent_id.set_value(String::from("my_agent"));
//! # let mut offer_id =  mesos::OfferID::new();
//! # offer_id.set_value(String::from("some_offer"));
//!
//! let cpu = model::ScalarResourceBuilder::default()
//!     .name("cpus")
//!     .value(0.1)
//!     .build()?;
//!
//! let mem = model::ScalarResourceBuilder::default()
//!     .name("mem")
//!     .value(32.0)
//!     .build()?;
//!
//! let command = model::ShellCommandBuilder::default()
//!     .command("sleep 100000")
//!     .build()?;
//!
//! let task_info = model::TaskInfoBuilder::default()
//!     .name("sleep_task")
//!     .task_id(task_id)
//!     .agent_id(agent_id)
//!     .resources(vec![cpu, mem])
//!     .command(command)
//!     .build()?;
//!
//! let operation = model::OfferLaunchOperationBuilder::default()
//!     .task_info(task_info)
//!     .build()?;
//! let call = client.accept(vec![offer_id], vec![operation]);
//! Ok(call)
//! # }
//! # fn main() {
//! # build_accept_call().unwrap();
//! # }
//! ```

use failure;
use mesos;
use protobuf::core::Message;
use protobuf::repeated::RepeatedField;

/// Builder for Mesos offer operation.
///
/// # Example
///
/// ```no_run
/// # use async_mesos::model;
/// # let task_info = model::TaskInfoBuilder::default().build().unwrap();
/// #
/// model::OfferLaunchOperationBuilder::default()
///     .task_info(task_info)
///     .build();
/// ```
pub struct OfferLaunchOperationBuilder {
    operation: mesos::Offer_Operation,
}

impl OfferLaunchOperationBuilder {
    pub fn default() -> Self {
        let mut operation = mesos::Offer_Operation::new();
        let mut launch = mesos::Offer_Operation_Launch::new();
        launch.set_task_infos(RepeatedField::new());
        operation.set_launch(launch);
        operation.set_field_type(mesos::Offer_Operation_Type::LAUNCH);

        OfferLaunchOperationBuilder { operation }
    }

    pub fn task_info(mut self, task_info: mesos::TaskInfo) -> Self {
        self.operation.mut_launch().mut_task_infos().push(task_info);
        self
    }

    pub fn build(self) -> Result<mesos::Offer_Operation, failure::Error> {
        if self.operation.is_initialized() {
            Ok(self.operation)
        } else {
            Err(format_err!("Mesos offer operation was not complete"))
        }
    }
}

pub struct TaskInfoBuilder {
    task_info: mesos::TaskInfo,
}

impl TaskInfoBuilder {
    pub fn default() -> Self {
        let task_info = mesos::TaskInfo::new();

        TaskInfoBuilder { task_info }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.task_info.set_name(String::from(name));
        self
    }

    pub fn task_id(mut self, task_id: mesos::TaskID) -> Self {
        self.task_info.set_task_id(task_id);
        self
    }

    pub fn agent_id(mut self, agent_id: mesos::AgentID) -> Self {
        self.task_info.set_agent_id(agent_id);
        self
    }

    pub fn command(mut self, command: mesos::CommandInfo) -> Self {
        self.task_info.set_command(command);
        self
    }

    pub fn resources(mut self, resources: Vec<mesos::Resource>) -> Self {
        self.task_info.set_resources(RepeatedField::from_vec(resources));
        self
    }

    pub fn build(self) -> Result<mesos::TaskInfo, failure::Error> {
        if self.task_info.is_initialized() {
            Ok(self.task_info)
        } else {
            Err(format_err!("Mesos task info was not complete"))
        }
    }
}

pub struct ShellCommandBuilder {
    command_info: mesos::CommandInfo,
}

impl ShellCommandBuilder {
    pub fn default() -> Self {
        let mut command_info = mesos::CommandInfo::new();
        command_info.set_shell(true);
        ShellCommandBuilder { command_info }
    }

    pub fn command(mut self, command: &str) -> Self {
        self.command_info.set_value(String::from(command));
        self
    }

    pub fn build(self) -> Result<mesos::CommandInfo, failure::Error> {
        if self.command_info.is_initialized() {
            Ok(self.command_info)
        } else {
            Err(format_err!("Mesos command info was not complete"))
        }
    }
}

pub struct ScalarResourceBuilder {
    resource: mesos::Resource,
}

impl ScalarResourceBuilder {
    pub fn default() -> Self {
        let mut resource = mesos::Resource::new();
        let scalar = mesos::Value_Scalar::new();
        resource.set_field_type(mesos::Value_Type::SCALAR);
        resource.set_scalar(scalar);

        ScalarResourceBuilder { resource }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.resource.set_name(String::from(name));
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.resource.mut_scalar().set_value(value);
        self
    }

    pub fn build(self) -> Result<mesos::Resource, failure::Error> {
        if self.resource.is_initialized() {
            Ok(self.resource)
        } else {
            Err(format_err!("Mesos scala resource was not complete"))
        }
    }
}

#[cfg(test)]
mod tests {

    use mesos::{AgentID, TaskID};
    use model::{OfferLaunchOperationBuilder, ScalarResourceBuilder, ShellCommandBuilder,
                TaskInfoBuilder};
    use protobuf::core::Message;
    use spectral::prelude::*;

    #[test]
    fn scalar_resource() {
        let resource = ScalarResourceBuilder::default()
            .name("mem")
            .value(42.0)
            .build();

        assert_that(&resource).is_ok();

        let broken_resource = ScalarResourceBuilder::default().name("mem").build();

        assert_that(&broken_resource).is_err();
    }

    #[test]
    fn command_info() {
        let command = ShellCommandBuilder::default()
            .command("sleep 100000")
            .build();

        assert_that(&command).is_ok();

        let broken_command = ShellCommandBuilder::default().build();

        assert_that(&broken_command).is_ok();
    }

    #[test]
    fn task_info() {
        let mut agent_id = AgentID::new();
        agent_id.set_value(String::from("an_agent"));
        asserting(&"Agent id is initialized")
            .that(&agent_id.is_initialized())
            .is_true();

        let mut task_id = TaskID::new();
        task_id.set_value(String::from("my_task"));
        asserting(&"Task id is initialized")
            .that(&task_id.is_initialized())
            .is_true();

        let resource_cpu = ScalarResourceBuilder::default()
            .name("cpu")
            .value(0.1)
            .build()
            .expect("CPU resource was not complete.");

        let resource_mem = ScalarResourceBuilder::default()
            .name("mem")
            .value(32.0)
            .build()
            .expect("Memory resource was not complete.");

        let task_info = TaskInfoBuilder::default()
            .name("my_task")
            .task_id(task_id)
            .agent_id(agent_id)
            .resources(vec![resource_cpu, resource_mem])
            .build();

        assert_that(&task_info).is_ok();

        let broken_task_info = TaskInfoBuilder::default().build();

        assert_that(&broken_task_info).is_err();
    }

    #[test]
    fn offer_operation() {
        let broken_operation = OfferLaunchOperationBuilder::default().build();

        assert_that(&broken_operation).is_ok();
    }
}
