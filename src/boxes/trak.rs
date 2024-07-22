use std::io::{Read, Seek};

use crate::{BoxContainer, BoxHeader, BoxParser, BoxReader, BoxType, Error};

// https://developer.apple.com/documentation/quicktime-file-format/movie_header_atom
#[derive(Clone, Debug)]
pub struct TrackBox {
    pub header: BoxHeader,
    pub content: BoxContainer,

}

impl TrackBox {
    pub fn read<'a, T: Read + Seek>(parser: &mut BoxParser<T>, header: BoxHeader) -> Result<Self, Error> {
        let content = BoxContainer::read(parser, header)?;
        Ok(Self {
            header,
            content,
        })
    }
}

impl BoxReader for TrackBox {
    fn parse<'a, T: Read + Seek>(parser: &mut BoxParser<T>) -> Result<Self, Error> {
        let header = parser.next_header_with_type(BoxType::Track)?.clone();
        TrackBox::read(parser, header)
    }
}