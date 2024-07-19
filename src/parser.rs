use std::io::{self, Read, Seek};

pub use error::Error;
pub use fourcc::FourCC as FourCC;
use crate::{boxes::{BoxHeader, BoxType}, error, fourcc};

#[derive(Debug)]
pub struct BoxParser<'a, T: 'a> {
    reader: &'a mut T,
    pub error: Option<Error>,
    pub count: u64,
}

impl<'a, T: Read + Seek> BoxParser<'a, T> {
    fn new(reader: &mut T) -> BoxParser<T> {
        BoxParser { reader, error: None, count: 0 }
    }

    pub fn clean(&mut self) {
        self.count = 0;
    }

    fn set_error(&mut self, error: io::Error) -> Error {
        let error = Error::InvalidData(error.to_string());
        self.error = Some(error.clone());
        return error;
    }

    pub fn seek(&mut self, position: i64) -> Result<u64,Error> {
        let seek = self.reader.seek(io::SeekFrom::Current(position)).map_err(|error| self.set_error(error))?;
        // self.count -= seek;
        println!("seek:{:?} {:?} {:?}", self.count, seek, position);
        Ok(seek) 
    }

    pub fn read_u32(&mut self) -> Result<u32,Error> {
        let mut buf: [u8;4]  = [0; 4];
        if let Err(error) = self.reader.read_exact(&mut buf) {
            return Err(self.set_error(error));
        }
        let value: u32 = u32::from_be_bytes(buf);
        Ok(value)
    }

    pub fn read_u64(&mut self) -> Result<u64,Error> {
        let mut buf: [u8;8]  = [0; 8];
        if let Err(error) = self.reader.read_exact(&mut buf) {
            return Err(self.set_error(error));
        }
        let value = u64::from_be_bytes(buf);
        Ok(value)
    }

    pub fn read_string(&mut self, len: usize) -> Result<String,Error> {
        let mut buf  = Vec::with_capacity(len);
        if let Err(error) = self.reader.take(len.try_into().unwrap()).read_to_end(&mut buf) {
            return Err(self.set_error(error));
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
    }

    fn next(&mut self) -> Result<BoxHeader, Error> {
        if self.count > 0 {
            self.seek(self.count.try_into().unwrap())?;
        }
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
                _ => large_size - 8, // Remove large_size offset = 8
            };
        }
        if self.error.is_some() {
            println!("End:{:?}", self.show_error());
        }
        println!("count: {:?} {:?} {:#x}", self.count, size, four_cc);
        if size > 8 {
            self.count = size - 8;
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

pub fn parse<'a, T: Read + Seek>(src: &mut T) -> BoxParser<T> {
    let parser: BoxParser<T> = BoxParser::new(src);
    
    parser
}

pub trait BoxReader {
    fn parse<'a, T: Read + Seek>(parser: &mut BoxParser<T>) -> Result<Self, Error> where Self: Sized;
}