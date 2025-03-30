use serde::{Deserialize, Serialize};
use shellexpand;
use std::collections::{BTreeMap, HashMap};
use std::{env::VarError, fs::read_to_string, path::PathBuf};
use tree_sitter::{Language, Query};
use tree_sitter_highlight::HighlightConfiguration;

pub const HIGHLIGHT_NAMES: [&str; 90] = [
    "variable",
    "variable.builtin",
    "variable.parameter",
    "variable.parameter.builtin",
    "variable.member",
    "constant",
    "constant.builtin",
    "constant.macro",
    "module",
    "module.builtin",
    "label",
    "string",
    "string.documentation",
    "string.regexp",
    "string.escape",
    "string.special",
    "string.special.symbol",
    "string.special.path",
    "string.special.url",
    "character",
    "character.special",
    "boolean",
    "number",
    "number.float",
    "type",
    "type.builtin",
    "type.definition",
    "attribute",
    "attribute.builtin",
    "property",
    "function",
    "function.builtin",
    "function.call",
    "function.macro",
    "function.method",
    "function.method.call",
    "constructor",
    "operator",
    "keyword",
    "keyword.coroutine",
    "keyword.function",
    "keyword.operator",
    "keyword.import",
    "keyword.type",
    "keyword.modifier",
    "keyword.repeat",
    "keyword.return",
    "keyword.debug",
    "keyword.exception",
    "keyword.conditional",
    "keyword.conditional.ternary",
    "keyword.directive",
    "keyword.directive.define",
    "punctuation.delimiter",
    "punctuation.bracket",
    "punctuation.special",
    "comment",
    "comment.documentation",
    "comment.error",
    "comment.warning",
    "comment.todo",
    "comment.note",
    "markup.strong",
    "markup.italic",
    "markup.strikethrough",
    "markup.underline",
    "markup.heading",
    "markup.heading.1",
    "markup.heading.2",
    "markup.heading.3",
    "markup.heading.4",
    "markup.heading.5",
    "markup.heading.6",
    "markup.quote",
    "markup.math",
    "markup.link",
    "markup.link.label",
    "markup.link.url",
    "markup.raw",
    "markup.raw.block",
    "markup.list",
    "markup.list.checked",
    "markup.list.unchecked",
    "diff.plus",
    "diff.minus",
    "diff.delta",
    "tag",
    "tag.builtin",
    "tag.attribute",
    "tag.delimiter",
];

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_nvim_treesitter")]
    nvim_treesitter_location: PathBuf,
    #[serde(default = "BTreeMap::default")]
    languages: BTreeMap<String, HighlighterOptions>,
}
fn default_nvim_treesitter() -> PathBuf {
    // this is a valid path, and we don't try to read it here, so this should never panic.
    // TODO: test on W*ndows.
    return expand_path(PathBuf::from("~/.local/share/nvim/lazy/nvim-treesitter")).unwrap();
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
        //eprintln!("{:?}", self.languages);
        for name in names.iter() {
            match self.languages.remove(name).take() {
                Some(lang) => {
                    //eprintln!("inserting lang {:?}", lang);
                    configs.insert(name.clone(), lang.into_highlight_config()?);
                }
                None => {
                    let maybe_default_lang =
                        HighlighterOptions::from_name(&name, &self.nvim_treesitter_location);
                    match maybe_default_lang {
                        Ok(lang) => match lang.into_highlight_config() {
                            Ok(lang_config) => {
                                configs.insert(name.clone(), lang_config);
                            }
                            Err(err) => {
                                //eprintln!("{:?}, {:?}", err, lang);
                                // eprintln!("{} had a bad default configuration! skipping...", name);
                                // eprintln!("{:?}", lang);
                            }
                        },
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
}
#[derive(Serialize, Deserialize, Debug)]
pub struct HighlighterOptions {
    pub name: String,
    pub language: Option<LanguageSource>,
    pub highlights_query: Option<QuerySource>,
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
fn strip_nonstandard_predicates(mut query: Query) -> Query {
    for pattern_index in 0..query.pattern_count() {
        let general_predicates = query.general_predicates(pattern_index);
        if general_predicates.len() > 0 {
            // TODO enable some other captures here that might be useful
            /*
            eprintln!(
                "pattern due to nonstandard predicates {:?}",
                general_predicates
            );*/
            query.disable_pattern(pattern_index);
        }
    }
    return query;
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
        let mut language_path: PathBuf = nvim_treesitter_dir.clone();
        language_path.extend(["parser", name.into()].iter());
        language_path.set_extension("so");
        language_path = expand_path(language_path)?;
        let language = LanguageSource::SourceFile(SourceFileType {
            path: language_path,
            symbol_name: None,
        })
        .into();

        let mut query_directory: PathBuf = nvim_treesitter_dir.clone();
        query_directory.extend(["queries", name].iter());

        let mut highlights_path = query_directory.clone();
        highlights_path.push("highlights.scm");
        let highlights_query = QuerySource::Path(highlights_path).into();

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
            Some(query) => {
                match query {
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
            &None => match self.name.as_str() {
                #[cfg(feature = "javascript")]
                "javascript" => {
                    use tree_sitter_javascript;
                    return Ok(tree_sitter_javascript::HIGHLIGHT_QUERY.into());
                }
                #[cfg(feature = "python")]
                "python" => {
                    use tree_sitter_python;
                    return Ok(tree_sitter_python::HIGHLIGHTS_QUERY.into());
                }
                _ => {
                    eprintln!(
                        "Crate inclusion for language {} has not been implemented",
                        &self.name
                    );
                    return Err(HighlighterOptionError::NotImplementedError);
                }
            },
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
            Some(language) => match language {
                LanguageSource::SourceFile(source) => {
                    use libloading::{Library, Symbol};
                    let symbol_name = source
                        .symbol_name
                        .clone()
                        .unwrap_or(format!("tree_sitter_{}", self.name));
                    let library = match unsafe { Library::new(&expand_path(source.path.clone())?) }
                    {
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
            },
            None => match self.name.as_str() {
                "javascript" => {
                    use tree_sitter_javascript;
                    return Ok(tree_sitter_javascript::LANGUAGE.into());
                }
                _ => {
                    eprintln!(
                        "Crate inclusion for language {} has not been implemented",
                        &self.name
                    );
                    return Err(HighlighterOptionError::NotImplementedError);
                }
            },
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
        // check the highlights query for non-confirm
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
