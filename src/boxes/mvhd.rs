use std::io::Read;

use crate::{BoxHeader, BoxParser, BoxReader, BoxType, Error};

#[derive(Clone, Debug)]
pub struct MvhdBox {
    pub header: BoxHeader,
}

impl BoxReader for MvhdBox {
    fn parse<'a, T: Read>(mut parser: &mut BoxParser<T>) -> Result<Self, Error> {
        let header = parser.next_header_with_type(BoxType::MovieHeader)?.clone();
        Ok(Self {
            header
        })
    }
}