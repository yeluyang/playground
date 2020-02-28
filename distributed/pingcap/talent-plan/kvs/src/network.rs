use std::{
    io::{BufRead, BufReader, BufWriter, Read, Write},
    net::{TcpListener, TcpStream},
    path::Path,
};

extern crate log;

extern crate serde;
use serde::{Deserialize, Serialize};

pub extern crate serde_json as protocol_serde;

use crate::errors::{Error, Result};
use crate::KvStore;

/// Protocol
#[derive(Debug, Deserialize, Serialize)]
pub enum Protocol {
    GetRequest(String),
    GetResponse(Option<String>),

    SetRequest { key: String, value: String },
    SetResponse(()),

    RemoveRequest(String),
    RemoveResponse(()),

    Error(String),
}

impl Protocol {
    pub fn to_bytes(&self) -> Vec<u8> {
        protocol_serde::to_vec(self)
            .unwrap_or_else(|err| panic!("failed to serialize Protocol to bytes: {}", err))
    }
}

impl<'a> From<&'a str> for Protocol {
    fn from(s: &'a str) -> Self {
        protocol_serde::from_str(s)
            .unwrap_or_else(|err| panic!("failed to get Protocol from str={}: {}", s, err))
    }
}

impl<'a> From<&'a [u8]> for Protocol {
    fn from(b: &'a [u8]) -> Self {
        protocol_serde::from_slice(b)
            .unwrap_or_else(|err| panic!("failed to get Protocol from bytes={:?}: {}", b, err))
    }
}

pub struct Client {
    net_reader: BufReader<TcpStream>,
    net_writer: BufWriter<TcpStream>,
}

impl Client {
    pub fn connect(addr: &str) -> Result<Self> {
        debug!("connecting ip:port={}", addr);
        let stream = TcpStream::connect(addr)?;
        Ok(Self {
            net_reader: BufReader::new(stream.try_clone()?),
            net_writer: BufWriter::new(stream),
        })
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        debug!("getting value of key={} from server", key);

        trace!("writting GetRequest into socket");
        protocol_serde::to_writer(&mut self.net_writer, &Protocol::GetRequest(key))?;
        self.net_writer.write_all(b"\n")?;
        self.net_writer.flush()?;

        let mut buf: Vec<u8> = Vec::new();
        self.net_reader.read_to_end(&mut buf)?;
        trace!("read from socket: GetResponse={:?}", buf);

        let p = Protocol::from(buf.as_slice());
        trace!("get GetResponse from server: response={:?}", p);
        match p {
            Protocol::GetResponse(opt) => Ok(opt),
            Protocol::Error(err) => Err(Error::Simple(err)),
            _ => unreachable!(),
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        debug!("setting data:={{\"{}\": \"{}\"}}", key, value);

        trace!("writting SetRequest into socket");
        protocol_serde::to_writer(&mut self.net_writer, &Protocol::SetRequest { key, value })?;
        self.net_writer.write_all(b"\n")?;
        self.net_writer.flush()?;

        let mut buf: Vec<u8> = Vec::new();
        self.net_reader.read_to_end(&mut buf)?;
        trace!("read from socket: SetResponse={:?}", buf);

        let p = Protocol::from(buf.as_slice());
        trace!("get SetResponse from server: response={:?}", p);
        match p {
            Protocol::SetResponse(_) => Ok(()),
            Protocol::Error(err) => Err(Error::Simple(err)),
            _ => unreachable!(),
        }
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        debug!("removing data of key={}", key);

        trace!("writting RemoveRequest into socket");
        protocol_serde::to_writer(&mut self.net_writer, &Protocol::RemoveRequest(key))?;
        self.net_writer.write_all(b"\n")?;
        self.net_writer.flush()?;

        let mut buf: Vec<u8> = Vec::new();
        self.net_reader.read_to_end(&mut buf)?;
        trace!("read from socket: RemoveResponse={:?}", buf);

        let p = Protocol::from(buf.as_slice());
        trace!("get RemoveResponse from server: response={:?}", p);
        match p {
            Protocol::RemoveResponse(_) => Ok(()),
            Protocol::Error(err) => Err(Error::Simple(err)),
            _ => unreachable!(),
        }
    }
}

pub struct Server {
    kv_store: KvStore,
    listener: TcpListener,
}

impl Server {
    pub fn on<P: AsRef<Path>>(local_dir: P, addr: String) -> Result<Self> {
        debug!("server listening address={}", addr);
        Ok(Self {
            kv_store: KvStore::open(local_dir)?,
            listener: TcpListener::bind(addr)?,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        debug!("serving");
        for stream in self.listener.incoming() {
            trace!("new request income");
            let stream = stream?;
            let mut net_reader = BufReader::new(stream.try_clone()?);
            let mut net_writer = BufWriter::new(stream);

            let mut line = String::new();
            net_reader.read_line(&mut line)?;
            trace!("new request read from socket: request={}", line);

            let p = Protocol::from(line.as_str());
            trace!("request from client: request={:?}", p);
            match p {
                Protocol::GetRequest(key) => {
                    match self.kv_store.get(key) {
                        Err(err) => protocol_serde::to_writer(
                            &mut net_writer,
                            &Protocol::Error(err.to_string()),
                        )
                        .unwrap_or_else(|ser_err| {
                            error!("failed to serialize ErrorResponse={}: {}", err, ser_err)
                        }),
                        Ok(ref opt) => protocol_serde::to_writer(
                            &mut net_writer,
                            &Protocol::GetResponse(opt.clone()),
                        )
                        .unwrap_or_else(|err| {
                            error!("failed to serialize GetResponse={:?}: {}", opt, err)
                        }),
                    };
                }
                Protocol::SetRequest { key, value } => {
                    match self.kv_store.set(key, value) {
                        Err(err) => protocol_serde::to_writer(
                            &mut net_writer,
                            &Protocol::Error(err.to_string()),
                        )
                        .unwrap_or_else(|ser_err| {
                            error!("failed to serialize ErrorResponse={}: {}", err, ser_err)
                        }),
                        Ok(_) => {
                            protocol_serde::to_writer(&mut net_writer, &Protocol::SetResponse(()))
                                .unwrap_or_else(|err| {
                                    error!("failed to serialize SetResponse: {}", err)
                                })
                        }
                    };
                }
                Protocol::RemoveRequest(key) => {
                    match self.kv_store.remove(key) {
                        Err(err) => protocol_serde::to_writer(
                            &mut net_writer,
                            &Protocol::Error(err.to_string()),
                        )
                        .unwrap_or_else(|ser_err| {
                            error!("failed to serialize ErrorResponse={}: {}", err, ser_err)
                        }),
                        Ok(_) => protocol_serde::to_writer(
                            &mut net_writer,
                            &Protocol::RemoveResponse(()),
                        )
                        .unwrap_or_else(|err| {
                            error!("failed to serialize RemoveResponse: {}", err)
                        }),
                    };
                }
                Protocol::Error(err) => error!("receive error from client: {}", err),
                _ => unreachable!(),
            };
        }
        Ok(())
    }
}
