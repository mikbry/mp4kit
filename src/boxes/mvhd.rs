use std::io::Read;

use crate::{BoxHeader, BoxParser, Error};
use crate::boxes::Box;

#[derive(Clone, Debug)]
pub struct MvhdBox {
    pub header: BoxHeader,
}

impl MvhdBox {

}
impl Box for MvhdBox {
    fn parse<'a, T: Read>(parser: &mut BoxParser<T>) -> Result<Self, Error> {
        let header = match parser.next() {
            Some(header) => header,
            None => {
                return Err(Error::EOF());
            }
        };
        Ok(Self {
            header
        })
    }
}