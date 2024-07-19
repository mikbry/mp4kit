use std::io::Read;

use crate::{BoxHeader, BoxParser, BoxReader, BoxType, Error, FtypBox, MoovBox};

#[derive(Clone, Debug)]
pub struct Mp4 {
    pub ftyp: FtypBox,
    pub moov: MoovBox,
}

enum Mp4Box {
    Ftyp(FtypBox),
    Moov(MoovBox),
    None(),
}
impl Mp4 {

    fn read_box<'a, T: Read>(parser: &mut BoxParser<'a, T>) -> Result<Mp4Box, Error> {
        let header = BoxHeader::parse(parser)?;
        let result = match header.r#type {
            BoxType::FileType => {
                let ftyp_box = FtypBox::read(parser, header)?;
                Mp4Box::Ftyp(ftyp_box)
                // println!("ftyp box: {:?}", ftyp_box.clone());
            },
            BoxType::Movie => {
                let moov_box = MoovBox::read(parser, header)?;
                Mp4Box::Moov(moov_box)
                // println!("moov box: {:?}", moov_box);
            },
            _ => {
                return Err(Error::InvalidBox("Invalid box".to_owned())); // header.r#type.to_string()));
            },
        };
        Ok(result)
    }

    pub fn parse<'a, T: Read>(mut parser: BoxParser<T>) -> Result<Self, Error> {
        let mut ftyp: Option<FtypBox> = None;
        let mut moov: Option<MoovBox> = None;
        loop  {
            let mp4box = match Mp4::read_box(&mut parser) {
                Ok(mp4box) => mp4box,
                Err(error) => return Err(error),
            };
            match mp4box {
                Mp4Box::Ftyp(b) => ftyp = Some(b),
                Mp4Box::Moov(b) => moov = Some(b),
                Mp4Box::None() => {
                    break;
                },
            }
        
            /* match header.r#type {
                BoxType::FileType => {
                    let ftyp_box = FtypBox::read(parser, header)?.clone();
                    ftyp = Some(ftyp_box);
                    // println!("ftyp box: {:?}", ftyp_box.clone());
                },
                BoxType::Movie => {
                    let moov_box = MoovBox::read(parser, header)?;
                    moov = Some(moov_box);
                    // println!("moov box: {:?}", moov_box);
                },
                _ => {
                    return Err(Error::InvalidBox("Invalid box".to_owned())); // header.r#type.to_string()));
                },
            } */
        };

        if ftyp.is_none() {
            return Err(Error::BoxNotFound("Ftyp box is mandatory".to_owned()));
        }
        if moov.is_none() {
            return Err(Error::BoxNotFound("Moov box is mandatory".to_owned()));
        }
    
        Ok(Self {
            ftyp: ftyp.unwrap(),
            moov: moov.unwrap(),
        })
    }
}