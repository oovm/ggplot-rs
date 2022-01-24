use super::*;

macro_rules! error_wrap {
    ($t:ty => $name:ident) => {
        impl From<$t> for GGError {
            fn from(e: $t) -> Self {
                Self::$name(e)
            }
        }
    };
    ($($t:ty => $name:ident),+ $(,)?) => (
        $(error_wrap!($t=>$name);)+
    );
}

error_wrap![
    std::io::Error  => IOError,
    std::fmt::Error => FormatError,
];


impl From<Infallible> for GGError {
    fn from(_: Infallible) -> Self {
        Self::Unreachable
    }
}

impl From<()> for GGError {
    fn from(_: ()) -> Self {
        Self::Unreachable
    }
}
