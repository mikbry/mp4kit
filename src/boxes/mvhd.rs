use std::io::{Read, Seek};

use crate::{BoxHeader, BoxReader, Error, Matrix, Reader};

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

impl Reader for MvhdBox {
    fn read<'a, T: Read + Seek>(reader: &mut BoxReader<T>, header: BoxHeader) -> Result<Self, Error> {
        let (version, flags) = reader.read_header_extra()?;
        let (creation_time, modification_time, timescale, duration) = match version {
            // 32 bit creation, modification times and duration
            0 => {
                (
                    reader.read_u32()? as u64,
                    reader.read_u32()? as u64,
                    reader.read_u32()?,
                    reader.read_u32()? as u64,
                )
            }
            // 64 bit creation, modification times and duration
            1 => {
                (
                    reader.read_u64()?,
                    reader.read_u64()?,
                    reader.read_u32()?,
                    reader.read_u64()?,
                )
            }
            _ => return Err(Error::InvalidData(format!("Mvhd: unknown version {:?}", version))),
        };
        let rate = reader.read_u32()?;
        let volume = reader.read_u16()?;
        
        reader.skip(10)?; // Reserved

        let matrix = Matrix::read(reader, header)?;

        let preview_time = reader.read_u32()?;
        let preview_duration = reader.read_u32()?;
        let poster_time = reader.read_u32()?;
        let selection_time = reader.read_u32()?;
        let selection_duration = reader.read_u32()?;
        let current_time = reader.read_u32()?;

        let next_track_id = reader.read_u32()?;
    
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
