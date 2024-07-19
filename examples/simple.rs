use std::{fs::File, io::{BufReader, Read}};

use mp4kit::{fourcc, parse, BoxParser, Error, FourCC, Mp4};

fn parse_mp4<'a, T: Read>(reader: &'a mut T) -> Result<(), Error> {
    let mut parser: BoxParser<T> = parse(reader);

    let mp4 = Mp4::parse(parser)?;
    println!("Mp4 = {mp4:?}");

    Ok(())
}
fn main() {
    println!("Hello, world!");
    let v = FourCC::from_str("ftyp");
    println!("{} {}", v, fourcc!("ftyp"));

    let file: File = File::open("samples/video1.mp4").unwrap();
    let mut reader = BufReader::new(file);

    if let Err(error) = parse_mp4(&mut reader) {
        println!("{:?}", error.to_string());
    }
}
