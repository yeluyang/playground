mod map_reduce;
pub(crate) use self::map_reduce::{FileLocation, JobGetRequest, JobGetResponse, TaskType};

mod map_reduce_grpc;
pub(crate) use self::map_reduce_grpc::{create_master_grpc, MasterGrpc, MasterGrpcClient};

pub(crate) fn crate_task_type_from(task_type: self::TaskType) -> Option<crate::TaskType> {
    match task_type {
        self::TaskType::ANY => None,
        self::TaskType::MAP => Some(crate::TaskType::Map),
        self::TaskType::REDUCE => Some(crate::TaskType::Reduce),
    }
}

pub(crate) fn grpc_task_type_from(task_type: Option<crate::TaskType>) -> self::TaskType {
    match task_type {
        None => self::TaskType::ANY,
        Some(task_type) => match task_type {
            crate::TaskType::Map => self::TaskType::MAP,
            crate::TaskType::Reduce => self::TaskType::REDUCE,
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task_type_convert() {
        unimplemented!()
    }
}
