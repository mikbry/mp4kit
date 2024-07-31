use std::{fs::File, io::{BufReader, Read, Seek}};

use mp4kit::{Error, Mp4};

fn parse_mp4<'a, T: Read + Seek>(reader: &'a mut T) -> Result<(), Error> {
    let mp4 = Mp4::parse(reader)?;
    println!("Video: {mp4:?}");

    Ok(())
}
fn main() {
    println!("Parse samples/video1.mp4");

    let file: File = File::open("samples/video1.mp4").unwrap();
    let mut reader = BufReader::with_capacity(5120000,file);

    if let Err(error) = parse_mp4(&mut reader) {
        println!("{:?}", error.to_string());
    }
}
