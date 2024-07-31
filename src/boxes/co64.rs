use std::io::{Read, Seek};

use crate::{BoxHeader, BoxReader, Error, Reader};

// https://developer.apple.com/documentation/quicktime-file-format/sample-to-chunk_atom
#[derive(Clone, Debug)]
pub struct ChunkOffset64Box {
    pub version: u8,
    pub flags: u32,

    pub table: Vec<u64>,
}

impl Reader for ChunkOffset64Box {
    fn read<'a, T: Read + Seek>(reader: &mut BoxReader<T>, _header: BoxHeader) -> Result<Self, Error> {
        let (version, flags) = reader.read_header_extra()?;

        let entry_count = reader.read_u32()?;
        let mut table = Vec::with_capacity(entry_count as usize);
        for _i in 0..entry_count {
            table.push(reader.read_u64()?);
        }
        Ok(Self {
            version,
            flags,
            table,
        })
    }
}
