
pub mod ftyp;
pub mod moov;
pub mod mvhd;
pub mod trak;
pub mod mdat;
pub mod udta;
pub mod wide;
pub mod tkhd;
pub mod edts;
pub mod elst;
pub mod mdia;
pub mod mdhd;
pub mod hdlr;
pub mod minf;
pub mod vmhd;
pub mod smhd;
pub mod dinf;
pub mod dref;
pub mod stbl;

use std::io::{Read, Seek};

use crate::{box_definitions, BoxParser, BoxReader, Error, Parser, Reader};

pub use ftyp::FtypBox as FtypBox;
pub use moov::MoovBox as MoovBox;
pub use mvhd::MvhdBox as MvhdBox;
pub use tkhd::TrackHeaderBox as TrackHeaderBox;
pub use trak::TrackBox as TrackBox;
pub use mdat::MediaDataBox as MediaDataBox;
pub use udta::UserDataBox as UserDataBox;
pub use wide::WideBox as WideBox;
pub use edts::EditBox as EditBox;
pub use elst::EditListBox as EditListBox;
pub use mdia::MediaBox as MediaBox;
pub use mdhd::MediaHeaderBox as MediaHeaderBox;
pub use hdlr::HandlerBox as HandlerBox;
pub use minf::MediaInfoBox as MediaInfoBox;
pub use vmhd::VideoInfoBox as VideoInfoBox;
pub use smhd::SoundInfoBox as SoundInfoBox;
pub use dinf::DataInfoBox as DataInfoBox;
pub use dref::DataReferenceBox;

pub use stbl::SampleTableBox as SampleTableBox;

pub const HEADER_LENGTH: u64 = 8;

#[derive(Clone, Copy, Debug)]
pub struct BoxHeader {
    pub name: BoxType,
    pub start: u64,
    pub size: u64,
}

impl BoxHeader {
    pub fn skip_content<'a, T: Read + Seek>(&self, reader: &mut BoxReader<'a, T>, offset: u64) -> Result<(), Error> {
        let content_size = self.size - HEADER_LENGTH - offset;
        reader.skip(content_size)?;
        Ok(())
    }

    pub fn root(name: &str) -> Self {
        Self {
            name: BoxType::Root(FourCC::from_str(name)),
            start: 0,
            size: 0,
        }
    }

    fn read<'a, T: Read + Seek>(reader: &mut BoxReader<'a, T>) -> Result<Self, Error> {
        let start = reader.stream_position()
            .map_err(|error| Error::InvalidData(error.to_string()))?;
        let mut size: u64 = reader.read_u32()? as u64;
        let four_cc = reader.read_u32()?;

        if size == 1 {
            let large_size = reader.read_u64()?;
            size = match large_size {
                0 => 0,
                1..=15 => {
                    return Err(Error::InvalidData("Invalid Box size".to_owned()));
                }
                _ => large_size - 8, // Remove large_size offset = 8
            };
        }

        Ok(BoxHeader {
            name: BoxType::from(four_cc),
            size,
            start,
        })
    }
}

impl Parser for BoxHeader {
    fn parse<'a, T: Read + Seek>(parser: &mut BoxParser<'a, T>) -> Result<Self, Error> {
        let header = BoxHeader::read(parser.get_reader())?;
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
    Tkhd(TrackHeaderBox),
    Edts(EditBox),
    // Elst(EditListBox), // Elst is only present in Edts
    Mdia(MediaBox),
    Mdhd(MediaHeaderBox),
    Hdlr(HandlerBox),
    Minf(MediaInfoBox),
    Vmhd(VideoInfoBox),
    Smhd(SoundInfoBox),
    Dinf(DataInfoBox),
    // Dref(DataReferenceBox), // Dref isonly present in Dinf

    Stbl(SampleTableBox),

    Unknown(BoxHeader),
}

#[derive(Clone, Debug)]
pub struct BoxContainer {
    pub children: Vec<ChildBox>,
    // TODO add rules
}

impl BoxContainer {
    fn read_box<'a, T: Read + Seek>(reader: &mut BoxReader<'a, T>, header: BoxHeader) -> Result<ChildBox, Error> {
        let result = match header.name {
            BoxType::FileType => {
                let ftyp_box = FtypBox::read(reader, header)?;
                ChildBox::Ftyp(ftyp_box)
            },
            BoxType::Movie => {
                let moov_box = MoovBox::read(reader, header)?;
                ChildBox::Moov(moov_box)
            },
            BoxType::MovieHeader => {
                let mvhd_box = MvhdBox::read(reader, header)?;
                ChildBox::Mvhd(mvhd_box)
            },
            BoxType::Track => {
                let track_box = TrackBox::read(reader, header)?;
                ChildBox::Trak(track_box)
            },
            BoxType::MediaData => {
                let mediadata_box = MediaDataBox::read(reader, header)?;
                ChildBox::Mdat(mediadata_box)
            },
            BoxType::UserData => {
                let userdata_box = UserDataBox::read(reader, header)?;
                ChildBox::Udta(userdata_box)
            },
            BoxType::Wide => {
                let wide_box = WideBox::read(reader, header)?;
                ChildBox::Wide(wide_box)
            },
            BoxType::TrackHeader => {
                let trackheader_box = TrackHeaderBox::read(reader, header)?;
                ChildBox::Tkhd(trackheader_box)
            },
            BoxType::Edit => {
                let edit_box = EditBox::read(reader, header)?;
                ChildBox::Edts(edit_box)
            },
            BoxType::Media => {
                let media_box = MediaBox::read(reader, header)?;
                ChildBox::Mdia(media_box)
            },
            BoxType::MediaHeader => {
                let mediaheader_box = MediaHeaderBox::read(reader, header)?;
                ChildBox::Mdhd(mediaheader_box)
            },
            BoxType::Handler => {
                let handler_box = HandlerBox::read(reader, header)?;
                ChildBox::Hdlr(handler_box)
            },
            BoxType::MediaInfo => {
                let mediainfo_box = MediaInfoBox::read(reader, header)?;
                ChildBox::Minf(mediainfo_box)
            },
            BoxType::VideoInfo => {
                let videoinfo_box = VideoInfoBox::read(reader, header)?;
                ChildBox::Vmhd(videoinfo_box)
            },
            BoxType::SoundInfo => {
                let soundinfo_box = SoundInfoBox::read(reader, header)?;
                ChildBox::Smhd(soundinfo_box)
            },
            BoxType::DataInfo => {
                let datainfo_box = DataInfoBox::read(reader, header)?;
                ChildBox::Dinf(datainfo_box)
            },
            BoxType::SampleTable => {
                let sampletable_box = SampleTableBox::read(reader, header)?;
                ChildBox::Stbl(sampletable_box)
            },
            _ => {
                ChildBox::Unknown(header)
            },
        };
        Ok(result)
    }
}

impl Reader for BoxContainer {
    fn read<'a, T: Read + Seek>(reader: &mut BoxReader<T>, header: BoxHeader) -> Result<Self, Error> {
        let mut children: Vec<ChildBox> = Vec::new();
        let mut content_parsed_size: u64  = 0;
        loop  {
            if header.size > 0 && content_parsed_size >= header.size - HEADER_LENGTH {
                break;
            }
            let child_header = match BoxHeader::read(reader) {
                Ok(header) => header,
                Err(error) => {
                    if error == Error::EOF() {
                        break; 
                    }
                    return Err(error);
                },
            };
            let child = BoxContainer::read_box(reader, child_header)?;
            println!("{:?}: {:?}", header.name, child);
            
            if let ChildBox::Unknown(unknown_box) = child {
                unknown_box.skip_content(reader, 0)?;
            }

            children.push(child);
                    
            content_parsed_size += child_header.size;
        };
        Ok(Self {
            children,
        })
    }
}

impl Parser  for BoxContainer {
    fn parse<'a, T: Read + Seek>(parser: &mut BoxParser<T>) -> Result<Self, Error> {
        let header = parser.next_header_with_type(BoxType::Movie)?;
        BoxContainer::read(parser.get_reader(), header)
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
    TrackHeader 0x746b6864u32,  // "tkhd"
    Edit        0x65647473u32,  // "edts"
    EditList    0x656c7374u32,  // "elst"
    Media       0x6d646961u32,  // "mdia"
    MediaHeader 0x6d646864u32,  // "mdhd"
    Handler     0x68646c72u32,  // "hdlr"
    MediaInfo   0x6d696e66u32,  // "minf"
    VideoInfo   0x766d6864u32,  // "vmhd"
    SoundInfo   0x736d6864u32,  // "smhd"
    DataInfo    0x64696e66u32,  // "dinf"
    DataRef     0x64726566u32,  // "dref"
    UrlRef      0x75726c20u32,  // "url "

    SampleTable 0x7374626cu32,  // "stbl"
);
