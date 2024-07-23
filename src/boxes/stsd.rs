use std::io::{Read, Seek};

use crate::{BoxHeader, BoxParser, BoxReader, BoxType, Error, Parser, Reader};

// https://developer.apple.com/documentation/quicktime-file-format/video_sample_description
#[derive(Clone, Debug)]
pub struct VideoSampleDescriptionBox {
    pub header: BoxHeader,

    pub version: u8,
    pub flags: u32,

    pub codec: VideoCodec,
}

#[derive(Clone, Debug)]
pub enum VideoCodec {
    Unknown(BoxHeader),
}

impl Reader for VideoSampleDescriptionBox {
    fn read<'a, T: Read + Seek>(reader: &mut BoxReader<T>, header: BoxHeader) -> Result<Self, Error> {
        let (version, flags) = reader.read_header_extra()?;

        let entry_count = reader.read_u32()?;
        if entry_count != 1 {
            return Err(Error::InvalidData(format!("Stsd: invalid entry_count={:?}", entry_count)));
        }
        let child_header = BoxHeader::read(reader)?;
        let codec = match child_header.name {
            BoxType::DataRef => {
                child_header.skip_content(reader, 0)?;
                VideoCodec::Unknown(child_header)
            }
            _ => {
                child_header.skip_content(reader, 0)?;
                VideoCodec::Unknown(child_header)
                // return Err(Error::InvalidBox(format!("Stsd: invalid codec {:?}", child_header.name)));
            }
        };

        Ok(Self {
            header,
            version,
            flags,
            codec,
        })
    }
}

impl Parser for VideoSampleDescriptionBox {
    fn parse<'a, T: Read + Seek>(parser: &mut BoxParser<T>) -> Result<Self, Error> {
        let header = parser.next_header_with_type(BoxType::VideoSampleDescription)?.clone();
        VideoSampleDescriptionBox::read(parser.get_reader(), header)
    }
}