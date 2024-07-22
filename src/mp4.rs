use std::io::{Read, Seek};

use crate::{BoxContainer, BoxHeader, BoxParser, ChildBox, Error, FtypBox, MediaDataBox, MoovBox, Reader};

#[derive(Clone, Debug)]
pub struct Mp4 {
    pub ftyp: FtypBox,
    pub moov: MoovBox,
    pub mdat: MediaDataBox,
}

impl Mp4 {
    pub fn parse<'a, T: Read + Seek>(mut parser: BoxParser<T>) -> Result<Self, Error> {
        let mut ftyp: Option<FtypBox> = None;
        let mut moov: Option<MoovBox> = None;
        let mut mdat: Option<MediaDataBox> = None;
        let header = BoxHeader::root("Mp4 ");
        let content = BoxContainer::read(parser.get_reader(), header)?;
        let mut is_wide = false;
        for child in content.children {
            match child {
                ChildBox::Ftyp(b) => ftyp = Some(b),
                ChildBox::Moov(b) => moov = Some(b),
                ChildBox::Wide(_) => {
                    is_wide = true;
                }
                ChildBox::Mdat(b) => mdat = Some(b),
                _ => (),
            }
            // is_wide = false;
        }
        println!("Mp4: TODO handle wide {:}", is_wide);
    
        if ftyp.is_none() {
            return Err(Error::BoxNotFound("Mp4: Ftyp box is mandatory".to_owned()));
        }
        if moov.is_none() {
            return Err(Error::BoxNotFound("Mp4: Moov box is mandatory".to_owned()));
        }
        if mdat.is_none() {
            return Err(Error::BoxNotFound("Mp4: Mdat box is mandatory".to_owned()));
        }

        Ok(Self {
            ftyp: ftyp.unwrap(),
            moov: moov.unwrap(),
            mdat: mdat.unwrap(),
        })
    }
}
