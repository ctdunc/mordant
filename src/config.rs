use crate::HIGHLIGHT_NAMES;
use serde::{Deserialize, Serialize};
use shellexpand;
use std::collections::{BTreeMap, HashMap};
use std::{env::VarError, fs::read_to_string, path::PathBuf};
use tree_sitter::Language;
use tree_sitter_highlight::HighlightConfiguration;
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_nvim_treesitter")]
    nvim_treesitter_location: PathBuf,
    languages: BTreeMap<String, HighlighterOptions>,
}
fn default_nvim_treesitter() -> PathBuf {
    // this is a valid path, and we don't try to read it here, so this should never panic.
    // TODO: test on W*ndows.
    return expand_path(PathBuf::from("~/.local/share/lazy/nvim/nvim-treesitter")).unwrap();
}
impl Config {
    /// Gets [`HighlightConfiguration`]s for the provided languages.
    ///
    /// # Errors
    ///
    /// This function will return an error if the user provided a bad configuration.
    /// If it cannot find a language with the fallback (whether or not the user provided
    /// the nvim-treesitter directory) it will print an error message and skip that language.
    pub fn get_highlight_configurations(
        mut self,
        names: Vec<String>,
    ) -> Result<HashMap<String, HighlightConfiguration>, HighlighterOptionError> {
        let mut configs: HashMap<String, HighlightConfiguration> = HashMap::new().into();
        for name in names.iter() {
            match self.languages.remove(name).take() {
                Some(lang) => {
                    configs.insert(name.clone(), lang.into_highlight_config()?);
                }
                None => {
                    let maybe_default_lang =
                        HighlighterOptions::from_name(&name, &self.nvim_treesitter_location);
                    match maybe_default_lang {
                        Ok(lang) => {
                            if let Ok(lang_config) = lang.into_highlight_config() {
                                configs.insert(name.clone(), lang_config);
                            } else {
                                // TODO improve these error messages
                                eprintln!("{} had a bad default configuration! skipping...", name);
                            }
                        }
                        Err(_) => {
                            eprintln!(
                                "could not construct lang for language {}! skipping...",
                                name,
                            );
                        }
                    }
                }
            }
        }

        return Ok(configs);
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub enum QuerySource {
    Path(PathBuf),
    Text(String),
}
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
impl Into<HighlighterOptionError> for std::io::Error {
    fn into(self) -> HighlighterOptionError {
        return HighlighterOptionError::IOError(self);
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SourceFileType {
    pub path: PathBuf,
    pub symbol_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LanguageSource {
    SourceFile(SourceFileType),
    Default(String),
}
#[derive(Serialize, Deserialize, Debug)]
pub struct HighlighterOptions {
    pub name: String,
    pub language: LanguageSource,
    pub highlights_query: QuerySource,
    pub injections_query: Option<QuerySource>,
    pub locals_query: Option<QuerySource>,
}

fn expand_path(path: PathBuf) -> Result<PathBuf, HighlighterOptionError> {
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
impl HighlighterOptions {
    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn from_name(
        name: &str,
        nvim_treesitter_dir: &PathBuf,
    ) -> Result<HighlighterOptions, HighlighterOptionError> {
        eprintln!("Trying to get config for {}", name);
        let mut language_path: PathBuf = nvim_treesitter_dir.clone();
        language_path.extend(["parser", name.into()].iter());
        language_path.set_extension("so");
        language_path = expand_path(language_path)?;
        let language = LanguageSource::SourceFile(SourceFileType {
            path: language_path,
            symbol_name: None,
        });

        let mut query_directory: PathBuf = nvim_treesitter_dir.clone();
        query_directory.extend(["queries", name].iter());

        let mut highlights_path = query_directory.clone();
        highlights_path.push("highlights.scm");
        let highlights_query = QuerySource::Path(highlights_path);

        let mut injections_path = query_directory.clone();
        injections_path.push("injections.scm");
        let injections_query = QuerySource::Path(injections_path).into();

        let mut locals_path = query_directory.clone();
        locals_path.push("locals.scm");
        let locals_query = QuerySource::Path(locals_path).into();

        return Ok(Self {
            name: name.into(),
            language,
            highlights_query,
            injections_query,
            locals_query,
        });
    }

    /// Returns the highlights query of this [`HighlighterOptions`].
    ///
    /// # Errors
    ///
    /// This function will return an error if the provided query file does not exist.
    pub fn highlights_query(&self) -> Result<String, HighlighterOptionError> {
        match &self.highlights_query {
            QuerySource::Path(p) => {
                let path = expand_path(p.clone())?;
                let maybe_query = read_to_string(path);
                match maybe_query {
                    Ok(query) => {
                        return Ok(query);
                    }
                    Err(err) => {
                        eprintln!("could not find file {:?}", &p);
                        return Err(HighlighterOptionError::IOError(err));
                    }
                }
            }
            QuerySource::Text(query) => {
                // TODO don't clone here if not neccessary.
                return Ok(query.clone());
            }
        }
    }
    /// Returns the injections query of this [`HighlighterOptions`].
    /// If the provided file doesn't exist, this will return an empty string.
    /// This behavior may be changed in the future.
    pub fn injections_query(&self) -> String {
        if let Some(q) = &self.injections_query {
            match q {
                QuerySource::Path(p) => {
                    if let Ok(path) = expand_path(p.clone()) {
                        let maybe_query = read_to_string(path);
                        match maybe_query {
                            Ok(query) => {
                                return query;
                            }
                            _ => {
                                return "".into();
                            }
                        }
                    } else {
                        return "".into();
                    }
                }
                QuerySource::Text(query) => {
                    return query.into();
                }
            }
        } else {
            return "".into();
        }
    }
    /// Returns the locals query of this [`HighlighterOptions`].
    /// If the provided file doesn't exist, this will return an empty string.
    /// This behavior may be changed in the future.
    pub fn locals_query(&self) -> String {
        if let Some(q) = &self.locals_query {
            match q {
                QuerySource::Path(p) => {
                    if let Ok(path) = expand_path(p.clone()) {
                        let maybe_query = read_to_string(path);
                        match maybe_query {
                            Ok(query) => {
                                return query;
                            }
                            _ => {
                                return "".into();
                            }
                        }
                    } else {
                        return "".into();
                    }
                }
                QuerySource::Text(query) => {
                    return query.into();
                }
            }
        } else {
            return "".into();
        }
    }

    /// Returns the language of this [`HighlighterOptions`].
    ///
    /// # Errors
    ///
    /// This function will return an error if
    /// - The specified language file does not exist.
    /// - The specified language file does not contain the provided symbol (defaults to
    /// `format!("tree_sitter_{}, self.name)`.
    /// - The specified language name was not compiled as an included crate.
    pub fn language(&self) -> Result<Language, HighlighterOptionError> {
        match &self.language {
            LanguageSource::SourceFile(source) => {
                use libloading::{Library, Symbol};
                let symbol_name = source
                    .symbol_name
                    .clone()
                    .unwrap_or(format!("tree_sitter_{}", self.name));
                let library = match unsafe { Library::new(&expand_path(source.path.clone())?) } {
                    Ok(lib) => lib,
                    Err(err) => {
                        return Err(HighlighterOptionError::LibLoadingError(err));
                    }
                };

                let language = unsafe {
                    let language_fn: Symbol<unsafe extern "C" fn() -> *const ()> =
                        match library.get(symbol_name.as_bytes()) {
                            Ok(lib) => lib,
                            Err(err) => {
                                return Err(HighlighterOptionError::LibLoadingError(err));
                            }
                        };
                    tree_sitter_language::LanguageFn::from_raw(*language_fn)
                };
                std::mem::forget(library);
                return Ok(language.into());
            }
            LanguageSource::Default(name) => {
                eprintln!(
                    "Crate inclusion for language {} has not been implemented",
                    name
                );
                return Err(HighlighterOptionError::NotImplementedError);
            }
        }
    }

    /// Returns the [`HighlightConfiguration`] specified by these [`HighlighterOptions`].
    ///
    /// # Errors
    ///
    /// This function will return an error if
    /// - Any of the provided configurations do not point to valid files.
    /// - There is an error parsing any provided queries.
    pub fn into_highlight_config(&self) -> Result<HighlightConfiguration, HighlighterOptionError> {
        match HighlightConfiguration::new(
            self.language()?,
            self.name.clone(),
            self.highlights_query()?.as_str(),
            self.injections_query().as_str(),
            self.locals_query().as_str(),
        ) {
            Ok(mut highlighter_config) => {
                highlighter_config.configure(&HIGHLIGHT_NAMES);
                return Ok(highlighter_config);
            }
            Err(err) => {
                return Err(HighlighterOptionError::TreeSitterError(err));
            }
        }
    }
}
