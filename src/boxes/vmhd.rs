use std::io::{Read, Seek};

use crate::{BoxHeader, BoxReader, Error, Reader};

// https://developer.apple.com/documentation/quicktime-file-format/video_media_information_header_atom
#[derive(Clone, Debug)]
pub struct VideoInfoBox {
    pub version: u8,
    pub flags: u32,

    pub graphics_mode: u16,
    pub op_color: (u16, u16, u16),
}

impl Reader for VideoInfoBox {
    fn read<'a, T: Read + Seek>(reader: &mut BoxReader<T>, _header: BoxHeader) -> Result<Self, Error> {
        let (version, flags) = reader.read_header_extra()?;

        let graphics_mode = reader.read_u16()?;
        let op_color = (
            reader.read_u16()?,
            reader.read_u16()?,
            reader.read_u16()?,
        );
    
        Ok(Self {
            version,
            flags,

            graphics_mode,
            op_color,
        })
    }
}
