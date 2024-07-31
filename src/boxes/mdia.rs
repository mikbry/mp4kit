use std::io::{Read, Seek};

use crate::{ListBox, BoxHeader, BoxReader, BoxContent, Error, HandlerBox, MediaHeaderBox, MediaInfoBox, Reader};

// https://developer.apple.com/documentation/quicktime-file-format/media_atom
#[derive(Clone, Debug)]
pub struct MediaBox {
    pub media_header: MediaHeaderBox,
    pub handler: Option<HandlerBox>,
    pub info: Option<MediaInfoBox>
}

impl Reader for MediaBox {
    fn read<'a, T: Read + Seek>(reader: &mut BoxReader<T>, header: BoxHeader) -> Result<Self, Error> {
        let content = ListBox::read(reader, header)?;
        let mut media_header: Option<MediaHeaderBox> = None;
        let mut handler: Option<HandlerBox> = None;
        let mut info: Option<MediaInfoBox> = None;
        for child in content.children {
            match child.content {
                BoxContent::Mdhd(b) => media_header = Some(b),
                BoxContent::Hdlr(b) => handler = Some(b),
                BoxContent::Minf(b) => info = Some(b),
                _ => {
                    println!("Mdia: TODO not implemented {:?}", child);
                },
            }
        }
    
        if media_header.is_none() {
            return Err(Error::BoxNotFound("Mdia: mdhd box is mandatory".to_owned()));
        }
        Ok(Self {
            media_header: media_header.unwrap(),
            handler,
            info,
        })
    }
}
