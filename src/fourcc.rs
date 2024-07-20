use std::fmt;

use crate::BoxType;


#[derive(Clone, Copy)]
pub struct FourCC {
    pub value: [u8; 4],
}

impl FourCC {
    #[inline]
    pub fn from_str(fourcc: &str) -> u32 {
        let bytes = fourcc.as_bytes();
        u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
    
}

impl From<BoxType> for FourCC {
    fn from(value: BoxType) -> Self {
        let box_num: u32 = Into::into(value);
        From::from(box_num)
    }
}

impl From<[u8; 4]> for FourCC {
    fn from(value: [u8; 4]) -> FourCC {
        FourCC { value }
    }
}

impl From<u32> for FourCC {
    fn from(number: u32) -> FourCC {
        FourCC {
            value: number.to_be_bytes(),
        }
    }
}

impl fmt::Debug for FourCC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match std::str::from_utf8(&self.value) {
            Ok(s) => f.write_str(s),
            Err(_) => self.value.fmt(f),
        }
    }
}

impl fmt::Display for FourCC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(std::str::from_utf8(&self.value).unwrap_or("null"))
    }
}

#[macro_export]
macro_rules! fourcc {
    ($str:expr) => {
        format!("{:#x}u32", $crate::FourCC::from_str($str))
        // $crate::FourCC::from_str($str)
    }
}
