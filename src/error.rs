use std::{fmt, io};

use crate::user_config::error::MordantConfigError;
pub type MordantResult<T> = Result<T, MordantError>;

pub enum MordantError {
    Config(MordantConfigError),
    IO(io::Error),
    Lib,
}

impl From<MordantConfigError> for MordantError {
    fn from(e: MordantConfigError) -> Self {
        return Self::Config(e);
    }
}

impl From<io::Error> for MordantError {
    fn from(e: io::Error) -> Self {
        return Self::IO(e);
    }
}

impl fmt::Display for MordantError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Config(err) => {
                write!(f, "{err}")
            }
            Self::IO(err) => {
                write!(f, "{err}")
            }
            Self::Lib => {
                write!(f, "shouldbe imposslbe")
            }
        }
    }
}
