use std::io::{Read, Seek};

use crate::{BoxContainer, BoxHeader, BoxParser, ChildBox, Error, FtypBox, MoovBox};

#[derive(Clone, Debug)]
pub struct Mp4 {
    pub ftyp: FtypBox,
    pub moov: MoovBox,
}



impl Mp4 {
    pub fn parse<'a, T: Read + Seek>(mut parser: BoxParser<T>) -> Result<Self, Error> {
        let mut ftyp: Option<FtypBox> = None;
        let mut moov: Option<MoovBox> = None;
        let header = BoxHeader::root();
        let content = BoxContainer::read(&mut parser, header)?;
        for child in content.children {
            match child {
                ChildBox::Ftyp(b) => ftyp = Some(b),
                ChildBox::Moov(b) => moov = Some(b),
                _ => (),
            }
        }

        if ftyp.is_none() {
            return Err(Error::BoxNotFound("Mp4: Ftyp box is mandatory".to_owned()));
        }
        if moov.is_none() {
            return Err(Error::BoxNotFound("Mp4: Moov box is mandatory".to_owned()));
        }
    
        Ok(Self {
            ftyp: ftyp.unwrap(),
            moov: moov.unwrap(),
        })
    }
}