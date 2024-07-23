use std::io::{Read, Seek};

use crate::{BoxContainer, BoxHeader, BoxParser, BoxReader, BoxType, Error, Parser, Reader};

// https://developer.apple.com/documentation/quicktime-file-format/base_media_information_atom
#[derive(Clone, Debug)]
pub struct MediaInfoBox {
    pub header: BoxHeader,
    pub content: BoxContainer,

}

impl Reader for MediaInfoBox {
    fn read<'a, T: Read + Seek>(reader: &mut BoxReader<T>, header: BoxHeader) -> Result<Self, Error> {
        let content = BoxContainer::read(reader, header)?;
        Ok(Self {
            header,
            content,
        })
    }
}

impl Parser for MediaInfoBox {
    fn parse<'a, T: Read + Seek>(parser: &mut BoxParser<T>) -> Result<Self, Error> {
        let header = parser.next_header_with_type(BoxType::MediaInfo)?.clone();
        MediaInfoBox::read(parser.get_reader(), header)
    }
}