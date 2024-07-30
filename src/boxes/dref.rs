use std::io::{Read, Seek};

use crate::{BoxHeader, BoxReader, BoxType, Error, Reader, HEADER_LENGTH};

// https://developer.apple.com/documentation/quicktime-file-format/media_data_reference_atom
#[derive(Clone, Debug)]
pub struct DataReferenceBox {
    pub header: BoxHeader,

    pub version: u8,
    pub flags: u32,

    pub references: Vec<Reference>,
}

#[derive(Debug, Clone)]
pub enum Reference {
    Url(UrlBox)
}

impl Reader for DataReferenceBox {
    fn read<'a, T: Read + Seek>(reader: &mut BoxReader<T>, header: BoxHeader) -> Result<Self, Error> {
        let (version, flags) = reader.read_header_extra()?;

        let entry_count = reader.read_u32()?;
        let mut content_parsed_size: u64  = 8 + HEADER_LENGTH;
        let mut references = Vec::with_capacity(entry_count as usize);
        for _i in 0..entry_count  {
            if content_parsed_size >= header.size {
                println!("Dref: wrong size {:?}", content_parsed_size);
                break;
            }
            let child_header = match BoxHeader::read(reader) {
                Ok(header) => header,
                Err(error) => {
                    if error == Error::EOF() {
                        break; 
                    }
                    return Err(error);
                },
            };
            match child_header.name {
                BoxType::UrlRef => {
                    let url_box = UrlBox::read(reader, child_header)?;
                    references.push(Reference::Url(url_box));
                },
                _ => {
                    println!("Dref: unknown box {:?}", child_header);
                    header.skip_content(reader, 0)?;
                }
            };
            content_parsed_size += child_header.size;
        }

        Ok(Self {
            header,

            version,
            flags,

            references,
        })
    }
}

#[derive(Debug, Clone)]
pub struct UrlBox {
    pub header: BoxHeader,
    pub version: u8,
    pub flags: u32,
    pub location: String,
}

impl Reader for UrlBox {
    fn read<'a, T: Read + Seek>(reader: &mut BoxReader<T>, header: BoxHeader) -> Result<Self, Error> {
        let (version, flags) = reader.read_header_extra()?;
        let len = header.size - HEADER_LENGTH - 4; 
        let location = reader.read_string(len as usize)?;
        Ok(Self {
            header,
            version,
            flags,
            location,
        })
    }
}