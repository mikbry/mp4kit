use std::io::{Read, Seek};

use crate::{BoxHeader, BoxParser, BoxReader, BoxType, Error, Matrix};

// https://developer.apple.com/documentation/quicktime-file-format/movie_header_atom
#[derive(Clone, Debug)]
pub struct MvhdBox {
    pub header: BoxHeader,

    pub version: u8,
    pub flags: u32,

    pub creation_time: u64,
    pub modification_time: u64,
    pub timescale: u32,
    pub duration: u64,

    pub rate: u32,
    pub volume: u16,

    pub matrix: Matrix,

    pub preview_time: u32,
    pub preview_duration: u32,
    pub poster_time: u32,
    pub selection_time: u32,
    pub selection_duration: u32,
    pub current_time: u32,

    pub next_track_id: u32,
}

impl MvhdBox {
    pub fn read<'a, T: Read + Seek>(parser: &mut BoxParser<T>, header: BoxHeader) -> Result<Self, Error> {
        let (version, flags) = parser.read_header_extra()?;
        let (creation_time, modification_time, timescale, duration) = match version {
            // 32 bit creation, modification times and duration
            0 => {
                (
                    parser.read_u32()? as u64,
                    parser.read_u32()? as u64,
                    parser.read_u32()?,
                    parser.read_u32()? as u64,
                )
            }
            // 64 bit creation, modification times and duration
            1 => {
                (
                    parser.read_u64()?,
                    parser.read_u64()?,
                    parser.read_u32()?,
                    parser.read_u64()?,
                )
            }
            _ => return Err(Error::InvalidData(format!("Mvhd: unknown version {:?}", version))),
        };
        let rate = parser.read_u32()?;
        let volume = parser.read_u16()?;
        
        parser.skip(10)?; // Reserved

        let matrix = Matrix::read(parser)?;

        let preview_time = parser.read_u32()?;
        let preview_duration = parser.read_u32()?;
        let poster_time = parser.read_u32()?;
        let selection_time = parser.read_u32()?;
        let selection_duration = parser.read_u32()?;
        let current_time = parser.read_u32()?;

        let next_track_id = parser.read_u32()?;
    
        Ok(Self {
            header,
            version,
            flags,
            creation_time,
            modification_time,
            timescale,
            duration,

            rate,
            volume,

            matrix,

            preview_time,
            preview_duration,
            poster_time,
            selection_time,
            selection_duration,
            current_time,

            next_track_id,
        })
    }
}

impl BoxReader for MvhdBox {
    fn parse<'a, T: Read + Seek>(parser: &mut BoxParser<T>) -> Result<Self, Error> {
        let header = parser.next_header_with_type(BoxType::MovieHeader)?.clone();
        MvhdBox::read(parser, header)
    }
}