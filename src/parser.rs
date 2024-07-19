use std::{cell::RefCell, io::Read, rc::Rc, sync::{Arc, Mutex}};

pub use error::Error;
pub use fourcc::FourCC as FourCC;
use crate::{boxes::{BoxHeader, BoxType}, error, fourcc};

#[derive(Debug)]
pub struct BoxParser<'a, T: 'a> {
    reader: &'a mut T,
    pub error: Option<Error>,
}

impl<'a, T: Read> BoxParser<'a, T> {
    fn new(reader: &mut T) -> BoxParser<T> {
        BoxParser { reader, error: None }
    }

    pub fn read_u32(&mut self) -> Result<u32,Error> {
        let mut buf: [u8;4]  = [0; 4];
        if let Err(error) = self.reader.read_exact(&mut buf) {
            let error = Error::InvalidData(error.to_string());
            self.error = Some(error.clone());
            return Err(error);
        }
        let value: u32 = u32::from_be_bytes(buf);
        Ok(value)
    }

    pub fn read_u64(&mut self) -> Result<u64,Error> {
        let mut buf: [u8;8]  = [0; 8];
        if let Err(error) = self.reader.read_exact(&mut buf) {
            let error = Error::InvalidData(error.to_string());
            self.error = Some(error.clone());
            return Err(error);
        }
        let value = u64::from_be_bytes(buf);
        Ok(value)
    }

    pub fn read_string(&mut self, len: usize) -> Result<String,Error> {
        let mut buf  = Vec::with_capacity(len);
        if let Err(error) = self.reader.take(len.try_into().unwrap()).read_to_end(&mut buf) {
            let error = Error::InvalidData(error.to_string());
            self.error = Some(error.clone());
            return Err(error);
        }
        let value = match String::from_utf8(buf) {
            Ok(v) => v,
            Err(error) => {
                let error = Error::InvalidData(error.to_string());
            self.error = Some(error.clone());
            return Err(error);
            },
        };
        Ok(value)
    }

    pub fn show_error(&self) -> String {
        match &self.error {
            Some(error) => return error.to_string(),
            None => return "Ok".to_string(),
        }
        // return self.error.as_ref().unwrap().into();
    }

    /* pub fn iter(self) -> BoxIter<'a, T> {
        BoxIter {
            parser: Rc::new(RefCell::new(self))
        }
    } */

    fn next(&mut self) -> Result<BoxHeader, Error> {
        let mut size: u64 = self.read_u32()?.into();
        let four_cc = self.read_u32()?;

        if size == 1 {
            let large_size = self.read_u64()?;
            size = match large_size {
                0 => 0,
                1..=15 => {
                    self.error = Some(Error::InvalidData("Invalid Box size".to_owned()));
                    return Err(Error::InvalidData("Invalid Box size".to_owned()));
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
        Ok(parsed_box)
    }

    pub fn next_header(&mut self) -> Result<BoxHeader, Error> {
        let header = self.next()?;        
        Ok(header)
    }

    pub fn next_header_with_type(&mut self, header_type: BoxType) -> Result<BoxHeader, Error> {
        let header = self.next_header()?;
        if header.r#type != header_type {
            return Err(Error::InvalidBoxType());
        }
        
        Ok(header)
    }
}

/* pub struct BoxIter<'a, T: 'a> {
    parser: Rc<RefCell<BoxParser<'a, T>>>,
}

impl<'a, T: Read> BoxIter<'a, T> {

    pub fn next_header(&mut self) -> Result<BoxHeader, Error> {
        let header = match self.next() {
            Some(header) => header,
            None => {
                return Err(Error::EOF());
            }
        };        
        Ok(header)
    }

    pub fn next_header_with_type(&mut self, header_type: BoxType) -> Result<BoxHeader, Error> {
        let header = self.next_header()?;
        if header.r#type != header_type {
            return Err(Error::InvalidBoxType());
        }
        
        Ok(header)
    }
} */

/* impl<'a, T: Read> Iterator for BoxParser<'a, T> {
    type Item = BoxHeader;

    fn next(&mut self) -> Option<Self::Item> {
        let mut parser = self; //.parser.borrow_mut();
        let mut size: u64 = match parser.read_u32() {
            Ok(v) => v.into(),
            Err(_) => return None,
        };
        let four_cc = match parser.read_u32() {
            Ok(v) => v,
            Err(_) => return None,
        };

        if size == 1 {
            let large_size = match parser.read_u64() {
                Ok(v) => v,
                Err(_) => return None,
            };
            size = match large_size {
                0 => 0,
                1..=15 => {
                    parser.error = Some(Error::InvalidData("Invalid Box size".to_owned()));
                    return None;
                },
                _ => large_size - 8,
            };
        }
        if parser.error.is_some() {
            println!("End:{:?}", parser.show_error());
        }
        let parsed_box = BoxHeader {
            r#type: BoxType::from(four_cc),    
            size,       
        };
        Some(parsed_box)
    }
} */

pub fn parse<'a, T: Read>(src: &mut T) -> BoxParser<T> {
    let parser: BoxParser<T> = BoxParser::new(src);
    
    parser
}


pub trait BoxReader {
    fn parse<'a, T: Read>(parser: &mut BoxParser<T>) -> Result<Self, Error> where Self: Sized;
}