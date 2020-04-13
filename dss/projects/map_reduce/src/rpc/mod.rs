use crate::{master::Master, FileLocation, Task};

extern crate grpcio;
use grpcio::{RpcContext, UnarySink};

mod grpc;
use grpc::{MasterGrpc, MasterGrpcClient, TaskGetRequest, TaskGetResponse, TaskType};

pub(crate) struct MasterServer {
    master: Master,
}

impl MasterGrpc for MasterServer {
    fn task_get(&mut self, ctx: RpcContext, req: TaskGetRequest, sink: UnarySink<TaskGetResponse>) {
        let task_type = match req.get_task_type() {
            TaskType::ANY => crate::TaskType::Any,
            TaskType::MAP => crate::TaskType::Map,
            TaskType::REDUCE => crate::TaskType::Reduce,
        };
        let task = self.master.alloc_task(task_type, req.get_host().to_owned());
        sink.success(TaskGetResponse::new());
    }
}

pub(crate) struct MasterClient {
    client: MasterGrpcClient,
}

impl MasterClient {
    pub(crate) fn new() -> Self {
        unimplemented!()
    }

    pub fn get_task(&self, host: String) -> Option<Task> {
        let mut req = TaskGetRequest::new();
        req.host = host;

        let rsp = self.client.task_get(&req).unwrap();

        let host = rsp.get_file_location().get_host().to_owned();
        let path = rsp.get_file_location().get_path().to_owned();

        match rsp.get_task_type() {
            TaskType::MAP => Some(Task::new(crate::TaskType::Map, host, path)),
            TaskType::REDUCE => Some(Task::new(crate::TaskType::Reduce, host, path)),
            TaskType::ANY => unreachable!(),
        }
    }
}
