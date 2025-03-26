use std::collections::HashMap;
use tree_sitter::{self, Language};
use tree_sitter_highlight::HighlightConfiguration;

const HIGHLIGHT_NAMES: [&str; 26] = [
    "attribute",
    "comment",
    "constant",
    "constant.builtin",
    "constructor",
    "embedded",
    "function",
    "function.builtin",
    "keyword",
    "module",
    "number",
    "operator",
    "property",
    "property.builtin",
    "punctuation",
    "punctuation.bracket",
    "punctuation.delimiter",
    "punctuation.special",
    "string",
    "string.special",
    "tag",
    "type",
    "type.builtin",
    "variable",
    "variable.builtin",
    "variable.parameter",
];

pub fn create_highlights() -> HashMap<String, HighlightConfiguration> {
    let mut map: HashMap<String, HighlightConfiguration> = Default::default();

    for lang in ["python", "typescript", "javascript", "lua", "json"] {
        if let Some(hl) = get_lang_by_name(lang) {
            //println!("got lang: {}", lang);
            map.insert(lang.into(), hl);
        }
    }
    return map;
}
fn get_lang_by_name(lang: &str) -> Option<HighlightConfiguration> {
    let hl_lang: Option<Language>;
    let hl_query: Option<&str>;
    let inj_query: Option<&str>;
    match lang {
        "python" => {
            hl_lang = Some(tree_sitter_python::LANGUAGE.into());
            hl_query = Some(tree_sitter_python::HIGHLIGHTS_QUERY);
            inj_query = Some(
                "(call
  (identifier) @name (#eq? @name clientside_callback) 
  (argument_list 
    ((string (string_content) 
	     @injection.content 
	     (#set! injection.include-children)
	     (#set! injection.language \"javascript\")))
	)
)"
                .into(),
            );
        }
        "typescript" => {
            hl_lang = Some(tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into());
            hl_query = Some(tree_sitter_typescript::HIGHLIGHTS_QUERY);
            inj_query = None;
        }
        "javascript" => {
            hl_lang = Some(tree_sitter_javascript::LANGUAGE.into());
            hl_query = Some(tree_sitter_javascript::HIGHLIGHT_QUERY);
            inj_query = None;
        }
        "lua" => {
            hl_lang = Some(tree_sitter_lua::LANGUAGE.into());
            hl_query = Some(tree_sitter_lua::HIGHLIGHTS_QUERY);
            inj_query = None;
        }
        "json" => {
            hl_lang = Some(tree_sitter_json::LANGUAGE.into());
            hl_query = Some(tree_sitter_json::HIGHLIGHTS_QUERY);
            inj_query = None;
        }
        _ => {
            hl_lang = None;
            hl_query = None;
            inj_query = None;
        }
    }
    if let (Some(l), Some(q)) = (hl_lang, hl_query) {
        let i = inj_query.unwrap_or("");
        //println!("{}, {}", lang, i);
        let mut hl_cfg =
            HighlightConfiguration::new(l.into(), lang, q, i.into(), "".into()).unwrap();
        hl_cfg.configure(&HIGHLIGHT_NAMES);
        return Some(hl_cfg.into());
    } else {
        return None;
    }
}
