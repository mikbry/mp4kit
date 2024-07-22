use std::io::{Read, Seek};

use crate::{BoxContainer, BoxHeader, BoxParser, BoxReader, BoxType, ChildBox, Error, MvhdBox, TrackBox};

// https://developer.apple.com/documentation/quicktime-file-format/movie_atom
#[derive(Clone, Debug)]
pub struct MoovBox {
    pub header: BoxHeader,
    pub mvhd: MvhdBox,
}

impl MoovBox {
    pub fn read<'a, T: Read + Seek>(parser: &mut BoxParser<T>, header: BoxHeader) -> Result<Self, Error> {
        let content = BoxContainer::read(parser, header)?;
        let mut mvhd: Option<MvhdBox> = None;
        let mut tracks: Vec<TrackBox> = Vec::new();
        for child in content.children {
            match child {
                ChildBox::Mvhd(b) => mvhd = Some(b),
                ChildBox::Trak(b) => tracks.push(b),
                _ => (),
            }
        }

        if mvhd.is_none() {
            return Err(Error::BoxNotFound("Moov: Mvhd box is mandatory".to_owned()));
        }
        if tracks.len() == 0 {
            return Err(Error::BoxNotFound("Moov: No track found".to_owned()));
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
