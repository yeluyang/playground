use std::{
    convert::{TryFrom, TryInto},
    io::Cursor,
    mem, result,
};

extern crate byteorder;
use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::{
    error::{Error, Result},
    Endian,
};

#[derive(Default, Debug, PartialEq)]
pub struct Header {
    pub payload_len: u128,
    pub payload_size: u128,
    pub entry_seq: u128,
    pub frame_seq: u128,
    pub total: u128,
}
pub const HEADER_SIZE: usize = mem::size_of::<Header>();

impl Header {
    fn new(length: u128, size: u128, entry_seq: u128, frame_seq: u128, total: u128) -> Self {
        assert!(length <= size);
        assert!(total > 0);
        assert!(frame_seq < total);
        Self {
            payload_len: length,
            payload_size: size,
            entry_seq,
            frame_seq,
            total,
        }
    }
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();

        bytes.write_u128::<Endian>(self.payload_len)?;
        bytes.write_u128::<Endian>(self.payload_size)?;
        bytes.write_u128::<Endian>(self.entry_seq)?;
        bytes.write_u128::<Endian>(self.frame_seq)?;
        bytes.write_u128::<Endian>(self.total)?;

        assert_eq!(bytes.len(), HEADER_SIZE);

        Ok(bytes)
    }
}

impl TryFrom<&[u8]> for Header {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> result::Result<Self, Self::Error> {
        assert_eq!(bytes.len(), HEADER_SIZE);

        let mut h = Self::default();

        let mut r = Cursor::new(Vec::from(bytes));
        h.payload_len = r.read_u128::<Endian>()?;
        h.payload_size = r.read_u128::<Endian>()?;
        h.entry_seq = r.read_u128::<Endian>()?;
        h.frame_seq = r.read_u128::<Endian>()?;
        h.total = r.read_u128::<Endian>()?;

        Ok(h)
    }
}

impl TryFrom<Vec<u8>> for Header {
    type Error = Error;

    fn try_from(bytes: Vec<u8>) -> result::Result<Self, Self::Error> {
        Self::try_from(bytes.as_slice())
    }
}

impl TryInto<Vec<u8>> for Header {
    type Error = Error;

    fn try_into(self) -> result::Result<Vec<u8>, Self::Error> {
        self.to_bytes()
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct Frame {
    pub header: Header,
    payload: Vec<u8>,
}

impl Frame {
    fn new(header: Header, mut payload: Vec<u8>) -> Self {
        assert_eq!(payload.len(), header.payload_len as usize);

        if payload.len() < header.payload_size as usize {
            payload.resize_with(header.payload_size as usize, Default::default);
        };
        assert!(payload.len() == header.payload_size as usize);

        Self { header, payload }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = self.header.to_bytes()?;
        bytes.extend(self.payload.clone());
        assert_eq!(bytes.len(), HEADER_SIZE + self.payload.len());

        Ok(bytes)
    }

    pub fn take_payload(mut self) -> Vec<u8> {
        self.payload.truncate(self.header.payload_len as usize);
        self.payload
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload[..self.header.payload_len as usize]
    }

    pub fn is_first(&self) -> bool {
        self.header.frame_seq == 0
    }
}

impl TryFrom<&[u8]> for Frame {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> result::Result<Self, Self::Error> {
        assert!(bytes.len() > HEADER_SIZE);

        let header = Header::try_from(&bytes[..HEADER_SIZE])?;

        assert!(bytes.len() == HEADER_SIZE + header.payload_size as usize);
        let payload = Vec::from(&bytes[HEADER_SIZE..]);

        Ok(Self { header, payload })
    }
}

impl TryFrom<Vec<u8>> for Frame {
    type Error = Error;

    fn try_from(bytes: Vec<u8>) -> result::Result<Self, Self::Error> {
        // XXX: should move `bytes[HEADER_SIZE..]` into `frame.payload` for performance
        Self::try_from(bytes.as_slice())
    }
}

impl TryInto<Vec<u8>> for Frame {
    type Error = Error;

    fn try_into(self) -> result::Result<Vec<u8>, Self::Error> {
        self.to_bytes()
    }
}

pub fn create(mut payload: Vec<u8>, entry_seq: u128, partial_size: usize) -> Vec<Frame> {
    assert!(partial_size > 0);
    let total = if payload.len() % partial_size != 0 {
        payload.len() / partial_size as usize + 1
    } else {
        payload.len() / partial_size as usize
    };

    trace!(
        "creating {} frames: data.len={}, partial.size={}",
        total,
        payload.len(),
        partial_size
    );

    let mut frames: Vec<Frame> = Vec::with_capacity(total);

    for frame_seq in 0..total {
        let next = payload.split_off(partial_size);

        frames.push(Frame::new(
            Header::new(
                payload.len() as u128,
                partial_size as u128,
                entry_seq,
                frame_seq as u128,
                total as u128,
            ),
            payload,
        ));

        payload = next;
    }

    frames
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        struct Case {
            payload: Vec<u8>,
            partial_size: usize,
            entry_seq: u128,
            expected_segments: usize,
        }
        let cases = [
            Case {
                payload: vec![1; 1024],
                partial_size: 128,
                entry_seq: 0,
                expected_segments: 8,
            },
            Case {
                payload: vec![2; 0],
                partial_size: 128,
                entry_seq: 1,
                expected_segments: 0,
            },
            Case {
                payload: vec![3; 1025],
                partial_size: 128,
                entry_seq: 3,
                expected_segments: 9,
            },
            Case {
                payload: vec![4; 125],
                partial_size: 128,
                entry_seq: 4,
                expected_segments: 1,
            },
        ];

        for c in &cases {
            let remain_bytes = c.payload.len() % c.partial_size;
            let frames = create(c.payload.clone(), c.entry_seq, c.partial_size);
            assert_eq!(frames.len(), c.expected_segments);

            if c.expected_segments != 0 {
                assert!(frames[0].is_first());

                let frame_last = frames.last().unwrap();
                assert_eq!(frame_last.header.payload_size as usize, c.partial_size);
                assert_eq!(frame_last.payload.len(), c.partial_size);
                assert_eq!(frame_last.header.frame_seq, frame_last.header.total);
                assert_eq!(frame_last.header.frame_seq as usize, frames.len() - 1);

                if remain_bytes != 0 {
                    assert_eq!(frame_last.header.payload_len as usize, remain_bytes);
                    assert_eq!(
                        &frame_last.payload[..remain_bytes],
                        &c.payload[c.payload.len() - remain_bytes..]
                    );
                } else {
                    assert_eq!(frame_last.header.payload_len as usize, c.partial_size);
                    assert_eq!(
                        frame_last.payload.as_slice(),
                        &c.payload[c.payload.len() - frame_last.payload.len()..]
                    );
                }

                let mut last = 0usize;
                for (i, seg) in frames.iter().enumerate() {
                    assert_eq!(seg.header.payload_len as usize, c.partial_size);
                    assert_eq!(seg.header.payload_size as usize, c.partial_size);
                    assert_eq!(seg.payload.len(), c.partial_size);
                    assert_eq!(seg.header.frame_seq as usize, i);
                    assert_eq!(
                        seg.payload.as_slice(),
                        &c.payload[last..last + seg.payload.len()]
                    );
                    last += seg.payload.len();
                }
            }
        }
    }

    #[test]
    fn test_header() {
        struct Case {
            header: Header,
        }
        let cases = [Case {
            header: Header {
                payload_len: 128,
                payload_size: 128,
                entry_seq: 0, // TODO
                frame_seq: 8,
                total: 16,
            },
        }];

        for c in cases.iter() {
            let bytes = c.header.to_bytes().unwrap();
            assert_eq!(bytes.len(), HEADER_SIZE);
            let header = Header::try_from(bytes.as_slice()).unwrap();
            assert_eq!(header, c.header);
            assert_eq!(header.to_bytes().unwrap(), bytes);
        }
    }

    #[test]
    fn test_segment() {
        struct Case {
            segment: Frame,
        }
        let cases = [
            Case {
                segment: Frame {
                    header: Header {
                        payload_len: 0,
                        payload_size: 128,
                        entry_seq: 0, // TODO
                        frame_seq: 4,
                        total: 16,
                    },
                    payload: vec![0; 0],
                },
            },
            Case {
                segment: Frame {
                    header: Header {
                        payload_len: 128,
                        payload_size: 128,
                        entry_seq: 0, // TODO
                        frame_seq: 8,
                        total: 16,
                    },
                    payload: vec![0; 128],
                },
            },
            Case {
                segment: Frame {
                    header: Header {
                        payload_len: 64,
                        payload_size: 128,
                        entry_seq: 0, // TODO
                        frame_seq: 15,
                        total: 16,
                    },
                    payload: vec![0; 64],
                },
            },
        ];

        for c in cases.iter() {
            assert_eq!(
                c.segment.payload(),
                &c.segment.payload[..c.segment.header.payload_len as usize]
            );

            let bytes = c.segment.to_bytes().unwrap();
            assert_eq!(bytes.len(), HEADER_SIZE + c.segment.payload.len());
            let segment = Frame::try_from(bytes.as_slice()).unwrap();
            assert_eq!(segment, c.segment);
            assert_eq!(segment.to_bytes().unwrap(), bytes);
            assert_eq!(segment.payload(), c.segment.payload());
        }
    }
}
