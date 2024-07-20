use std::io::{Read, Seek};

use crate::{BoxHeader, BoxParser, BoxReader, BoxType, Error, MvhdBox};

#[derive(Clone, Debug)]
pub struct MoovBox {
    pub header: BoxHeader,

    pub mvhd: MvhdBox,
}

enum ChildBox {
    Mvhd(MvhdBox),
    Unknown(BoxHeader),
    None(),
}

impl MoovBox {
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
            BoxType::MovieHeader => {
                let mvhd_box = MvhdBox::read(parser, header)?;
                ChildBox::Mvhd(mvhd_box)
            },
            _ => {
                ChildBox::Unknown(header)
            },
        };
        Ok(result)
    }

    pub fn read<'a, T: Read + Seek>(parser: &mut BoxParser<T>, header: BoxHeader) -> Result<Self, Error> {
        parser.clean();
        let mut mvhd: Option<MvhdBox> = None;
        let mut content_parsed_size: u64  = 0;
        loop  {
            // println!("content_size {content_parsed_size:} {:?}", header.size - 8);
            if content_parsed_size >= header.size - 8 {
                break;
            }
            let mp4box = match MoovBox::read_children_box(parser) {
                Ok(mp4box) => mp4box,
                Err(error) => return Err(error),
            };
            match mp4box {
                ChildBox::Mvhd(b) => {
                    content_parsed_size += b.header.size;
                    println!("Moov: {b:?}");
                    mvhd = Some(b);
                },
                ChildBox::None() => {
                    break;
                },
                ChildBox::Unknown(unknown_box) => {
                    content_parsed_size += unknown_box.size;
                    println!("Moov: unknown {unknown_box:?}");
                    unknown_box.skip_content(parser)?;
                },
            }
        };

        if mvhd.is_none() {
            return Err(Error::BoxNotFound("Moov: Mvhd box is mandatory".to_owned()));
        }
        Ok(Self {
            header,
            mvhd: mvhd.unwrap(),
        })
    }
}
impl BoxReader  for MoovBox {
    fn parse<'a, T: Read + Seek>(parser: &mut BoxParser<T>) -> Result<Self, Error> {
        let header = parser.next_header_with_type(BoxType::Movie)?;
        MoovBox::read(parser, header)
    }
}
