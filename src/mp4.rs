use std::io::{Read, Seek};

use crate::{BoxHeader, BoxParser, BoxReader, BoxType, Error, FtypBox, MoovBox};

#[derive(Clone, Debug)]
pub struct Mp4 {
    pub ftyp: FtypBox,
    pub moov: MoovBox,
}

enum ChildBox {
    Ftyp(FtypBox),
    Moov(MoovBox),
    Unknown(BoxHeader),
    None(),
}

impl Mp4 {
    fn read_children_box<'a, T: Read + Seek>(parser: &mut BoxParser<'a, T>) -> Result<ChildBox, Error> {
        let header = match BoxHeader::parse(parser) {
            Ok(header) => header,
            Err(error) => {
                if error == Error::EOF() {
                    return Ok(ChildBox::None()); 
                }
                return Err(error);
            },
        };
        let result = match header.r#type {
            BoxType::FileType => {
                let ftyp_box = FtypBox::read(parser, header)?;
                println!("Mp4: {:?}", ftyp_box);
                ChildBox::Ftyp(ftyp_box)
                // 
            },
            BoxType::Movie => {
                let moov_box = MoovBox::read(parser, header)?;
                println!("Mp4: {:?}", moov_box);
                ChildBox::Moov(moov_box)
            },
            _ => {
                // return Err(Error::InvalidBox(format!("Mp4: Invalid child box {:?}", header))); // header.r#type.to_string()));
                println!("Mp4: unknown {:?}", header);
                ChildBox::Unknown(header)
            },
        };
        Ok(result)
    }

    pub fn parse<'a, T: Read + Seek>(mut parser: BoxParser<T>) -> Result<Self, Error> {
        let mut ftyp: Option<FtypBox> = None;
        let mut moov: Option<MoovBox> = None;
        loop  {
            let mp4box = match Mp4::read_children_box(&mut parser) {
                Ok(mp4box) => mp4box,
                Err(error) => return Err(error),
            };
            match mp4box {
                ChildBox::Ftyp(b) => ftyp = Some(b),
                ChildBox::Moov(b) => moov = Some(b),
                ChildBox::Unknown(unknown_box) => {
                    // println!("Mp4: unknown box: {unknown_box:?}");
                    unknown_box.skip_content(&mut parser)?;
                },
                ChildBox::None() => {
                    break;
                },
            }
        };

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