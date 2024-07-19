use std::io::{Read, Seek};

use crate::{BoxHeader, BoxParser, BoxReader, BoxType, Error};

#[derive(Clone, Debug)]
pub struct MvhdBox {
    pub header: BoxHeader,
}

impl MvhdBox {
    pub fn read<'a, T: Read + Seek>(parser: &mut BoxParser<T>, header: BoxHeader) -> Result<Self, Error> {
        parser.seek((header.size - 8).try_into().unwrap())?;
        Ok(Self {
            header
        })
    }
}

impl BoxReader for MvhdBox {
    fn parse<'a, T: Read + Seek>(parser: &mut BoxParser<T>) -> Result<Self, Error> {
        let header = parser.next_header_with_type(BoxType::MovieHeader)?.clone();
        parser.seek((header.size - 8).try_into().unwrap())?;
        Ok(Self {
            header
        })
    }
}