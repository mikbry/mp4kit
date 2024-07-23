
#[macro_export]
macro_rules! box_definitions {
    ($($(#[$attr:meta])* $boxenum:ident $boxtype:expr),*,) => {
        use std::fmt;
        use $crate::FourCC;
    
        #[derive(Clone, Copy, PartialEq, Eq)]
        pub enum BoxType {
            $($(#[$attr])* $boxenum),*,
            Root(u32),
            Unknown(u32),
        }

        impl From<u32> for BoxType {
            fn from(t: u32) -> BoxType {
                use self::BoxType::*;
                match t {
                    $($(#[$attr])* $boxtype => $boxenum),*,
                    _ => Unknown(t),
                }
            }
        }

        impl From<BoxType> for u32 {
            fn from(b: BoxType) -> u32 {
                use self::BoxType::*;
                match b {
                    $($(#[$attr])* $boxenum => $boxtype),*,
                    Unknown(t) => t,
                    Root(t) => t,
                }
            }
        }

        impl fmt::Debug for BoxType {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let fourcc: FourCC = From::from(*self);
                fourcc.fmt(f)
            }
        }

        impl fmt::Display for BoxType {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let fourcc: FourCC = From::from(*self);
                fourcc.fmt(f)
            }
        }
    }
}
