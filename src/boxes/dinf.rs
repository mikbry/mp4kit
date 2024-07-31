use std::io::{Read, Seek};

use crate::{dref::DataReferenceBox, BoxHeader, BoxReader, BoxType, Error, Reader};

// https://developer.apple.com/documentation/quicktime-file-format/data_information_atom
#[derive(Clone, Debug)]
pub struct DataInfoBox {
    pub data_reference: DataReferenceBox,
}

impl Reader for DataInfoBox {
    fn read<'a, T: Read + Seek>(reader: &mut BoxReader<T>, _header: BoxHeader) -> Result<Self, Error> {
        let child_header = BoxHeader::read(reader)?;
        let data_reference = match child_header.name {
            BoxType::DataRef => {
                DataReferenceBox::read(reader, child_header)?
            }
            _ => {
                return Err(Error::InvalidBox(format!("Dinf: invalid child type only dref is valid {:?}", child_header.name)));
            }
        };
        Ok(Self {
            data_reference,
        })
    }
}
