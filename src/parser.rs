use std::io::Read;

pub use error::Error;
pub use fourcc::FourCC as FourCC;
pub use crate::boxes::*;
use crate::{error, fourcc};

#[derive(Debug)]
pub struct BoxParser<'a, T: 'a> {
    src: &'a mut T,
    pub error: Option<Error>,
}

impl<'a, T: Read> BoxParser<'a, T> {
    fn new(src: &mut T) -> BoxParser<T> {
        BoxParser { src, error: None }
    }

    pub fn read_u32(&mut self) -> Result<u32,Error> {
        let mut buf: [u8;4]  = [0; 4];
        if let Err(error) = self.src.read_exact(&mut buf) {
            let error = Error::InvalidData(error.to_string());
            self.error = Some(error.clone());
            return Err(error);
        }
        let value: u32 = u32::from_be_bytes(buf);
        Ok(value)
    }

    pub fn read_u64(&mut self) -> Result<u64,Error> {
        let mut buf: [u8;8]  = [0; 8];
        if let Err(error) = self.src.read_exact(&mut buf) {
            let error = Error::InvalidData(error.to_string());
            self.error = Some(error.clone());
            return Err(error);
        }
        let value = u64::from_be_bytes(buf);
        Ok(value)
    }

    pub fn read_string(&mut self, len: usize) -> Result<String,Error> {
        let mut buf  = Vec::with_capacity(len);
        if let Err(error) = self.src.take(len.try_into().unwrap()).read_to_end(&mut buf) {
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
}

pub fn parse<'a, T: Read>(src: &'a mut T) -> BoxParser<'a, T> {
    let parser = BoxParser::new(src);
    
    parser
}