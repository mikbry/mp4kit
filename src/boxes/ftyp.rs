use std::io::{Read, Seek};

use crate::{BoxHeader, BoxReader, Error, Reader};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FtypBox {
    pub major_brand: String,
    pub minor_brand: u32,
    pub compatible_brands: Vec<String>,
}

impl Default for FtypBox {
    fn default() -> Self {
        Self {
            major_brand: Default::default(),
            minor_brand: Default::default(),
            compatible_brands: Default::default(),
        }
    }
}

impl Reader for FtypBox {
    fn read<'a, T: Read + Seek>(
        reader: &mut BoxReader<'a, T>,
        header: BoxHeader,
    ) -> Result<Self, Error> {
        let size = header.size;
        if size < 16 || size % 4 != 0 {
            return Err(Error::InvalidData("ftyp has a wrong size".to_owned()));
        }
        let major_brand = match reader.read_string(4) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        let minor_brand = match reader.read_u32() {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        let size = (size - 16) / 4;
        let mut compatible_brands = vec![];
        for _ in 0..size {
            let brand = match reader.read_string(4) {
                Ok(value) => value,
                Err(error) => return Err(error),
            };
            compatible_brands.push(brand);
        }

        Ok(Self {
            major_brand,
            minor_brand,
            compatible_brands,
        })
    }
}
