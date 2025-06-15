use core::fmt;

use crate::interfaces::WriteModeTrait;

#[derive(Debug, Clone)]
pub(crate) enum WriteMode {
    OverWrite,
    Insert,
}

impl WriteModeTrait for WriteMode {
    // 書き込みモード変更
    fn toggle_mode(&self) -> WriteMode {
        match self {
            Self::OverWrite => Self::Insert,
            Self::Insert => Self::OverWrite,
        }
    }
}

impl fmt::Display for WriteMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OverWrite => write!(f, " OVR "),
            Self::Insert => write!(f, " INT "),
        }
    }
}
