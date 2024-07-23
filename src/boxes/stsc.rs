use std::io::{Read, Seek};

use crate::{BoxHeader, BoxParser, BoxReader, BoxType, Error, Parser, Reader};

// https://developer.apple.com/documentation/quicktime-file-format/sample-to-chunk_atom
#[derive(Clone, Debug)]
pub struct SampleToChunkBox {
    pub header: BoxHeader,

    pub version: u8,
    pub flags: u32,

    pub table: Vec<(u32, u32, u32)>, // First chunk | Sample per chunk | Sample description ID
}

impl Reader for SampleToChunkBox {
    fn read<'a, T: Read + Seek>(reader: &mut BoxReader<T>, header: BoxHeader) -> Result<Self, Error> {
        let (version, flags) = reader.read_header_extra()?;

        let entry_count = reader.read_u32()?;
        let mut table = Vec::with_capacity(entry_count as usize);
        for _i in 0..entry_count {
            table.push((reader.read_u32()?, reader.read_u32()?, reader.read_u32()?));
        }
        Ok(Self {
            header,
            version,
            flags,
            table,
        })
    }
}

impl Parser for SampleToChunkBox {
    fn parse<'a, T: Read + Seek>(parser: &mut BoxParser<T>) -> Result<Self, Error> {
        let header = parser.next_header_with_type(BoxType::SampleToChunk)?.clone();
        SampleToChunkBox::read(parser.get_reader(), header)
    }
}