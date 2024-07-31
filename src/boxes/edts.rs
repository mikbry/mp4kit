use std::io::{Read, Seek};

use crate::{BoxHeader, BoxReader, BoxType, EditListBox, Error, Reader, HEADER_LENGTH};

// https://developer.apple.com/documentation/quicktime-file-format/edit_atom
#[derive(Clone, Debug)]
pub struct EditBox {
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
            list,
        })
    }
}
