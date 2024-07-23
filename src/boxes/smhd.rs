use std::io::{Read, Seek};

use crate::{BoxHeader, BoxParser, BoxReader, BoxType, Error, Parser, Reader};

// https://developer.apple.com/documentation/quicktime-file-format/sound_media_information_header_atom
#[derive(Clone, Debug)]
pub struct SoundInfoBox {
    pub header: BoxHeader,

    pub version: u8,
    pub flags: u32,

    pub balance: u16,
}

impl Reader for SoundInfoBox {
    fn read<'a, T: Read + Seek>(reader: &mut BoxReader<T>, header: BoxHeader) -> Result<Self, Error> {
        let (version, flags) = reader.read_header_extra()?;

        let balance = reader.read_u16()?;
        reader.skip(2)?;
    
        Ok(Self {
            header,

            version,
            flags,

            balance,
        })
    }
}

impl Parser for SoundInfoBox {
    fn parse<'a, T: Read + Seek>(parser: &mut BoxParser<T>) -> Result<Self, Error> {
        let header = parser.next_header_with_type(BoxType::Track)?.clone();
        SoundInfoBox::read(parser.get_reader(), header)
    }
}