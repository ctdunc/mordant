use serde::{Deserialize, Serialize};
use shellexpand;
use std::{fs::read_to_string, path::PathBuf};
use tree_sitter::Language;
use tree_sitter_highlight::HighlightConfiguration;

use super::{
    error::{MordantConfigError, MordantConfigResult},
    treesitter_util::{
        HIGHLIGHT_NAMES, get_builtin_highlights, get_builtin_language, get_builtin_locals,
        get_language_from_source_file, strip_nonstandard_predicates,
    },
};

/// Convenient method for expanding paths, raising errors appropriately if failing.
///
/// # Errors
///
/// This function will return an error if the provided path is not parseable or expandable, or
/// contains invalid unicode data.
pub(super) fn expand_path(path: PathBuf) -> MordantConfigResult<PathBuf> {
    match path.into_os_string().into_string() {
        Ok(path_as_string) => {
            return Ok(PathBuf::from(
                shellexpand::full(path_as_string.as_str())?.to_string(),
            ));
        }
        Err(path_as_string) => return Err(MordantConfigError::InvalidPath(path_as_string.into())),
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum QuerySrc {
    Path { path: PathBuf },
    Text { query: String },
    BuiltIn,
}
impl Default for QuerySrc {
    fn default() -> Self {
        return QuerySrc::BuiltIn;
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum LanguageSrc {
    FromSource {
        path: PathBuf,
        symbol_name: Option<String>,
    },
    BuiltIn,
}

impl Default for LanguageSrc {
    fn default() -> Self {
        return LanguageSrc::BuiltIn;
    }
}
fn _false() -> bool {
    false
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MordantHighlighterConfig {
    pub name: String,
    #[serde(default)]
    pub language: LanguageSrc,
    #[serde(default)]
    pub highlights_query: QuerySrc,
    #[serde(default)]
    pub injections_query: Option<QuerySrc>,
    #[serde(default)]
    pub locals_query: Option<QuerySrc>,
    #[serde(default = "_false")]
    pub html_escape: bool,
}

impl MordantHighlighterConfig {
    /// Returns the language of this [`MordantHighlighterConfig`].
    ///
    /// # Errors
    ///
    /// This function will return an error if
    /// - the provided configuration is incorrect, or attempting to use a non-existent builtin.
    pub fn language(&self) -> MordantConfigResult<Language> {
        match &self.language {
            LanguageSrc::FromSource { path, symbol_name } => {
                return get_language_from_source_file(
                    &path,
                    symbol_name
                        .clone()
                        .unwrap_or(format!("tree_sitter_{}", self.name))
                        .as_str(),
                );
            }
            LanguageSrc::BuiltIn => {
                // try to get the language from preinstalled langs.
                return get_builtin_language(&self.name.as_str());
            }
        }
    }

    /// Returns the highlights query of this [`MordantHighlighterConfig`].
    ///
    /// # Errors
    ///
    /// This function will return an error if the provided configuration points
    /// to a nonexistent or invalid file path.
    pub fn highlights_query(&self) -> MordantConfigResult<String> {
        match &self.highlights_query {
            QuerySrc::Path { path: _path } => {
                let path = expand_path(_path.clone())?;
                return Ok(read_to_string(&path)?);
            }
            QuerySrc::Text { query: text } => {
                return Ok(text.into());
            }
            QuerySrc::BuiltIn => {
                return get_builtin_highlights(&self.name.as_str());
            }
        }
    }

    /// Returns the injections query of this [`MordantHighlighterConfig`].
    /// If we are unable to find one, return an empty string rather than error.
    pub fn injections_query(&self) -> String {
        // TODO make this fail loudly or at least print some kind of error. These queries don't matter
        // as much as the highlighter query.
        if let Some(query) = &self.injections_query {
            match query {
                QuerySrc::Path { path: _path } => {
                    let path = expand_path(_path.clone()).unwrap_or("".into());
                    match read_to_string(&path) {
                        Ok(str) => return str,
                        Err(_) => return "".into(),
                    };
                }
                QuerySrc::Text { query: text } => {
                    return text.into();
                }
                QuerySrc::BuiltIn => {
                    return "".into();
                }
            }
        } else {
            return "".into();
        }
    }
    /// Returns the locals query of this [`MordantHighlighterConfig`].
    /// If we are unable to find one, return an empty string rather than error.
    pub fn locals_query(&self) -> String {
        // TODO ditto [`injections_query`]
        if let Some(query) = &self.locals_query {
            match query {
                QuerySrc::Path { path: _path } => {
                    let path = expand_path(_path.clone()).unwrap_or("".into());
                    match read_to_string(&path) {
                        Ok(str) => return str,
                        Err(_) => return "".into(),
                    };
                }
                QuerySrc::Text { query: text } => {
                    return text.into();
                }
                QuerySrc::BuiltIn => {
                    return get_builtin_locals(&self.name.as_str()).unwrap_or("".into());
                }
            }
        } else {
            return "".into();
        }
    }

    pub fn set_base_dir(mut self, base_dir: &PathBuf) -> Self {
        match self.language {
            LanguageSrc::FromSource { path, symbol_name } => {
                if path.is_relative() {
                    let mut new_path = PathBuf::new();
                    new_path.push(&base_dir);
                    new_path.push(path);
                    self.language = LanguageSrc::FromSource {
                        path: new_path,
                        symbol_name,
                    };
                } else {
                    self.language = LanguageSrc::FromSource { path, symbol_name }
                }
            }
            _ => {}
        }
        match self.highlights_query {
            QuerySrc::Path { path } => {
                if path.is_relative() {
                    let mut new_path = PathBuf::new();
                    new_path.push(&base_dir);
                    new_path.push(path);
                    self.highlights_query = QuerySrc::Path { path: new_path };
                } else {
                    self.highlights_query = QuerySrc::Path { path };
                }
            }
            _ => {}
        }
        if let Some(query) = &self.injections_query {
            match query {
                QuerySrc::Path { path } => {
                    if path.is_relative() {
                        let mut new_path = PathBuf::new();
                        new_path.push(&base_dir);
                        new_path.push(path);
                        self.injections_query = Some(QuerySrc::Path { path: new_path });
                    } else {
                        self.injections_query = Some(QuerySrc::Path {
                            path: path.to_path_buf(),
                        });
                    }
                }
                _ => {}
            }
        }
        if let Some(query) = &self.locals_query {
            match query {
                QuerySrc::Path { path } => {
                    if path.is_relative() {
                        let mut new_path = PathBuf::new();
                        new_path.push(&base_dir);
                        new_path.push(path);
                        self.locals_query = Some(QuerySrc::Path { path: new_path });
                    } else {
                        self.locals_query = Some(QuerySrc::Path {
                            path: path.to_path_buf(),
                        });
                    }
                }
                _ => {}
            }
        }
        return self;
    }
}

impl TryInto<HighlightConfiguration> for MordantHighlighterConfig {
    type Error = MordantConfigError;
    fn try_into(self) -> MordantConfigResult<HighlightConfiguration> {
        let mut highlighter_config = HighlightConfiguration::new(
            self.language()?,
            self.name.clone(),
            self.highlights_query()?.as_str(),
            self.injections_query().as_str(),
            self.locals_query().as_str(),
        )?;
        highlighter_config.configure(&HIGHLIGHT_NAMES);
        highlighter_config.query = strip_nonstandard_predicates(highlighter_config.query);
        return Ok(highlighter_config);
    }
}
