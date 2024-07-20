use std::io::{Read, Seek};

use crate::{BoxHeader, BoxParser, BoxReader, BoxType, Error};

#[derive(Clone, Debug)]
pub struct FtypBox {
    pub header: BoxHeader,

    pub major_brand: String,
    pub minor_brand: u32,
    pub compatible_brands: Vec<String>,
}

impl FtypBox {
    pub fn read<'a, T: Read + Seek>(parser: &mut BoxParser<'a, T>, header: BoxHeader) -> Result<Self, Error> {
        let size = header.size;
        if size < 16 || size % 4 != 0 {
            return Err(Error::InvalidData("ftyp has a wrong size".to_owned()));
        }
        let major_brand = match parser.read_string(4) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        let minor_brand = match parser.read_u32() {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        let size = (size - 16) / 4;
        let mut compatible_brands = vec![];
        for _ in 0..size {
            let brand = match parser.read_string(4) {
                Ok(value) => value,
                Err(error) => return Err(error),
            };
            compatible_brands.push(brand);
        }
        parser.clean();
        Ok(Self {
            header,
            major_brand,
            minor_brand,
            compatible_brands,
        })
    }
}
impl BoxReader for FtypBox {
    fn parse<'a, T: Read + Seek>(parser: &mut BoxParser<'a, T>) -> Result<Self, Error> {
        let header = parser.next_header_with_type(BoxType::FileType)?;
        FtypBox::read(parser, header)
    }
}
