use std::io::{Read, Seek};

use crate::{BoxHeader, BoxParser, BoxReader, BoxType, Error, Parser, Reader};

// https://developer.apple.com/documentation/quicktime-file-format/sample-to-chunk_atom
#[derive(Clone, Debug)]
pub struct SyncSampleBox {
    pub header: BoxHeader,

    pub version: u8,
    pub flags: u32,

    pub samples: Vec<u32>,
}

impl Reader for SyncSampleBox {
    fn read<'a, T: Read + Seek>(reader: &mut BoxReader<T>, header: BoxHeader) -> Result<Self, Error> {
        let (version, flags) = reader.read_header_extra()?;

        let sample_count = reader.read_u32()?;
        let mut samples = Vec::with_capacity(sample_count as usize);
        for _i in 0..sample_count {
            samples.push(reader.read_u32()?);
        }
        Ok(Self {
            header,
            version,
            flags,
            samples,
        })
    }
}

impl Parser for SyncSampleBox {
    fn parse<'a, T: Read + Seek>(parser: &mut BoxParser<T>) -> Result<Self, Error> {
        let header = parser.next_header_with_type(BoxType::SyncSample)?.clone();
        SyncSampleBox::read(parser.get_reader(), header)
    }
}