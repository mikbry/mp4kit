use std::io::{Read, Seek};

use crate::{BoxHeader, BoxParser, BoxReader, BoxType, Error, Parser, Reader};

// https://developer.apple.com/documentation/quicktime-file-format/composition_offset_atom
#[derive(Clone, Debug)]
pub struct CompositionOffsetBox {
    pub header: BoxHeader,

    pub version: u8,
    pub flags: u32,

    pub table: Vec<(u32, i32)>, // sample_count | sample_offset
}

impl Reader for CompositionOffsetBox {
    fn read<'a, T: Read + Seek>(reader: &mut BoxReader<T>, header: BoxHeader) -> Result<Self, Error> {
        let (version, flags) = reader.read_header_extra()?;

        let entry_count = reader.read_u32()?;
        let mut table = Vec::with_capacity(entry_count as usize);
        for _i in 0..entry_count {
            table.push((reader.read_u32()?, reader.read_i32()?));
        }
        Ok(Self {
            header,
            version,
            flags,
            table,
        })
    }
}

impl Parser for CompositionOffsetBox {
    fn parse<'a, T: Read + Seek>(parser: &mut BoxParser<T>) -> Result<Self, Error> {
        let header = parser.next_header_with_type(BoxType::CompositionOffset)?.clone();
        CompositionOffsetBox::read(parser.get_reader(), header)
    }
}