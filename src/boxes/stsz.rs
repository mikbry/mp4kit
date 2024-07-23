use std::io::{Read, Seek};

use crate::{BoxHeader, BoxParser, BoxReader, BoxType, Error, Parser, Reader};

// https://developer.apple.com/documentation/quicktime-file-format/sample_size_atom
#[derive(Clone, Debug)]
pub struct SampleSizeBox {
    pub header: BoxHeader,

    pub version: u8,
    pub flags: u32,

    pub sample_size: u32,
    pub sample_sizes: Vec<u32>,
}

impl Reader for SampleSizeBox {
    fn read<'a, T: Read + Seek>(reader: &mut BoxReader<T>, header: BoxHeader) -> Result<Self, Error> {
        let (version, flags) = reader.read_header_extra()?;

        let sample_size = reader.read_u32()?;
        let sample_count = reader.read_u32()?;
        let mut sample_sizes = Vec::new();
        if sample_size == 0 {
            sample_sizes.reserve(sample_count as usize);
            for _ in 0..sample_count {
                sample_sizes.push(reader.read_u32()?);
            }
        }

        Ok(Self {
            header,
            version,
            flags,

            sample_size,
            sample_sizes,
        })
    }
}

impl Parser for SampleSizeBox {
    fn parse<'a, T: Read + Seek>(parser: &mut BoxParser<T>) -> Result<Self, Error> {
        let header = parser.next_header_with_type(BoxType::SampleSize)?.clone();
        SampleSizeBox::read(parser.get_reader(), header)
    }
}