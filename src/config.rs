use crate::HIGHLIGHT_NAMES;
use serde::{Deserialize, Serialize};
use shellexpand;
use std::collections::{BTreeMap, HashMap};
use std::{env::VarError, fs::read_to_string, path::PathBuf};
use tree_sitter::Language;
use tree_sitter_highlight::HighlightConfiguration;
const NVIM_TREESITTER_LOCATION: [&str; 6] =
    ["~", ".local", "share", "nvim", "lazy", "nvim-treesitter"];
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    languages: BTreeMap<String, HLO>,
}

impl Config {
    /// consumes the highlight configurations.
    pub fn get_highlighter_configurations(
        mut self,
        names: Vec<String>,
    ) -> Result<HashMap<String, HighlightConfiguration>, HighlighterOptionError> {
        let mut configs: HashMap<String, HighlightConfiguration> = HashMap::new().into();
        for name in names.iter() {
            let cnfg: HLO = self.languages.remove(name).take().unwrap_or_else(|| {
                HLO::from_name(&name)
                    .expect(format!("Provided configuration was invalid for {}", name).as_str())
            });
            configs.insert(name.clone(), cnfg.into_highlight_config()?);
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
/// SourceFile second argument should be the name of the symbol in the source file/.
/// It's assumed to be `tree_sitter_{lang}` if not provided.
#[derive(Serialize, Deserialize, Debug)]
pub enum LanguageSource {
    SourceFile(SourceFileType),
    Default(String),
}
#[derive(Serialize, Deserialize, Debug)]
pub struct HLO {
    pub name: String,
    pub language: LanguageSource,
    pub highlights_query: QuerySource,
    pub injections_query: Option<QuerySource>,
    pub locals_query: Option<QuerySource>,
}

fn _expand_path(path: PathBuf) -> PathBuf {
    return PathBuf::from(
        shellexpand::tilde(path.into_os_string().into_string().unwrap().as_str()).as_ref(),
    );
}
impl HLO {
    /// Try to create a new highlighter configuration from just the language name,
    /// using the nvim-treesitter installation location to search for languages.

    pub fn from_name(name: &str) -> Result<HLO, HighlighterOptionError> {
        eprintln!("Trying to get config for {}", name);
        let nvim_treesitter_dir: PathBuf = NVIM_TREESITTER_LOCATION.iter().collect();
        let mut language_path: PathBuf = nvim_treesitter_dir.clone();
        language_path.extend(["parser", name.into()].iter());
        language_path.set_extension("so");
        language_path = _expand_path(language_path);
        let language = LanguageSource::SourceFile(SourceFileType {
            path: language_path,
            symbol_name: None,
        });

        let mut query_directory: PathBuf = nvim_treesitter_dir;
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

    pub fn highlights_query(&self) -> Result<String, HighlighterOptionError> {
        match &self.highlights_query {
            QuerySource::Path(p) => {
                let path = _expand_path(p.clone());
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
    pub fn injections_query(&self) -> String {
        if let Some(q) = &self.injections_query {
            match q {
                QuerySource::Path(p) => {
                    let path = _expand_path(p.clone());
                    let maybe_query = read_to_string(path);
                    match maybe_query {
                        Ok(query) => {
                            return query;
                        }
                        _ => {
                            return "".into();
                        }
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
    pub fn locals_query(&self) -> String {
        if let Some(q) = &self.locals_query {
            match q {
                QuerySource::Path(p) => {
                    let path = _expand_path(p.clone());
                    let maybe_query = read_to_string(path);
                    match maybe_query {
                        Ok(query) => {
                            return query;
                        }
                        _ => {
                            return "".into();
                        }
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
    pub fn language(&self) -> Result<Language, HighlighterOptionError> {
        match &self.language {
            LanguageSource::SourceFile(source) => {
                use libloading::{Library, Symbol};
                let symbol_name = source
                    .symbol_name
                    .clone()
                    .unwrap_or(format!("tree_sitter_{}", self.name));
                let library = match unsafe { Library::new(&_expand_path(source.path.clone())) } {
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

#[derive(Serialize, Deserialize)]
pub struct HighlighterOptions {
    pub name: String,
    pub language_path: PathBuf,
    pub query_directory: PathBuf,
}
impl HighlighterOptions {
    pub fn try_from_name(name: &str) -> Self {
        let nvim_treesitter_dir: PathBuf = [
            "/",
            "home",
            "connor",
            ".local",
            "share",
            "nvim",
            "lazy",
            "nvim-treesitter",
        ]
        .iter()
        .collect();
        // look in nvim-treesitter. It already has all of the relevant configuration.
        let mut language_path: PathBuf = nvim_treesitter_dir.clone();
        language_path.extend(["parser", name].iter());
        language_path.set_extension("so");

        let mut query_directory: PathBuf = nvim_treesitter_dir;
        query_directory.extend(["queries", name].iter());

        return Self {
            name: name.into(),
            language_path,
            query_directory,
        };
    }
    pub fn as_highlight_config(&self) -> Option<HighlightConfiguration> {
        assert!(self.language_path.is_file());
        assert!(self.query_directory.is_dir());
        use libloading::{Library, Symbol};
        let library = unsafe { Library::new(&self.language_path) }.unwrap();
        let language_name = format!("tree_sitter_{}", self.name);
        let language = unsafe {
            let language_fn: Symbol<unsafe extern "C" fn() -> *const ()> =
                library.get(language_name.as_bytes()).unwrap();
            tree_sitter_language::LanguageFn::from_raw(*language_fn)
        };

        std::mem::forget(library);

        let mut highlights_path = self.query_directory.clone();
        highlights_path.push("highlights.scm");
        let highlights = read_to_string(highlights_path).unwrap();

        let mut injections_path = self.query_directory.clone();
        injections_path.push("injections.scm");
        let injections = read_to_string(injections_path).unwrap_or("".into());

        let mut locals_path = self.query_directory.clone();
        locals_path.push("locals.scm");
        let locals = read_to_string(locals_path).unwrap_or("".into());

        let mut highlight_config = HighlightConfiguration::new(
            language.into(),
            self.name.as_str(),
            highlights.as_str(),
            injections.as_str(),
            locals.as_str(),
        )
        .unwrap();
        highlight_config.configure(&HIGHLIGHT_NAMES);
        return Some(highlight_config);
    }
}
