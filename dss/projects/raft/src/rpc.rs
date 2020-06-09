use std::sync::mpsc::Sender;

use crate::{
    peer::{LogSeq, Vote},
    EndPoint,
};

pub trait PeerClientRPC: Clone {
    fn connect(host: &EndPoint) -> Self;
    fn heart_beat(&self);

    fn request_vote_async(
        &self,
        host: EndPoint,
        term: usize,
        log_seq: Option<LogSeq>,
        ch: Sender<Vote>,
    );
}
