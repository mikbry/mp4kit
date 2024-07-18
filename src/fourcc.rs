
#[derive(Clone, Copy)]
pub struct FourCC;

impl FourCC {
    #[inline]
    pub fn from_str(fourcc: &str) -> u32 {
        let bytes = fourcc.as_bytes();
        u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
    
}

#[macro_export]
macro_rules! fourcc {
    ($str:expr) => {
        format!("{:#x}u32", $crate::FourCC::from_str($str))
        // $crate::FourCC::from_str($str)
    }
}
