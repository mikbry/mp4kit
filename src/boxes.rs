
pub mod ftyp;
pub mod moov;
pub mod mvhd;

use std::io::Read;

use crate::{box_definitions, BoxParser, BoxReader, Error};
pub use ftyp::FtypBox as FtypBox;
pub use moov::MoovBox as MoovBox;
pub use mvhd::MvhdBox as MvhdBox;

#[derive(Clone, Copy, Debug)]
pub struct BoxHeader {
    pub r#type: BoxType,
    pub size: u64,
}

impl BoxReader for BoxHeader {
    fn parse<'a, T: Read>(parser: &mut BoxParser<'a, T>) -> Result<Self, Error> {
        let header = parser.next_header_with_type(BoxType::MovieHeader)?.clone();
        Ok(header)
    }
}

box_definitions!(
    FileType    0x66747970u32,  // "ftyp"
    Movie       0x6d6f6f76u32,  // "moov"
    MovieHeader 0x6d766864u32,  // "mvhd"
);
