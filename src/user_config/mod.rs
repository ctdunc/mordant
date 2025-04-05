use error::MordantConfigResult;
use highlighter_options::{LanguageSrc, MordantHighlighterConfig, QuerySrc};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};
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
}

impl MordantConfig {
    pub fn get_highlight_configurations(
        mut self,
        names: HashSet<String>,
    ) -> MordantConfigResult<BTreeMap<String, HighlightConfiguration>> {
        let mut configs: BTreeMap<String, HighlightConfiguration> = BTreeMap::default();
        for name in names.iter() {
            let hl_config: MordantConfigResult<HighlightConfiguration> =
                match self.languages.remove(name).take() {
                    Some(lang) => lang,
                    _ => MordantHighlighterConfig {
                        name: name.clone(),
                        language: LanguageSrc::BuiltIn,
                        highlights_query: QuerySrc::BuiltIn,
                        locals_query: Some(QuerySrc::BuiltIn),
                        injections_query: Some(QuerySrc::BuiltIn),
                    },
                }
                .try_into();

            match hl_config {
                Ok(hl) => {
                    let _ = configs.insert(name.clone(), hl);
                }
                Err(err) => {
                    eprintln!("{:?}", err);
                }
            };
        }
        Ok(configs)
    }
}
