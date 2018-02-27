/// Model builders for Mesos Protobufs.

use failure;
use mesos;
use protobuf::core::Message;

///// Construct offer launch operation.
/////
///// # Argument
/////
///// * `task_infos` - Description of the tasks to launch.
//pub fn launch_operation(&self, task_infos: Vec<mesos::TaskInfo>) -> mesos::Offer_Operation {
//    let mut operation = mesos::Offer_Operation::new();
//    let mut launch = mesos::Offer_Operation_Launch::new();
//    launch.set_task_infos(RepeatedField::from_vec(task_infos));
//
//    operation.set_launch(launch);
//    operation.set_field_type(mesos::Offer_Operation_Type::LAUNCH);
//    operation
//}
//
///// Construct task info.
/////
///// # Argument
/////
///// * `name` - The name of the launched task.
///// * `task_id` - The id of the launched task.
///// * `agent_id` - The agent where the task is lauched.
///// * `resources` - The resources to use.
///// * `commdn` - The command of the task to execute.
//pub fn task_info(
//    &self,
//    name: String,
//    task_id: mesos::TaskID,
//    agent_id: mesos::AgentID,
//    resources: Vec<mesos::Resource>,
//    command: mesos::CommandInfo,
//) -> mesos::TaskInfo {
//    let mut task_info = mesos::TaskInfo::new();
//    task_info.set_name(name);
//    task_info.set_task_id(task_id);
//    task_info.set_agent_id(agent_id);
//    task_info.set_resources(RepeatedField::from_vec(resources));
//    task_info.set_command(command);
//    task_info
//}
//
///// Construct shell command.
//pub fn command_shell(&self, command: String) -> mesos::CommandInfo {
//    let mut command_info = mesos::CommandInfo::new();
//    command_info.set_shell(true);
//    command_info.set_value(command);
//    command_info
//}
//
//pub fn resource_cpu(&self, cpus: f64) -> mesos::Resource {
//    let mut resource = mesos::Resource::new();
//    resource.set_name(String::from("cpus"));
//    resource.set_field_type(mesos::Value_Type::SCALAR);
//
//    let mut scalar = mesos::Value_Scalar::new();
//    scalar.set_value(cpus);
//    resource.set_scalar(scalar);
//    resource
//}
//
//pub fn resource_mem(&self, mem: f64) -> mesos::Resource {
//    ScalarResourceBuilder::default()
//        .name("mem")
//        .value(mem)
//        .build()
//}

struct ScalarResourceBuilder {
    resource: mesos::Resource,
}

impl ScalarResourceBuilder {

    fn default() -> Self {
        let mut resource = mesos::Resource::new();
        let scalar = mesos::Value_Scalar::new();
        resource.set_field_type(mesos::Value_Type::SCALAR);
        resource.set_scalar(scalar);

        ScalarResourceBuilder{ resource }
    }

    fn name(mut self, name: &str) -> Self {
        self.resource.set_name(String::from(name));
        self
    }

    fn value(mut self, value: f64) -> Self {
        self.resource.mut_scalar().set_value(value);
        self
    }

    fn build(self) -> Result<mesos::Resource, failure::Error> {
        if self.resource.is_initialized() {
            Ok(self.resource)
        } else {
            Err(format_err!("Mesos scala resource was not complete"))
        }
    }
}

#[cfg(test)]
mod tests {

    use model::ScalarResourceBuilder;
    use spectral::prelude::*;

    #[test]
    fn scalar_resource() {
        let resource = ScalarResourceBuilder::default()
        .name("mem")
        .value(42.0)
        .build();

        assert_that(&resource).is_ok();

        let broken_resource = ScalarResourceBuilder::default()
        .name("mem")
        .build();

        assert_that(&broken_resource).is_err();
    }
}
