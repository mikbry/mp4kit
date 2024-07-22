use std::io::{Read, Seek};

use crate::{BoxContainer, BoxHeader, BoxParser, BoxReader, BoxType, Error, Parser, Reader};

// https://developer.apple.com/documentation/quicktime-file-format/media_atom
#[derive(Clone, Debug)]
pub struct MediaBox {
    pub header: BoxHeader,
    pub content: BoxContainer,
    // pub media_header: MediaHeaderBox,
    // pub handler: Option<HandlerBox>,
    // pub information: Option<MediaInformationBox>
}

impl Reader for MediaBox {
    fn read<'a, T: Read + Seek>(reader: &mut BoxReader<T>, header: BoxHeader) -> Result<Self, Error> {
        let content = BoxContainer::read(reader, header)?;
        Ok(Self {
            header,
            content,
        })
    }
}

impl Parser for MediaBox {
    fn parse<'a, T: Read + Seek>(parser: &mut BoxParser<T>) -> Result<Self, Error> {
        let header = parser.next_header_with_type(BoxType::Track)?.clone();
        MediaBox::read(parser.get_reader(), header)
    }
}