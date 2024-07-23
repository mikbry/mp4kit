use std::io::{Read, Seek};

use crate::{BoxHeader, BoxParser, BoxReader, BoxType, Error, Parser, Reader};

// https://developer.apple.com/documentation/quicktime-file-format/time-to-sample_atom/time-to-sample_table
#[derive(Clone, Debug)]
pub struct TimeToSampleBox {
    pub header: BoxHeader,

    pub version: u8,
    pub flags: u32,

    pub table: Vec<(u32, u32)>, // Sample count | Sample duration
}

impl Reader for TimeToSampleBox {
    fn read<'a, T: Read + Seek>(reader: &mut BoxReader<T>, header: BoxHeader) -> Result<Self, Error> {
        let (version, flags) = reader.read_header_extra()?;

        let entry_count = reader.read_u32()?;
        let mut table = Vec::with_capacity(entry_count as usize);
        for _i in 0..entry_count {
            table.push((reader.read_u32()?, reader.read_u32()?));
        }
        Ok(Self {
            header,
            version,
            flags,
            table,
        })
    }
}

impl Parser for TimeToSampleBox {
    fn parse<'a, T: Read + Seek>(parser: &mut BoxParser<T>) -> Result<Self, Error> {
        let header = parser.next_header_with_type(BoxType::TimeToSample)?.clone();
        TimeToSampleBox::read(parser.get_reader(), header)
    }
}