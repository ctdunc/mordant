use libloading;
use shellexpand;
use std::env::VarError;
use std::result::Result;
pub type MordantConfigResult<T> = Result<T, HighlighterOptionError>;

#[derive(Debug)]
pub enum HighlighterOptionError {
    LanguageNotFound(String),
    IOError(std::io::Error),
    LibLoadingError(libloading::Error),
    NvimTreeSitterNotFound,
    NotImplementedError,
    TreeSitterError(tree_sitter::QueryError),
    ShellExpandError(shellexpand::LookupError<VarError>),
    UnhandledError(String),
}
