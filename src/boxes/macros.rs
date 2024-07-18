#[macro_export]
macro_rules! box_definitions {
    ($($(#[$attr:meta])* $boxenum:ident $boxtype:expr),*,) => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum BoxType {
            $($(#[$attr])* $boxenum),*,
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
                }
            }
        }

    }
}
