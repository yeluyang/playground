use std::default::Default;

extern crate protobuf;
use protobuf::{RepeatedField, SingularPtrField};

mod map_reduce;
pub(crate) use self::map_reduce::{
    FileLocation, JobGetRequest, JobGetResponse, JobGetResponse_oneof_job,
};
use self::map_reduce::{MapJob, ReduceJob};

mod map_reduce_grpc;
pub(crate) use self::map_reduce_grpc::{create_master_grpc, MasterGrpc, MasterGrpcClient};

pub(crate) fn crate_job_from(job: JobGetResponse_oneof_job) -> crate::Job {
    match job {
        JobGetResponse_oneof_job::map_job(mut map_job) => crate::Job::Map {
            host: map_job.mut_file().take_host(),
            path: map_job.mut_file().take_path(),
        },
        JobGetResponse_oneof_job::reduce_job(mut reduce_job) => {
            let mut paths: Vec<(String, String)> = Vec::new();
            for file in reduce_job.mut_files().iter_mut() {
                paths.push((file.take_host(), file.take_path()));
            }

            crate::Job::Reduce {
                key: reduce_job.take_key(),
                paths,
            }
        }
    }
}

pub(crate) fn grpc_job_from(job: crate::Job) -> JobGetResponse_oneof_job {
    match job {
        crate::Job::Map { host, path } => JobGetResponse_oneof_job::map_job(MapJob {
            file: SingularPtrField::from_option(Some(FileLocation {
                host,
                path,
                unknown_fields: Default::default(),
                cached_size: Default::default(),
            })),
            unknown_fields: Default::default(),
            cached_size: Default::default(),
        }),
        crate::Job::Reduce { key, paths } => {
            let mut files = RepeatedField::new();
            for (host, path) in paths {
                files.push(FileLocation {
                    host,
                    path,
                    unknown_fields: Default::default(),
                    cached_size: Default::default(),
                });
            }

            JobGetResponse_oneof_job::reduce_job(ReduceJob {
                key,
                files,
                unknown_fields: Default::default(),
                cached_size: Default::default(),
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task_type_convert() {
        struct TestCase {
            crate_job: crate::Job,
            self_job: self::JobGetResponse_oneof_job,
        }
        let cases = &[
            TestCase {
                crate_job: crate::Job::Map {
                    host: "127.0.0.1".to_owned(),
                    path: "/path/to/map/file".to_owned(),
                },
                self_job: self::JobGetResponse_oneof_job::map_job(MapJob {
                    file: SingularPtrField::from_option(Some(FileLocation {
                        host: "127.0.0.1".to_owned(),
                        path: "/path/to/map/file".to_owned(),
                        unknown_fields: Default::default(),
                        cached_size: Default::default(),
                    })),
                    unknown_fields: Default::default(),
                    cached_size: Default::default(),
                }),
            },
            TestCase {
                crate_job: crate::Job::Reduce {
                    key: "1".to_owned(),
                    paths: vec![("127.0.0.1".to_owned(), "/path/to/reduce/file".to_owned())],
                },
                self_job: self::JobGetResponse_oneof_job::reduce_job(ReduceJob {
                    key: "1".to_owned(),
                    files: RepeatedField::from_vec(vec![FileLocation {
                        host: "127.0.0.1".to_owned(),
                        path: "/path/to/reduce/file".to_owned(),
                        unknown_fields: Default::default(),
                        cached_size: Default::default(),
                    }]),
                    unknown_fields: Default::default(),
                    cached_size: Default::default(),
                }),
            },
        ];
        for c in cases {
            assert_eq!(crate_job_from(c.self_job.clone()), c.crate_job);
            assert_eq!(grpc_job_from(c.crate_job.clone()), c.self_job);
        }
    }
}
