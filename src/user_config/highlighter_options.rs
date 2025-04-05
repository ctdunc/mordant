use crate::user_config::error::MordantConfigResult;
use serde::{Deserialize, Serialize};
use shellexpand;
use std::{fs::read_to_string, path::PathBuf};
use tree_sitter::Language;
use tree_sitter_highlight::HighlightConfiguration;

use crate::user_config::{
    error::HighlighterOptionError,
    treesitter_util::{
        HIGHLIGHT_NAMES, get_language_from_source_file, get_precompiled_language,
        strip_nonstandard_predicates,
    },
};

pub fn expand_path(path: PathBuf) -> MordantConfigResult<PathBuf> {
    let path_as_str = path.into_os_string().into_string();
    match path_as_str {
        Ok(p) => match shellexpand::full(p.as_str()) {
            Ok(new_path) => return Ok(PathBuf::from(new_path.to_string())),
            Err(new_path) => return Err(HighlighterOptionError::ShellExpandError(new_path)),
        },
        Err(p) => {
            return Err(HighlighterOptionError::UnhandledError(
                format!("Provided path contained invalid unicode data: {:?}", p).into(),
            ));
        }
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
}

impl MordantHighlighterConfig {
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
                return get_precompiled_language(&self.name.as_str());
            }
        }
    }

    fn get_query_string(query: &QuerySrc) -> MordantConfigResult<String> {
        match &query {
            QuerySrc::Path { path: _path } => {
                let path = expand_path(_path.clone())?;
                match read_to_string(&path) {
                    Ok(str) => return Ok(str),
                    Err(err) => return Err(HighlighterOptionError::IOError(err)),
                };
            }
            QuerySrc::Text { query: text } => {
                return Ok(text.into());
            }
            QuerySrc::BuiltIn => {
                return Err(HighlighterOptionError::NotImplementedError);
            }
        }
    }
    pub fn highlights_query(&self) -> MordantConfigResult<String> {
        return Self::get_query_string(&self.highlights_query);
    }
    pub fn injections_query(&self) -> String {
        return Self::get_query_string(
            &self
                .injections_query
                .as_ref()
                .unwrap_or(&QuerySrc::Text { query: "".into() }),
        )
        .unwrap_or_else(|err| {
            // TODO print error
            return "".into();
        });
    }
    pub fn locals_query(&self) -> String {
        return Self::get_query_string(
            &self
                .locals_query
                .as_ref()
                .unwrap_or(&QuerySrc::Text { query: "".into() }),
        )
        .unwrap_or_else(|err| {
            // TODO print error
            return "".into();
        });
    }
}

impl TryInto<HighlightConfiguration> for MordantHighlighterConfig {
    type Error = HighlighterOptionError;
    fn try_into(self) -> MordantConfigResult<HighlightConfiguration> {
        match HighlightConfiguration::new(
            self.language()?,
            self.name.clone(),
            self.highlights_query()?.as_str(),
            self.injections_query().as_str(),
            self.locals_query().as_str(),
        ) {
            Ok(mut highlighter_config) => {
                highlighter_config.configure(&HIGHLIGHT_NAMES);
                highlighter_config.query = strip_nonstandard_predicates(highlighter_config.query);
                return Ok(highlighter_config);
            }
            Err(err) => {
                return Err(HighlighterOptionError::TreeSitterError(err));
            }
        }
    }
}
