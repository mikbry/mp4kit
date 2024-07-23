use std::io::{self, ErrorKind, Read, Seek, SeekFrom};

use crate::{
    boxes::{BoxHeader, BoxType},
    error, fourcc,
};
pub use error::Error;
pub use fourcc::FourCC;

#[derive(Debug)]
pub struct BoxReader<'a, T: 'a> {
    src: &'a mut T,
    pub error: Option<Error>,
}

impl<'a, T: Read + Seek> BoxReader<'a, T> {
    fn new(src: &'a mut T) -> BoxReader<T> {
        Self {
            src,
            error: None,
        }
    }

    fn set_error(&mut self, error: io::Error) -> Error {
        if error.kind() == ErrorKind::UnexpectedEof {
            return Error::EOF();
        }
        let error = Error::InvalidData(error.to_string());
        self.error = Some(error.clone());
        return error;
    }

    pub fn stream_position(&mut self) -> Result<u64, Error> {
        self.src.stream_position().map_err(|error| self.set_error(error))
    }

    pub fn skip(&mut self, size: u64) -> Result<(), Error> {
        self.src
            .seek(SeekFrom::Current(size as i64))
            .map_err(|error| self.set_error(error))?;
        Ok(())
    }

    pub fn read_u8(&mut self) -> Result<u8, Error> {
        let mut buf: [u8; 1] = [0; 1];
        if let Err(error) = self.src.read_exact(&mut buf) {
            return Err(self.set_error(error));
        }
        Ok(buf[0])
    }

    pub fn read_u16(&mut self) -> Result<u16, Error> {
        let mut buf: [u8; 2] = [0; 2];
        if let Err(error) = self.src.read_exact(&mut buf) {
            return Err(self.set_error(error));
        }
        let value: u16 = u16::from_be_bytes(buf);
        Ok(value)
    }

    pub fn read_u32(&mut self) -> Result<u32, Error> {
        let mut buf: [u8; 4] = [0; 4];
        if let Err(error) = self.src.read_exact(&mut buf) {
            return Err(self.set_error(error));
        }
        let value: u32 = u32::from_be_bytes(buf);
        Ok(value)
    }

    pub fn read_i32(&mut self) -> Result<i32, Error> {
        let mut buf: [u8; 4] = [0; 4];
        if let Err(error) = self.src.read_exact(&mut buf) {
            return Err(self.set_error(error));
        }
        let value: i32 = i32::from_be_bytes(buf);
        Ok(value)
    }

    pub fn read_u64(&mut self) -> Result<u64, Error> {
        let mut buf: [u8; 8] = [0; 8];
        if let Err(error) = self.src.read_exact(&mut buf) {
            return Err(self.set_error(error));
        }
        let value = u64::from_be_bytes(buf);
        Ok(value)
    }

    pub fn read_i64(&mut self) -> Result<i64, Error> {
        let mut buf: [u8; 8] = [0; 8];
        if let Err(error) = self.src.read_exact(&mut buf) {
            return Err(self.set_error(error));
        }
        let value = i64::from_be_bytes(buf);
        Ok(value)
    }

    pub fn read_string(&mut self, len: usize) -> Result<String, Error> {
        let mut buf = Vec::with_capacity(len);
        if let Err(error) = self.src.take(len.try_into().unwrap()).read_to_end(&mut buf) {
            return Err(self.set_error(error));
        }
        if let Some(end) = buf.iter().position(|&b| b == b'\0') {
            buf.truncate(end);
        }
        let value = match String::from_utf8(buf) {
            Ok(v) => v,
            Err(error) => {
                let error = Error::InvalidData(error.to_string());
                self.error = Some(error.clone());
                return Err(error);
            }
        };
        Ok(value)
    }

    pub fn read_header_extra(&mut self) -> Result<(u8, u32), Error> {
        let version = self.read_u8()?;
        let mut buf: [u8; 3] = [0; 3];
        if let Err(error) = self.src.read_exact(&mut buf) {
            return Err(self.set_error(error));
        }
        Ok((
            version,
            u32::from(buf[0]) << 16 | u32::from(buf[1]) << 8 | u32::from(buf[2]),
        ))
    }

    pub fn show_error(&self) -> String {
        match &self.error {
            Some(error) => return error.to_string(),
            None => return "Ok".to_string(),
        }
    }
}


pub trait Reader {
    fn read<'a, T: Read + Seek>(reader: &mut BoxReader<T>, header: BoxHeader) -> Result<Self, Error>
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct BoxParser<'a, T: 'a> {
    reader: BoxReader<'a, T>,
}

impl<'a, T: Read + Seek> BoxParser<'a, T> {
    fn new(src: &'a mut T) -> BoxParser<'a, T> {
        BoxParser {
            reader: BoxReader::new(src),
        }
    }

    pub fn get_reader(&mut self) -> &mut BoxReader<'a, T> {
        return &mut self.reader;
    }

    pub fn next_header(&mut self) -> Result<BoxHeader, Error> {
        let header = BoxHeader::parse(self)?;
        Ok(header)
    }

    pub fn next_header_with_type(&mut self, header_type: BoxType) -> Result<BoxHeader, Error> {
        let header = self.next_header()?;
        if header.name != header_type {
            return Err(Error::InvalidBoxType());
        }

        Ok(header)
    }
}

pub fn parse<'a, T: Read + Seek>(src: &mut T) -> BoxParser<T> {
    let parser: BoxParser<T> = BoxParser::new(src);
    parser
}

pub trait Parser {
    fn parse<'a, T: Read + Seek>(parser: &mut BoxParser<T>) -> Result<Self, Error>
    where
        Self: Sized;
}
