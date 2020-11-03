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
    pub length: u128,
    pub size: u128,
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
            length,
            size,
            entry_seq,
            frame_seq,
            total,
        }
    }
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();

        bytes.write_u128::<Endian>(self.length)?;
        bytes.write_u128::<Endian>(self.size)?;
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
        h.length = r.read_u128::<Endian>()?;
        h.size = r.read_u128::<Endian>()?;
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
    pub payload: Vec<u8>,
}

impl Frame {
    fn new(header: Header, mut payload: Vec<u8>) -> Self {
        assert_eq!(payload.len(), header.length as usize);
        assert!(payload.len() <= header.size as usize);

        if payload.len() < header.size as usize {
            payload.resize_with(header.size as usize, Default::default);
        };
        assert!(payload.len() == header.size as usize);

        Self { header, payload }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = self.header.to_bytes()?;
        bytes.extend(self.payload.clone());
        assert_eq!(bytes.len(), HEADER_SIZE + self.payload.len());

        Ok(bytes)
    }

    // TODO: add `take_payload`
    pub fn payload(&self) -> &[u8] {
        &self.payload[..self.header.length as usize]
    }

    pub fn is_first(&self) -> bool {
        self.header.frame_seq == 0
    }
}

impl TryFrom<&[u8]> for Frame {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> result::Result<Self, Self::Error> {
        assert!(bytes.len() >= HEADER_SIZE);

        let header = Header::try_from(&bytes[..HEADER_SIZE])?;
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

pub fn create(mut payload: Vec<u8>, entry_seq: u128, partial_limits: usize) -> Vec<Frame> {
    let n = if payload.len() % partial_limits != 0 {
        payload.len() / partial_limits as usize + 1
    } else {
        payload.len() / partial_limits as usize
    };

    debug!(
        "creating {} frames: data.len={}, partial.size={}",
        n,
        payload.len(),
        partial_limits
    );

    let mut frames: Vec<Frame> = Vec::with_capacity(n);

    for i in 0..n {
        let next = if payload.len() >= partial_limits {
            payload.split_off(partial_limits)
        } else {
            vec![]
        };
        assert!(!payload.is_empty());
        assert!(payload.len() <= partial_limits);

        frames.push(Frame::new(
            Header::new(
                payload.len() as u128,
                partial_limits as u128,
                entry_seq,
                i as u128,
                n as u128,
            ),
            payload,
        ));

        payload = next;
    }
    assert_eq!(frames.len(), n);

    frames
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        struct Case {
            payload: Vec<u8>,
            partial_limits: usize,
            expected_segments: usize,
        }
        let cases = [
            Case {
                payload: vec![1; 1024],
                partial_limits: 128,
                expected_segments: 8,
            },
            Case {
                payload: vec![2; 0],
                partial_limits: 128,
                expected_segments: 0,
            },
            Case {
                payload: vec![3; 1025],
                partial_limits: 128,
                expected_segments: 9,
            },
            Case {
                payload: vec![3; 125],
                partial_limits: 128,
                expected_segments: 1,
            },
        ];

        for c in &cases {
            let remain_bytes = c.payload.len() % c.partial_limits;
            let segments = create(c.payload.clone(), c.partial_limits);
            assert_eq!(segments.len(), c.expected_segments);

            if c.expected_segments != 0 {
                assert!(segments[0].is_first());

                let (last_seg, segments) = segments.split_last().unwrap();
                assert_eq!(last_seg.header.size as usize, c.partial_limits);
                assert_eq!(last_seg.payload.len(), c.partial_limits);
                assert!(last_seg.is_last());
                assert_eq!(last_seg.header.frame_seq as usize, segments.len());

                if remain_bytes != 0 {
                    assert_eq!(last_seg.header.length as usize, remain_bytes);
                    assert_eq!(
                        &last_seg.payload[..remain_bytes],
                        &c.payload[c.payload.len() - remain_bytes..]
                    );
                } else {
                    assert_eq!(last_seg.header.length as usize, c.partial_limits);
                    assert_eq!(
                        last_seg.payload.as_slice(),
                        &c.payload[c.payload.len() - last_seg.payload.len()..]
                    );
                }

                let mut last = 0usize;
                for (i, seg) in segments.iter().enumerate() {
                    assert_eq!(seg.header.length as usize, c.partial_limits);
                    assert_eq!(seg.header.size as usize, c.partial_limits);
                    assert_eq!(seg.payload.len(), c.partial_limits);
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
                length: 128,
                size: 128,
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
                        length: 0,
                        size: 128,
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
                        length: 128,
                        size: 128,
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
                        length: 64,
                        size: 128,
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
                &c.segment.payload[..c.segment.header.length as usize]
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
