use std::io::{Read, Seek};

use crate::{BoxContent, BoxElement, BoxHeader, BoxReader, Error, ListBox };

#[derive(Clone, Debug)]
pub struct Mp4 {
    pub ftyp: BoxElement,
    pub moov: BoxElement,
    pub mdat: BoxElement,
}

impl Mp4 {
    pub fn parse<'a, T: Read + Seek>(src: &'a mut T) -> Result<Self, Error> {
        let mut ftyp: Option<BoxElement> = None;
        let mut moov: Option<BoxElement> = None;
        let mut mdat: Option<BoxElement> = None;
        let header = BoxHeader::root("Mp4 ");
        let mut iter = ListBox::iter(header);
        let mut is_wide = false;
        while let Some(child) = iter.next(&mut BoxReader::new(src))? {
            println!("{:?}: {:?}", header.name, child);
            match child.content {
                BoxContent::Ftyp(_) => ftyp = Some(child),
                BoxContent::Moov(_) => moov = Some(child),
                BoxContent::Wide(_) => {
                    is_wide = true;
                }
                BoxContent::Mdat(_) => mdat = Some(child),
                _ => (),
            }
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
