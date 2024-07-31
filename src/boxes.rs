pub mod dinf;
pub mod dref;
pub mod edts;
pub mod elst;
pub mod ftyp;
pub mod hdlr;
pub mod mdat;
pub mod mdhd;
pub mod mdia;
pub mod minf;
pub mod moov;
pub mod mvhd;
pub mod smhd;
pub mod tkhd;
pub mod trak;
pub mod udta;
pub mod vmhd;
pub mod wide;

pub mod co64;
pub mod ctts;
pub mod stbl;
pub mod stco;
pub mod stsc;
pub mod stsd;
pub mod stss;
pub mod stsz;
pub mod stts;

use std::io::{Read, Seek};

use crate::{box_types, BoxParser, BoxReader, Error, Parser, Reader};

pub use dinf::DataInfoBox;
pub use dref::DataReferenceBox;
pub use edts::EditBox;
pub use elst::EditListBox;
pub use ftyp::FtypBox;
pub use hdlr::HandlerBox;
pub use mdat::MediaDataBox;
pub use mdhd::MediaHeaderBox;
pub use mdia::MediaBox;
pub use minf::MediaInfoBox;
pub use moov::MoovBox;
pub use mvhd::MvhdBox;
pub use smhd::SoundInfoBox;
pub use tkhd::TrackHeaderBox;
pub use trak::TrackBox;
pub use udta::UserDataBox;
pub use vmhd::VideoInfoBox;
pub use wide::WideBox;

pub use co64::ChunkOffset64Box;
pub use ctts::CompositionOffsetBox;
pub use stbl::SampleTableBox;
pub use stco::ChunkOffsetBox;
pub use stsc::SampleToChunkBox;
pub use stsd::VideoSampleDescriptionBox;
pub use stss::SyncSampleBox;
pub use stsz::SampleSizeBox;
pub use stts::TimeToSampleBox;

pub const HEADER_LENGTH: u64 = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoxHeader {
    pub name: BoxType,
    pub start: u64,
    pub size: u64,
}

impl BoxHeader {
    pub fn skip_content<'a, T: Read + Seek>(
        &self,
        reader: &mut BoxReader<'a, T>,
        offset: u64,
    ) -> Result<(), Error> {
        let content_size = self.size - HEADER_LENGTH - offset;
        reader.skip(content_size)?;
        Ok(())
    }

    pub fn root(name: &str) -> Self {
        Self {
            name: BoxType::Root(FourCC::from_str(name)),
            start: 0,
            size: HEADER_LENGTH,
        }
    }

    fn read<'a, T: Read + Seek>(reader: &mut BoxReader<'a, T>) -> Result<Self, Error> {
        let start = reader
            .stream_position()
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
pub struct ListBox {
    pub children: Vec<BoxElement>,
}

impl ListBox {
    pub fn iter(header: BoxHeader) -> ListBoxIterator {
        ListBoxIterator {
            content_parsed_size: 0,
            content_size: header.size - HEADER_LENGTH,
        }
    }
}
impl Reader for ListBox {
    fn read<'a, T: Read + Seek>(
        reader: &mut BoxReader<T>,
        header: BoxHeader,
    ) -> Result<Self, Error> {
        let mut children: Vec<BoxElement> = Vec::new();
        let mut iter = Self::iter(header);
        while let Some(child) = iter.next(reader)? {
            println!("{:?}: {:?}", header.name, child);
            children.push(child);
        }
        Ok(Self { children })
    }
}

#[derive(Clone, Debug)]
pub struct ListBoxIterator {
    content_parsed_size: u64,
    content_size: u64,
}

impl ListBoxIterator {
    pub fn next<'a, T: Read + Seek>(
        &mut self,
        reader: &mut BoxReader<T>,
    ) -> Result<Option<BoxElement>, Error> {
        if self.content_size > 0 && self.content_parsed_size >= self.content_size {
            return Ok(None);
        }
        let child_header = match BoxHeader::read(reader) {
            Ok(header) => header,
            Err(error) => {
                if error == Error::EOF() {
                    return Ok(None);
                }
                return Err(error);
            }
        };
        self.content_parsed_size += child_header.size;
        Ok(Some(BoxElement::read(reader, child_header)?))
    }
}
#[derive(Clone, Debug)]
pub struct SkipBox {}

impl Reader for SkipBox {
    fn read<'a, T: Read + Seek>(
        reader: &mut BoxReader<T>,
        header: BoxHeader,
    ) -> Result<Self, Error> {
        header.skip_content(reader, 0)?;
        Ok(Self {})
    }
}

#[derive(Clone, Debug)]
pub enum BoxContent {
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
    // Dref(DataReferenceBox), // Dref is only present in Dinf
    Stbl(SampleTableBox),
    Stsd(VideoSampleDescriptionBox),
    Stts(TimeToSampleBox),
    Stsc(SampleToChunkBox),
    Stsz(SampleSizeBox),
    Stss(SyncSampleBox),
    Stco(ChunkOffsetBox),
    Co64(ChunkOffset64Box),
    Ctts(CompositionOffsetBox),

    Unknown(SkipBox),
}

impl Reader for BoxContent {
    fn read<'a, T: Read + Seek>(reader: &mut BoxReader<T>, header: BoxHeader) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let result = match header.name {
            BoxType::FileType => BoxContent::Ftyp(FtypBox::read(reader, header)?),
            BoxType::Movie => BoxContent::Moov(MoovBox::read(reader, header)?),
            BoxType::MovieHeader => BoxContent::Mvhd(MvhdBox::read(reader, header)?),
            BoxType::Track => BoxContent::Trak(TrackBox::read(reader, header)?),
            BoxType::MediaData => BoxContent::Mdat(MediaDataBox::read(reader, header)?),
            BoxType::UserData => BoxContent::Udta(UserDataBox::read(reader, header)?),
            BoxType::Wide => BoxContent::Wide(WideBox::read(reader, header)?),
            BoxType::TrackHeader => BoxContent::Tkhd(TrackHeaderBox::read(reader, header)?),
            BoxType::Edit => BoxContent::Edts(EditBox::read(reader, header)?),
            BoxType::Media => BoxContent::Mdia(MediaBox::read(reader, header)?),
            BoxType::MediaHeader => BoxContent::Mdhd(MediaHeaderBox::read(reader, header)?),
            BoxType::Handler => BoxContent::Hdlr(HandlerBox::read(reader, header)?),
            BoxType::MediaInfo => BoxContent::Minf(MediaInfoBox::read(reader, header)?),
            BoxType::VideoInfo => BoxContent::Vmhd(VideoInfoBox::read(reader, header)?),
            BoxType::SoundInfo => BoxContent::Smhd(SoundInfoBox::read(reader, header)?),
            BoxType::DataInfo => BoxContent::Dinf(DataInfoBox::read(reader, header)?),
            BoxType::SampleTable => BoxContent::Stbl(SampleTableBox::read(reader, header)?),
            BoxType::VideoSampleDescription => {
                BoxContent::Stsd(VideoSampleDescriptionBox::read(reader, header)?)
            }
            BoxType::TimeToSample => BoxContent::Stts(TimeToSampleBox::read(reader, header)?),
            BoxType::SampleToChunk => BoxContent::Stsc(SampleToChunkBox::read(reader, header)?),
            BoxType::SampleSize => BoxContent::Stsz(SampleSizeBox::read(reader, header)?),
            BoxType::SyncSample => BoxContent::Stss(SyncSampleBox::read(reader, header)?),
            BoxType::ChunkOffset => BoxContent::Stco(ChunkOffsetBox::read(reader, header)?),
            BoxType::ChunkOffset64 => BoxContent::Co64(ChunkOffset64Box::read(reader, header)?),
            BoxType::CompositionOffset => {
                BoxContent::Ctts(CompositionOffsetBox::read(reader, header)?)
            }
            _ => BoxContent::Unknown(SkipBox::read(reader, header)?),
        };
        Ok(result)
    }
}

#[derive(Clone, Debug)]
pub struct BoxElement {
    pub header: BoxHeader,
    pub content: BoxContent,
}

impl Reader for BoxElement {
    fn read<'a, T: Read + Seek>(reader: &mut BoxReader<T>, header: BoxHeader) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let content = BoxContent::read(reader, header)?;
        Ok(Self { header, content })
    }
}

box_types!(
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
    VideoSampleDescription 0x73747364u32, // "stsd"
    TimeToSample 0x73747473u32, // "stts"
    SampleToChunk 0x73747363u32,// "stsc"
    SampleSize  0x7374737Au32,  // "stsz"
    SyncSample  0x73747373u32,  // "stss"
    ChunkOffset 0x7374636Fu32,  // "stco"
    ChunkOffset64 0x636F3634,   // "co64"
    CompositionOffset 0x63747473, // "ctts"
);
