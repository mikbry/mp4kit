use std::io::{Read, Seek};

use crate::{BoxContainer, BoxHeader, BoxReader, Error, Reader};

// https://developer.apple.com/documentation/quicktime-file-format/sample_table_atom
#[derive(Clone, Debug)]
pub struct SampleTableBox {
    pub header: BoxHeader,
    pub content: BoxContainer,

}

impl Reader for SampleTableBox {
    fn read<'a, T: Read + Seek>(reader: &mut BoxReader<T>, header: BoxHeader) -> Result<Self, Error> {
        let content = BoxContainer::read(reader, header)?;
        Ok(Self {
            header,
            content,
        })
    }
}
