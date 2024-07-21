use std::{fmt, io::{Read, Seek}};

use crate::{BoxParser, Error};

#[derive(Clone, Copy)]
pub struct Matrix {
    pub a: i32, // 16.16 fix point
    pub b: i32, // 16.16 fix point
    pub u: i32, // 2.30 fix point
    pub c: i32, // 16.16 fix point
    pub d: i32, // 16.16 fix point
    pub v: i32, // 2.30 fix point
    pub x: i32, // 16.16 fix point
    pub y: i32, // 16.16 fix point
    pub w: i32, // 2.30 fix point
}

impl Matrix {
    pub fn read<'a, T: Read + Seek>(parser: &mut BoxParser<T>) -> Result<Self, Error> {
        Ok(Self {
            a: parser.read_i32()?,
            b: parser.read_i32()?,
            u: parser.read_i32()?,
            c: parser.read_i32()?,
            d: parser.read_i32()?,
            v: parser.read_i32()?,
            x: parser.read_i32()?,
            y: parser.read_i32()?,
            w: parser.read_i32()?,

        })
    }

    fn  write_matrix(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ {:}, {:}, {:}, {:}, {:}, {:}, {:}, {:}, {:} }}",
            self.a, self.b, self.u, self.c, self.d, self.v, self.x, self.y, self.w
        )
}
}
impl Default for Matrix {
    fn default() -> Self {
        Self {
            // unity matrix according to ISO/IEC 14496-12:2005(E)
            a: 0x00010000,
            b: 0,
            u: 0,
            c: 0,
            d: 0x00010000,
            v: 0,
            x: 0,
            y: 0,
            w: 0x40000000,
        }
    }
}



impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.write_matrix(f)
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_matrix(f)
    }
}

