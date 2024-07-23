use std::io::{Read, Seek};

use crate::{dref::DataReferenceBox, BoxHeader, BoxParser, BoxReader, BoxType, Error, Parser, Reader};

// https://developer.apple.com/documentation/quicktime-file-format/data_information_atom
#[derive(Clone, Debug)]
pub struct DataInfoBox {
    pub header: BoxHeader,
    pub data_reference: DataReferenceBox,

}

impl Reader for DataInfoBox {
    fn read<'a, T: Read + Seek>(reader: &mut BoxReader<T>, header: BoxHeader) -> Result<Self, Error> {
        let child_header = BoxHeader::read(reader)?;
        let data_reference = match child_header.r#type {
            BoxType::DataRef => {
                DataReferenceBox::read(reader, child_header)?
            }
            _ => {
                return Err(Error::InvalidBox(format!("Dinf: invalid child type only dref is valid {:?}", child_header.r#type)));
            }
        };
        Ok(Self {
            header,

            data_reference,
        })
    }
}

impl Parser for DataInfoBox {
    fn parse<'a, T: Read + Seek>(parser: &mut BoxParser<T>) -> Result<Self, Error> {
        let header = parser.next_header_with_type(BoxType::Track)?.clone();
        DataInfoBox::read(parser.get_reader(), header)
    }
}