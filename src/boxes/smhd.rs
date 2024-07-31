use std::io::{Read, Seek};

use crate::{BoxHeader, BoxReader, Error, Reader};

// https://developer.apple.com/documentation/quicktime-file-format/sound_media_information_header_atom
#[derive(Clone, Debug)]
pub struct SoundInfoBox {
    pub version: u8,
    pub flags: u32,

    pub balance: u16,
}

impl Reader for SoundInfoBox {
    fn read<'a, T: Read + Seek>(reader: &mut BoxReader<T>, _header: BoxHeader) -> Result<Self, Error> {
        let (version, flags) = reader.read_header_extra()?;

        let balance = reader.read_u16()?;
        reader.skip(2)?;
    
        Ok(Self {
            version,
            flags,

            balance,
        })
    }
}
