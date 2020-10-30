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
pub(crate) struct SegmentHeader {
    // TODO add `segment_uuid`
    length: u128,
    size: u128,
    pub(crate) entry_seq: u128,
    pub(crate) partial_seq: u128,
    pub(crate) total: u128,
}
pub(crate) const SEGMENT_HEADER_SIZE: usize = mem::size_of::<SegmentHeader>();

impl SegmentHeader {
    fn new(length: u128, size: u128, partial_seq: u128, total: u128) -> Self {
        Self {
            length,
            size,
            entry_seq: 0, // TODO
            partial_seq,
            total,
        }
    }

    fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut wtr = Vec::new();

        wtr.write_u128::<Endian>(self.length)?;
        wtr.write_u128::<Endian>(self.size)?;
        wtr.write_u128::<Endian>(self.entry_seq)?;
        wtr.write_u128::<Endian>(self.partial_seq)?;
        wtr.write_u128::<Endian>(self.total)?;

        assert_eq!(wtr.len(), SEGMENT_HEADER_SIZE);

        Ok(wtr)
    }
}

impl TryFrom<&[u8]> for SegmentHeader {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> result::Result<Self, Self::Error> {
        assert_eq!(bytes.len(), SEGMENT_HEADER_SIZE);

        let mut h = Self::default();

        let mut rdr = Cursor::new(Vec::from(bytes));
        h.length = rdr.read_u128::<Endian>()?;
        h.size = rdr.read_u128::<Endian>()?;
        h.entry_seq = rdr.read_u128::<Endian>()?;
        h.partial_seq = rdr.read_u128::<Endian>()?;
        h.total = rdr.read_u128::<Endian>()?;

        Ok(h)
    }
}

impl TryFrom<Vec<u8>> for SegmentHeader {
    type Error = Error;

    fn try_from(bytes: Vec<u8>) -> result::Result<Self, Self::Error> {
        Self::try_from(bytes.as_slice())
    }
}

impl TryInto<Vec<u8>> for SegmentHeader {
    type Error = Error;

    fn try_into(self) -> result::Result<Vec<u8>, Self::Error> {
        self.to_bytes()
    }
}

#[derive(Default, Debug, PartialEq)]
pub(crate) struct Segment {
    pub(crate) header: SegmentHeader,
    pub(crate) payload: Vec<u8>,
}

impl Segment {
    fn new(header: SegmentHeader, mut payload: Vec<u8>) -> Self {
        assert_eq!(payload.len(), header.length as usize);
        assert!(payload.len() <= header.size as usize);

        if payload.len() < header.size as usize {
            payload.resize_with(header.size as usize, Default::default);
        };
        assert!(payload.len() == header.size as usize);

        Self { header, payload }
    }

    pub(crate) fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = self.header.to_bytes()?;
        bytes.extend(self.payload.clone());
        assert_eq!(bytes.len(), SEGMENT_HEADER_SIZE + self.payload.len());

        Ok(bytes)
    }

    // TODO: add `take_payload`
    pub(crate) fn payload(&self) -> &[u8] {
        &self.payload[..self.header.length as usize]
    }

    pub(crate) fn is_first(&self) -> bool {
        self.header.partial_seq == 0
    }

    pub(crate) fn is_last(&self) -> bool {
        self.header.partial_seq == self.header.total - 1
    }
}

impl TryFrom<&[u8]> for Segment {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> result::Result<Self, Self::Error> {
        assert!(bytes.len() >= SEGMENT_HEADER_SIZE);

        let header = SegmentHeader::try_from(&bytes[..SEGMENT_HEADER_SIZE])?;
        let payload = Vec::from(&bytes[SEGMENT_HEADER_SIZE..]);

        Ok(Self { header, payload })
    }
}

impl TryFrom<Vec<u8>> for Segment {
    type Error = Error;

    fn try_from(bytes: Vec<u8>) -> result::Result<Self, Self::Error> {
        Self::try_from(bytes.as_slice())
    }
}

impl TryInto<Vec<u8>> for Segment {
    type Error = Error;

    fn try_into(self) -> result::Result<Vec<u8>, Self::Error> {
        self.to_bytes()
    }
}

pub(crate) fn create(mut payload: Vec<u8>, partial_limits: usize) -> Vec<Segment> {
    let n = if payload.len() % partial_limits != 0 {
        payload.len() / partial_limits as usize + 1
    } else {
        payload.len() / partial_limits as usize
    };

    debug!(
        "creating {} segments: data.len={}, partial.size={}",
        n,
        payload.len(),
        partial_limits
    );

    let mut segments: Vec<Segment> = Vec::with_capacity(n);

    for i in 0..n {
        let next = if payload.len() >= partial_limits {
            payload.split_off(partial_limits)
        } else {
            vec![]
        };
        assert!(!payload.is_empty());
        assert!(payload.len() <= partial_limits);

        segments.push(Segment::new(
            SegmentHeader::new(
                payload.len() as u128,
                partial_limits as u128,
                i as u128,
                n as u128,
            ),
            payload,
        ));

        payload = next;
    }
    assert_eq!(segments.len(), n);

    segments
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
                assert_eq!(last_seg.header.partial_seq as usize, segments.len());

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
                    assert_eq!(seg.header.partial_seq as usize, i);
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
            header: SegmentHeader,
        }
        let cases = [Case {
            header: SegmentHeader {
                length: 128,
                size: 128,
                entry_seq: 0, // TODO
                partial_seq: 8,
                total: 16,
            },
        }];

        for c in cases.iter() {
            let bytes = c.header.to_bytes().unwrap();
            assert_eq!(bytes.len(), SEGMENT_HEADER_SIZE);
            let header = SegmentHeader::try_from(bytes.as_slice()).unwrap();
            assert_eq!(header, c.header);
            assert_eq!(header.to_bytes().unwrap(), bytes);
        }
    }

    #[test]
    fn test_segment() {
        struct Case {
            segment: Segment,
        }
        let cases = [
            Case {
                segment: Segment {
                    header: SegmentHeader {
                        length: 0,
                        size: 128,
                        entry_seq: 0, // TODO
                        partial_seq: 4,
                        total: 16,
                    },
                    payload: vec![0; 0],
                },
            },
            Case {
                segment: Segment {
                    header: SegmentHeader {
                        length: 128,
                        size: 128,
                        entry_seq: 0, // TODO
                        partial_seq: 8,
                        total: 16,
                    },
                    payload: vec![0; 128],
                },
            },
            Case {
                segment: Segment {
                    header: SegmentHeader {
                        length: 64,
                        size: 128,
                        entry_seq: 0, // TODO
                        partial_seq: 15,
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
            assert_eq!(bytes.len(), SEGMENT_HEADER_SIZE + c.segment.payload.len());
            let segment = Segment::try_from(bytes.as_slice()).unwrap();
            assert_eq!(segment, c.segment);
            assert_eq!(segment.to_bytes().unwrap(), bytes);
            assert_eq!(segment.payload(), c.segment.payload());
        }
    }
}
