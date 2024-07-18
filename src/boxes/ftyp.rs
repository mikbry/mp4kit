use std::io::Read;

use crate::boxes::Box;
use crate::{BoxHeader, BoxParser, Error};

#[derive(Clone, Debug)]
pub struct FtypBox {
    pub header: BoxHeader,
    pub major_brand: String,
    pub minor_brand: u32,
    pub compatible_brands: Vec<String>,
}

impl FtypBox {

}
impl Box for FtypBox {
    fn parse<'a, T: Read>(parser: &mut BoxParser<T>) -> Result<Self, Error> {
        let header = match parser.next() {
            Some(header) => header,
            None => {
                return Err(Error::EOF());
            }
        };
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
        Ok(Self {
            header,
            major_brand,
            minor_brand,
            compatible_brands,
        })
    }
}
