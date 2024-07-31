use std::io::{Read, Seek};

use crate::{ListBox, BoxHeader, BoxReader, BoxContent, Error, MvhdBox, Reader, TrackBox};

// https://developer.apple.com/documentation/quicktime-file-format/movie_atom
#[derive(Clone, Debug)]
pub struct MoovBox {
    pub mvhd: MvhdBox,
}

impl Reader for MoovBox {
    fn read<'a, T: Read + Seek>(reader: &mut BoxReader<T>, header: BoxHeader) -> Result<Self, Error> {
        let content = ListBox::read(reader, header)?;
        let mut mvhd: Option<MvhdBox> = None;
        let mut tracks: Vec<TrackBox> = Vec::new();
        for child in content.children {
            match child.content {
                BoxContent::Mvhd(b) => mvhd = Some(b),
                BoxContent::Trak(b) => tracks.push(b),
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
            mvhd: mvhd.unwrap(),
        })
    }
}
