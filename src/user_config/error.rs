use libloading;
use shellexpand;
use std::{env::VarError, fmt, io, path::PathBuf, result::Result};
pub type MordantConfigResult<T> = Result<T, MordantConfigError>;
impl<T> From<MordantConfigError> for MordantConfigResult<T> {
    fn from(e: MordantConfigError) -> Self {
        return Err(e);
    }
}

#[derive(Debug)]
pub enum MordantConfigError {
    IO(io::Error),
    LanguageSource {
        symbol_name: String,
        error: libloading::Error,
    },
    TreeSitterQuery(tree_sitter::QueryError),
    ShellExpandError(shellexpand::LookupError<VarError>),
    InvalidPath(PathBuf),
    NotSupported(String),
}
impl From<io::Error> for MordantConfigError {
    fn from(e: io::Error) -> Self {
        return Self::IO(e);
    }
}

impl From<tree_sitter::QueryError> for MordantConfigError {
    fn from(e: tree_sitter::QueryError) -> Self {
        return Self::TreeSitterQuery(e);
    }
}

impl From<shellexpand::LookupError<VarError>> for MordantConfigError {
    fn from(e: shellexpand::LookupError<VarError>) -> Self {
        return Self::ShellExpandError(e);
    }
}

impl fmt::Display for MordantConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IO(err) => {
                write!(f, "Encountered an error trying to open a file: {err}")
            }
            Self::LanguageSource { symbol_name, error } => {
                write!(
                    f,
                    "Tried to load symbol name {symbol_name}, but encountered an error!\n {error}"
                )
            }
            Self::TreeSitterQuery(err) => {
                write!(
                    f,
                    "Encountered an error while parsing a tree-sitter query! {err}"
                )
            }
            Self::ShellExpandError(err) => {
                write!(f, "Encountered an error expanding a path! {err}")
            }
            Self::InvalidPath(path) => {
                write!(
                    f,
                    "The provided path was invalid, or contained non-unicode data. {}",
                    path.to_string_lossy()
                )
            }
            Self::NotSupported(lang) => {
                write!(
                    f,
                    "Tried to load unsupported language: {lang}! Provide a path so a .so file, or recompile with support for this language."
                )
            }
        }
    }
}
