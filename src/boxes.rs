
pub mod ftyp;
pub mod moov;
pub mod mvhd;
pub mod trak;
pub mod mdat;
pub mod udta;
pub mod wide;

use std::io::{Read, Seek};

use crate::{box_definitions, BoxParser, BoxReader, Error};

pub use ftyp::FtypBox as FtypBox;
pub use moov::MoovBox as MoovBox;
pub use mvhd::MvhdBox as MvhdBox;
pub use trak::TrackBox as TrackBox;
pub use mdat::MediaDataBox as MediaDataBox;
pub use udta::UserDataBox as UserDataBox;
use wide::WideBox;

#[derive(Clone, Copy, Debug)]
pub struct BoxHeader {
    pub r#type: BoxType,
    pub start: u64,
    pub size: u64,
}

impl BoxHeader {
    pub fn skip_content<'a, T: Read + Seek>(&self, parser: &mut BoxParser<'a, T>, offset: u64) -> Result<(), Error> {
        let content_size = self.size - 8 - offset;
        parser.skip(content_size)?;
        Ok(())
    }

    pub fn root(name: &str) -> Self {
        Self {
            r#type: BoxType::Root(FourCC::from_str(name)),
            start: 0,
            size: 0,
        }
    }
}

impl BoxReader for BoxHeader {
    fn parse<'a, T: Read + Seek>(parser: &mut BoxParser<'a, T>) -> Result<Self, Error> {
        let header = parser.next_header()?.clone();
        // println!("{header:?}");
        Ok(header)
    }
}

#[derive(Clone, Debug)]
pub enum ChildBox {
    Ftyp(FtypBox),
    Moov(MoovBox),
    Mvhd(MvhdBox),
    Trak(TrackBox),
    Mdat(MediaDataBox),
    Udta(UserDataBox),
    Wide(WideBox),
    Unknown(BoxHeader),
}

#[derive(Clone, Debug)]
pub struct BoxContainer {
    pub children: Vec<ChildBox>,
    // TODO add rules
}

impl BoxContainer {
    fn read_box<'a, T: Read + Seek>(parser: &mut BoxParser<'a, T>, header: BoxHeader) -> Result<ChildBox, Error> {
        let result = match header.r#type {
            BoxType::FileType => {
                let ftyp_box = FtypBox::read(parser, header)?;
                ChildBox::Ftyp(ftyp_box)
            },
            BoxType::Movie => {
                let moov_box = MoovBox::read(parser, header)?;
                ChildBox::Moov(moov_box)
            },
            BoxType::MovieHeader => {
                let mvhd_box = MvhdBox::read(parser, header)?;
                ChildBox::Mvhd(mvhd_box)
            },
            BoxType::Track => {
                let track_box = TrackBox::read(parser, header)?;
                ChildBox::Trak(track_box)
            },
            BoxType::MediaData => {
                let mediadata_box = MediaDataBox::read(parser, header)?;
                ChildBox::Mdat(mediadata_box)
            },
            BoxType::UserData => {
                let userdata_box = UserDataBox::read(parser, header)?;
                ChildBox::Udta(userdata_box)
            },
            BoxType::Wide => {
                let wide_box = WideBox::read(parser, header)?;
                ChildBox::Wide(wide_box)
            },
            _ => {
                ChildBox::Unknown(header)
            },
        };
        Ok(result)
    }

    pub fn read<'a, T: Read + Seek>(parser: &mut BoxParser<T>, header: BoxHeader) -> Result<Self, Error> {
        parser.clean();
        let mut children: Vec<ChildBox> = Vec::new();
        let mut content_parsed_size: u64  = 0;
        loop  {
            if header.size > 0 && content_parsed_size >= header.size - 8 {
                break;
            }
            let child_header = match BoxHeader::parse(parser) {
                Ok(header) => header,
                Err(error) => {
                    if error == Error::EOF() {
                        break; 
                    }
                    return Err(error);
                },
            };
            let child = BoxContainer::read_box(parser, child_header)?;
            println!("{:?}: {:?}", header.r#type, child);
            
            if let ChildBox::Unknown(unknown_box) = child {
                unknown_box.skip_content(parser, 0)?;
            }

            children.push(child);
                    
            content_parsed_size += child_header.size;
        };
        Ok(Self {
            children,
        })
    }
}
impl BoxReader  for BoxContainer {
    fn parse<'a, T: Read + Seek>(parser: &mut BoxParser<T>) -> Result<Self, Error> {
        let header = parser.next_header_with_type(BoxType::Movie)?;
        BoxContainer::read(parser, header)
    }
}

box_definitions!(
    FileType    0x66747970u32,  // "ftyp"
    Movie       0x6d6f6f76u32,  // "moov"
    MovieHeader 0x6d766864u32,  // "mvhd"
    Track       0x7472616bu32,  // "trak"
    MediaData   0x6d646174u32,  // "mdat"
    UserData    0x75647461u32,  // "udta"
    Wide        0x77696465u32,  // "wide" 
);
