use std::{fmt, io};

use crate::user_config::error::MordantConfigError;
pub type MordantResult<T> = Result<T, MordantError>;

pub enum MordantError {
    Config(MordantConfigError),
    TOML(toml::de::Error),
    IO(io::Error),
}

impl From<toml::de::Error> for MordantError {
    fn from(value: toml::de::Error) -> Self {
        return Self::TOML(value);
    }
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
            Self::TOML(err) => {
                write!(f, "{err}")
            }
        }
    }
}
