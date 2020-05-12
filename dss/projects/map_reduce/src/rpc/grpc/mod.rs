use std::{collections::HashMap, default::Default};

extern crate protobuf;
use protobuf::{RepeatedField, SingularPtrField};

mod map_reduce;
pub(crate) use self::map_reduce::{
    FileLocation, JobDoneRequest, JobDoneRequest_oneof_result, JobDoneResponse, JobGetRequest,
    JobGetResponse, JobGetResponse_oneof_job, MapResult, ReduceResult,
};
use self::map_reduce::{MapJob, ReduceJob};

mod map_reduce_grpc;
pub(crate) use self::map_reduce_grpc::{create_master_grpc, MasterGrpc, MasterGrpcClient};

fn file_location_from(host: String, path: String) -> FileLocation {
    FileLocation {
        host,
        path,
        unknown_fields: Default::default(),
        cached_size: Default::default(),
    }
}

pub(crate) fn crate_job_from(job: JobGetResponse_oneof_job) -> crate::Job {
    match job {
        JobGetResponse_oneof_job::map_job(mut map_job) => crate::Job::Map {
            reducers: map_job.reducers as usize,
            host: map_job.mut_file().take_host(),
            path: map_job.mut_file().take_path(),
        },
        JobGetResponse_oneof_job::reduce_job(mut reduce_job) => {
            let mut paths: Vec<(String, String)> = Vec::new();
            for file in reduce_job.mut_files().iter_mut() {
                paths.push((file.take_host(), file.take_path()));
            }

            crate::Job::Reduce {
                output_dir: reduce_job.take_output_dir(),
                internal_key: reduce_job.take_internal_key(),
                paths,
            }
        }
    }
}

pub(crate) fn grpc_job_from(job: crate::Job) -> JobGetResponse_oneof_job {
    match job {
        crate::Job::Map {
            reducers,
            host,
            path,
        } => JobGetResponse_oneof_job::map_job(MapJob {
            reducers: reducers as i64,
            file: SingularPtrField::from_option(Some(file_location_from(host, path))),
            unknown_fields: Default::default(),
            cached_size: Default::default(),
        }),
        crate::Job::Reduce {
            output_dir,
            internal_key,
            paths,
        } => {
            let mut files = RepeatedField::new();
            for (host, path) in paths {
                files.push(file_location_from(host, path));
            }

            JobGetResponse_oneof_job::reduce_job(ReduceJob {
                output_dir,
                internal_key,
                files,
                unknown_fields: Default::default(),
                cached_size: Default::default(),
            })
        }
    }
}

pub(crate) fn crate_job_result_from(job_result: JobDoneRequest_oneof_result) -> crate::JobResult {
    match job_result {
        JobDoneRequest_oneof_result::map_result(mut map_result) => {
            let mut host: Option<String> = None;
            let mut paths = HashMap::new();
            for (internal_key, mut file_location) in map_result.take_internal_key_results() {
                if let Some(h) = &host {
                    if file_location.get_host() != h {
                        panic!("multi host from one MapResult");
                    }
                } else {
                    host = Some(file_location.take_host());
                }
                paths
                    .insert(internal_key, file_location.take_path())
                    .expect_none("multi internal key from one MapResult");
            }
            crate::JobResult::Map {
                host: host.unwrap(),
                paths,
            }
        }
        JobDoneRequest_oneof_result::reduce_result(mut reduce_result) => crate::JobResult::Reduce {
            internal_key: reduce_result.take_internal_key(),
            host: reduce_result.mut_result().take_host(),
            path: reduce_result.mut_result().take_path(),
        },
    }
}

pub(crate) fn grpc_job_result_from(job_result: crate::JobResult) -> JobDoneRequest_oneof_result {
    match job_result {
        crate::JobResult::Map { host, paths } => {
            let mut result = HashMap::new();
            for (internal_key, path) in paths {
                result
                    .insert(internal_key, file_location_from(host.clone(), path))
                    .expect_none("multi internal_key from one map job result");
            }
            JobDoneRequest_oneof_result::map_result(MapResult {
                internal_key_results: result,
                unknown_fields: Default::default(),
                cached_size: Default::default(),
            })
        }
        crate::JobResult::Reduce {
            internal_key,
            host,
            path,
        } => JobDoneRequest_oneof_result::reduce_result(ReduceResult {
            internal_key,
            result: SingularPtrField::from_option(Some(file_location_from(host, path))),
            unknown_fields: Default::default(),
            cached_size: Default::default(),
        }),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_job_convert() {
        struct TestCase {
            crate_job: crate::Job,
            grpc_job: self::JobGetResponse_oneof_job,
        }
        let cases = &[
            TestCase {
                crate_job: crate::Job::Map {
                    reducers: 4,
                    host: "127.0.0.1".to_owned(),
                    path: "/path/to/map/file".to_owned(),
                },
                grpc_job: self::JobGetResponse_oneof_job::map_job(MapJob {
                    reducers: 4,
                    file: SingularPtrField::from_option(Some(file_location_from(
                        "127.0.0.1".to_owned(),
                        "/path/to/map/file".to_owned(),
                    ))),
                    unknown_fields: Default::default(),
                    cached_size: Default::default(),
                }),
            },
            TestCase {
                crate_job: crate::Job::Reduce {
                    output_dir: "/path/to/reduce/files".to_owned(),
                    internal_key: "1".to_owned(),
                    paths: vec![("127.0.0.1".to_owned(), "/path/to/reduce/file".to_owned())],
                },
                grpc_job: self::JobGetResponse_oneof_job::reduce_job(ReduceJob {
                    output_dir: "/path/to/reduce/files".to_owned(),
                    internal_key: "1".to_owned(),
                    files: RepeatedField::from_vec(vec![file_location_from(
                        "127.0.0.1".to_owned(),
                        "/path/to/reduce/file".to_owned(),
                    )]),
                    unknown_fields: Default::default(),
                    cached_size: Default::default(),
                }),
            },
        ];
        for c in cases {
            assert_eq!(crate_job_from(c.grpc_job.clone()), c.crate_job);
            assert_eq!(grpc_job_from(c.crate_job.clone()), c.grpc_job);
        }
    }

    #[test]
    fn test_job_result_convert() {
        struct TestCase {
            crate_job_result: crate::JobResult,
            grpc_job_result: JobDoneRequest_oneof_result,
        }
        let cases = &[TestCase {
            crate_job_result: crate::JobResult::Map {
                host: "127.0.0.1".to_owned(),
                paths: hashmap! {
                    "0".to_owned() => "test/rpc/grpc/job_result_convert/map/0".to_owned(),
                    "1".to_owned() => "test/rpc/grpc/job_result_convert/map/1".to_owned(),
                },
            },
            grpc_job_result: JobDoneRequest_oneof_result::map_result(MapResult {
                internal_key_results: hashmap! {
                    "0".to_owned() => file_location_from("127.0.0.1".to_owned(),"test/rpc/grpc/job_result_convert/map/0".to_owned()),
                    "1".to_owned() => file_location_from("127.0.0.1".to_owned(),"test/rpc/grpc/job_result_convert/map/1".to_owned()),
                },
                unknown_fields: Default::default(),
                cached_size: Default::default(),
            }),
        }];

        for c in cases {
            assert_eq!(
                crate_job_result_from(c.grpc_job_result.clone()),
                c.crate_job_result
            );
            assert_eq!(
                grpc_job_result_from(c.crate_job_result.clone()),
                c.grpc_job_result
            );
        }
    }
}
