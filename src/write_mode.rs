use core::fmt;

use crate::interfaces::WriteModeTrait;

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_write_mode() {
        let write_mode = WriteMode::Insert;
        assert_eq!(write_mode, WriteMode::Insert);

        let write_mode = write_mode.toggle_mode();
        assert_eq!(write_mode, WriteMode::OverWrite);

        let write_mode = write_mode.toggle_mode();
        assert_eq!(write_mode, WriteMode::Insert);
    }
}
