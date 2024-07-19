use std::io::Read;

use crate::{BoxHeader, BoxParser, BoxReader, BoxType, Error, MvhdBox};

#[derive(Clone, Debug)]
pub struct MoovBox {
    pub header: BoxHeader,

    pub mvhd: MvhdBox,
}

impl MoovBox {
    pub fn read<'a, T: Read>(parser: &mut BoxParser<T>, header: BoxHeader) -> Result<Self, Error> {
        let mvhd = MvhdBox::parse(parser)?;
        Ok(Self {
            header,
            mvhd: mvhd.clone(),
        })
    }
}
impl BoxReader  for MoovBox {
    fn parse<'a, T: Read>(mut parser: &mut BoxParser<T>) -> Result<Self, Error> {
        let header = parser.next_header_with_type(BoxType::Movie)?;
        MoovBox::read(parser, header)
    }
}
