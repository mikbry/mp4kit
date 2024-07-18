use std::io::Read;

use crate::boxes::Box;
use crate::{BoxHeader, BoxParser, Error, MvhdBox};

#[derive(Clone, Debug)]
pub struct MoovBox {
    pub header: BoxHeader,
    pub mvhd: MvhdBox,
}

impl MoovBox {
}

impl Box for MoovBox {
    fn parse<'a, T: Read>(parser: &mut BoxParser<T>) -> Result<Self, Error> {
        let header = match parser.next() {
            Some(header) => header,
            None => {
                return Err(Error::EOF());
            }
        };
        let mvhd = MvhdBox::parse(parser)?;
        Ok(Self {
            header,
            mvhd,
        })
    }
}
