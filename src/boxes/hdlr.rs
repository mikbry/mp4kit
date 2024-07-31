use std::io::{Read, Seek};

use crate::{BoxHeader, BoxReader, Error, Reader, HEADER_LENGTH};

// https://developer.apple.com/documentation/quicktime-file-format/handler_reference_atom
#[derive(Clone, Debug)]
pub struct HandlerBox {
    pub version: u8,
    pub flags: u32,

    pub component_type: String, // FourCC
    pub handler: String, // FourCC
    pub name: String,
}

impl Reader for HandlerBox {
    fn read<'a, T: Read + Seek>(reader: &mut BoxReader<T>, header: BoxHeader) -> Result<Self, Error> {
        let (version, flags) = reader.read_header_extra()?;

        let component_type = reader.read_string(4)?;
        let handler = reader.read_string(4)?;
        reader.skip(12)?; // Reserved
        let len = header.size - HEADER_LENGTH - 4 - 4 - 4 - 12; 
        let name = reader.read_string(len as usize)?;
        Ok(Self {
            version,
            flags,
            component_type,
            handler,
            name,
        })
    }
}
