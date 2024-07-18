
mod boxes;
mod fourcc;
mod error;
mod parser;

use std::io::Read;

pub use error::Error;
pub use fourcc::FourCC as FourCC;
pub use crate::boxes::*;
pub use parser::*;

impl<'a, T: Read> Iterator for BoxParser<'a, T> {
    type Item = BoxHeader;

    fn next(&mut self) -> Option<Self::Item> {
        let mut size: u64 = match self.read_u32() {
            Ok(v) => v.into(),
            Err(_) => return None,
        };
        let four_cc = match self.read_u32() {
            Ok(v) => v,
            Err(_) => return None,
        };

        if size == 1 {
            let large_size = match self.read_u64() {
                Ok(v) => v,
                Err(_) => return None,
            };
            size = match large_size {
                0 => 0,
                1..=15 => {
                    self.error = Some(Error::InvalidData("Invalid Box size".to_owned()));
                    return None;
                },
                _ => large_size - 8,
            };
        }
        if self.error.is_some() {
            println!("End:{:?}", self.show_error());
        }
        let parsed_box = BoxHeader {
            r#type: BoxType::from(four_cc),    
            size,       
        };
        Some(parsed_box)
    }
}
