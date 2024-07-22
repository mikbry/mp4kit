use std::io::{Read, Seek};

use crate::{BoxContainer, BoxHeader, BoxParser, BoxReader, BoxType, ChildBox, Error, MediaHeaderBox, Parser, Reader};

// https://developer.apple.com/documentation/quicktime-file-format/media_atom
#[derive(Clone, Debug)]
pub struct MediaBox {
    pub header: BoxHeader,
    // pub content: BoxContainer,
    pub media_header: MediaHeaderBox,
    // pub handler: Option<HandlerBox>,
    // pub information: Option<MediaInformationBox>
}

impl Reader for MediaBox {
    fn read<'a, T: Read + Seek>(reader: &mut BoxReader<T>, header: BoxHeader) -> Result<Self, Error> {
        let content = BoxContainer::read(reader, header)?;
        let mut media_header: Option<MediaHeaderBox> = None;
        for child in content.children {
            match child {
                ChildBox::Mdhd(b) => media_header = Some(b),
                _ => {
                    println!("Mdia: TODO not implemented {:?}", child);
                },
            }
        }
    
        if media_header.is_none() {
            return Err(Error::BoxNotFound("Mdia: mdhd box is mandatory".to_owned()));
        }
        Ok(Self {
            header,
            // content,
            media_header: media_header.unwrap(),
        })
    }
}

impl Parser for MediaBox {
    fn parse<'a, T: Read + Seek>(parser: &mut BoxParser<T>) -> Result<Self, Error> {
        let header = parser.next_header_with_type(BoxType::Track)?.clone();
        MediaBox::read(parser.get_reader(), header)
    }
}