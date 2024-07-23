use std::io::{Read, Seek};

use crate::{BoxHeader, BoxParser, BoxReader, BoxType, EditListBox, Error, Parser, Reader, HEADER_LENGTH};

// https://developer.apple.com/documentation/quicktime-file-format/edit_atom
#[derive(Clone, Debug)]
pub struct EditBox {
    pub header: BoxHeader,
    pub list: Option<EditListBox>,

}

impl Reader for EditBox {
    fn read<'a, T: Read + Seek>(reader: &mut BoxReader<T>, header: BoxHeader) -> Result<Self, Error> {
        let mut list: Option<EditListBox> = None;
        if header.size > HEADER_LENGTH {
            let list_header = BoxHeader::read(reader)?;
            match list_header.name {
                BoxType::EditList => {
                    list = Some(EditListBox::read(reader, list_header)?);
                }
                _ => {
                    return Err(Error::InvalidBox(format!("Edts: invalid child type only elst is valid {:?}", list_header.name)));
                }
            }
        }
        Ok(Self {
            header,
            list,
        })
    }
}

impl Parser for EditBox {
    fn parse<'a, T: Read + Seek>(parser: &mut BoxParser<T>) -> Result<Self, Error> {
        let header = parser.next_header_with_type(BoxType::Edit)?.clone();
        EditBox::read(parser.get_reader(), header)
    }
}