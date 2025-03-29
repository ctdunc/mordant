use crate::HIGHLIGHT_NAMES;
use serde::{Deserialize, Serialize};
use std::{fs::read_to_string, path::PathBuf};
use tree_sitter_highlight::HighlightConfiguration;
#[derive(Serialize, Deserialize)]
pub struct HighlighterOptions {
    pub name: String,
    pub language_path: PathBuf,
    pub query_directory: PathBuf,
    //pub highlight_names: Option<Box<[&'a str]>>,
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
        println!("{:?}", self.query_directory);
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
