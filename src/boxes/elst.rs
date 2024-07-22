use std::io::{Read, Seek};

use crate::{BoxHeader, BoxParser, BoxReader, BoxType, Error, Parser, Reader};

// https://developer.apple.com/documentation/quicktime-file-format/edit_atom/edit_list_atom
#[derive(Clone, Debug)]
pub struct EditListBox {
    pub header: BoxHeader,

    pub version: u8,
    pub flags: u32,

    pub entries: Vec<EditEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EditEntry {
    pub segment_duration: u64,
    pub media_time: i64,
    pub media_rate_integer: u16,
    pub media_rate_fraction: u16,
}

impl Reader for EditListBox {
    fn read<'a, T: Read + Seek>(reader: &mut BoxReader<T>, header: BoxHeader) -> Result<Self, Error> {
        let (version, flags) = reader.read_header_extra()?;
        let entry_count = reader.read_u32()?;
        let mut entries = Vec::with_capacity(entry_count as usize);
        for _ in 0..entry_count {
            let (segment_duration, media_time) = match version {
            // 32 bit creation, modification times and duration
            0 => {
                (
                    reader.read_u32()? as u64,
                    reader.read_i32()? as i64,
                )
            }
            // 64 bit creation, modification times and duration
            1 => {
                (
                    reader.read_u64()?,
                    reader.read_i64()?,
                )
            }
            _ => return Err(Error::InvalidData(format!("Tkhd: unknown version {:?}", version))),
            };
            let entry = EditEntry {
                segment_duration,
                media_time,
                media_rate_integer: reader.read_u16()?,
                media_rate_fraction: reader.read_u16()?,
            };
            entries.push(entry);
        }
    
        Ok(Self {
            header,

            version,
            flags,

            entries,
        })
    }
}

impl Parser for EditListBox {
    fn parse<'a, T: Read + Seek>(parser: &mut BoxParser<T>) -> Result<Self, Error> {
        let header = parser.next_header_with_type(BoxType::Track)?.clone();
        EditListBox::read(parser.get_reader(), header)
    }
}