use std::io::{Read, Seek};

use crate::{BoxHeader, BoxParser, BoxReader, BoxType, Error, Matrix, Parser, Reader};

// https://developer.apple.com/documentation/quicktime-file-format/track_header_atom
#[derive(Clone, Debug)]
pub struct TrackHeaderBox {
    pub header: BoxHeader,

    pub version: u8,
    pub flags: u32,

    pub creation_time: u64,
    pub modification_time: u64,
    pub track_id: u32,
    pub duration: u64,
    pub layer: u16,
    pub alternate_group: u16,
    pub volume: u16,
    pub matrix: Matrix,
    pub width: u32,
    pub height: u32,
}

impl Reader for TrackHeaderBox {
    fn read<'a, T: Read + Seek>(reader: &mut BoxReader<T>, header: BoxHeader) -> Result<Self, Error> {
        let (version, flags) = reader.read_header_extra()?;
        let (creation_time, modification_time, track_id, _, duration) = match version {
            // 32 bit creation, modification times and duration
            0 => {
                (
                    reader.read_u32()? as u64,
                    reader.read_u32()? as u64,
                    reader.read_u32()?,
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
                    reader.read_u32()?,
                    reader.read_u64()?,
                )
            }
            _ => return Err(Error::InvalidData(format!("Tkhd: unknown version {:?}", version))),
        };
        reader.skip(8)?; // Reserved

        let layer = reader.read_u16()?;
        let alternate_group = reader.read_u16()?;
        let volume = reader.read_u16()?;
    
        reader.skip(2)?; // Reserved

        let matrix = Matrix::read(reader, header)?;

        let width = reader.read_u32()?;
        let height = reader.read_u32()?;

        Ok(Self {
            header,
            version,
            flags,
            creation_time,
            modification_time,
            track_id,
            duration,
            layer,
            alternate_group,
            volume,
            matrix,
            width,
            height,
        })
    }
}

impl Parser for TrackHeaderBox {
    fn parse<'a, T: Read + Seek>(parser: &mut BoxParser<T>) -> Result<Self, Error> {
        let header = parser.next_header_with_type(BoxType::Track)?.clone();
        TrackHeaderBox::read(parser.get_reader(), header)
    }
}