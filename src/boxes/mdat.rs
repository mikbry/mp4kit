use std::io::{Read, Seek};

use crate::{BoxHeader, BoxParser, BoxReader, BoxType, Error, Parser, Reader};

// https://developer.apple.com/documentation/quicktime-file-format/movie_header_atom
#[derive(Clone, Debug)]
pub struct MediaDataBox {
    pub header: BoxHeader,

}

impl Reader for MediaDataBox {
    fn read<'a, T: Read + Seek>(reader: &mut BoxReader<T>, header: BoxHeader) -> Result<Self, Error> {
        header.skip_content(reader, 0)?;
        Ok(Self {
            header,
        })
    }
}

impl Parser for MediaDataBox {
    fn parse<'a, T: Read + Seek>(parser: &mut BoxParser<T>) -> Result<Self, Error> {
        let header = parser.next_header_with_type(BoxType::Track)?.clone();
        MediaDataBox::read(parser.get_reader(), header)
    }
}