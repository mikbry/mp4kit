use std::{
    char::{decode_utf16, REPLACEMENT_CHARACTER},
    io::{Read, Seek},
};

use crate::{BoxHeader, BoxReader, Error, Reader};

// https://developer.apple.com/documentation/quicktime-file-format/media_header_atom
#[derive(Clone, Debug)]
pub struct MediaHeaderBox {
    pub version: u8,
    pub flags: u32,

    pub creation_time: u64,
    pub modification_time: u64,
    pub timescale: u32,
    pub duration: u64,
    pub language_code: u16,
    pub language: String,
    pub quality: u16,
}

impl Reader for MediaHeaderBox {
    fn read<'a, T: Read + Seek>(
        reader: &mut BoxReader<T>,
        _header: BoxHeader,
    ) -> Result<Self, Error> {
        let (version, flags) = reader.read_header_extra()?;

        let (creation_time, modification_time, timescale, duration) = match version {
            // 32 bit creation, modification times and duration
            0 => (
                reader.read_u32()? as u64,
                reader.read_u32()? as u64,
                reader.read_u32()?,
                reader.read_u32()? as u64,
            ),
            // 64 bit creation, modification times and duration
            1 => (
                reader.read_u64()?,
                reader.read_u64()?,
                reader.read_u32()?,
                reader.read_u64()?,
            ),
            _ => {
                return Err(Error::InvalidData(format!(
                    "Tkhd: unknown version {:?}",
                    version
                )))
            }
        };
        let language_code = reader.read_u16()?;
        let language = language_string(language_code);
        let quality = reader.read_u16()?;
        Ok(Self {
            version,
            flags,

            creation_time,
            modification_time,
            timescale,
            duration,
            language_code,
            language,
            quality,
        })
    }
}

fn language_string(language_code: u16) -> String {
    let language: [u16; 3] = [
        ((language_code >> 10) & 0x1F) + 0x60,
        ((language_code >> 5) & 0x1F) + 0x60,
        ((language_code) & 0x1F) + 0x60,
    ];

    // Decode utf-16 encoded bytes into a string.
    let lang_str = decode_utf16(language.iter().cloned())
        .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
        .collect::<String>();

    lang_str
}
