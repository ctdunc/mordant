use error::MordantConfigResult;
use highlighter_options::MordantHighlighterConfig;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;
use tree_sitter_highlight::HighlightConfiguration;
pub(crate) mod error;
pub(crate) mod highlighter_options;
pub(crate) mod treesitter_util;

fn default_nvim_treesitter() -> PathBuf {
    // this is a valid path, and we don't try to read it here, so this should never panic.
    // TODO: test on W*ndows.
    return PathBuf::from("~/.local/share/nvim/lazy/nvim-treesitter");
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MordantConfig {
    #[serde(default = "default_nvim_treesitter")]
    nvim_treesitter_location: PathBuf,
    #[serde(default = "BTreeMap::default")]
    languages: BTreeMap<String, MordantHighlighterConfig>,
    base_dir: Option<PathBuf>,
}

impl MordantConfig {
    pub fn get_highlight_configurations(
        self,
    ) -> MordantConfigResult<BTreeMap<String, HighlightConfiguration>> {
        let mut configs: BTreeMap<String, HighlightConfiguration> = BTreeMap::default();
        let base_dir = &self.base_dir.unwrap_or_default();
        for (name, mut lang) in self.languages.into_iter() {
            lang = lang.set_base_dir(&base_dir);
            eprintln!("{:#?}", &lang);
            let _ = configs.insert(name, lang.try_into()?);
        }
        return Ok(configs);
    }

    pub fn with_base_dir(mut self, path: PathBuf) -> Self {
        self.base_dir = Some(path);
        return self;
    }
}
