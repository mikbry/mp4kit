use std::fs::File;

use mp4kit::{fourcc, parse, Box, Error, FourCC, FtypBox, MoovBox};

fn parse_mp4(file_name: &str) -> Result<(), Error> {
    let mut file = File::open(file_name).unwrap();
    let mut parser = parse(&mut file);

    /* for parsed_box in parser {
        println!("box: {:?}", parsed_box)
    } */

    let ftyp = FtypBox::parse(&mut parser)?;
    println!("ftyp box: {:?}", ftyp);
    let moov = MoovBox::parse(&mut parser)?;
    println!("moov box: {:?}", moov);

    Ok(())
}
fn main() {
    println!("Hello, world!");
    let v = FourCC::from_str("ftyp");
    println!("{} {}", v, fourcc!("ftyp"));

    if let Err(error) = parse_mp4("samples/video1.mp4") {
        println!("{:?}", error.to_string());
    }
}
