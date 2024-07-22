use std::io::{Read, Seek};

use crate::{BoxHeader, BoxParser, BoxReader, BoxType, Error};

// https://developer.apple.com/documentation/quicktime-file-format/wide_atom
#[derive(Clone, Debug)]
pub struct WideBox {
    pub header: BoxHeader,

}

impl WideBox {
    pub fn read<'a, T: Read + Seek>(parser: &mut BoxParser<T>, header: BoxHeader) -> Result<Self, Error> {
        header.skip_content(parser, 0)?;
        Ok(Self {
            header,
        })
    }
}

impl BoxReader for WideBox {
    fn parse<'a, T: Read + Seek>(parser: &mut BoxParser<T>) -> Result<Self, Error> {
        let header = parser.next_header_with_type(BoxType::Track)?.clone();
        WideBox::read(parser, header)
    }
}