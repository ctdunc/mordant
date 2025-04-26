use crate::user_config::error::MordantConfigError;

use super::error::MordantConfigResult;
use super::highlighter_options::expand_path;
use std::path::PathBuf;
use tree_sitter::{Language, Query};
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

pub fn get_language_from_source_file(
    path: &PathBuf,
    symbol_name: &str,
) -> MordantConfigResult<Language> {
    use libloading::{Library, Symbol};
    match unsafe { Library::new(&expand_path(path.clone())?) } {
        Ok(library) => {
            let language = unsafe {
                let language_fn: Symbol<unsafe extern "C" fn() -> *const ()> =
                    match library.get(symbol_name.as_bytes()) {
                        Ok(lib) => lib,
                        Err(err) => {
                            return Err(MordantConfigError::LanguageSource {
                                symbol_name: symbol_name.into(),
                                error: err,
                            });
                        }
                    };
                tree_sitter_language::LanguageFn::from_raw(*language_fn)
            };
            std::mem::forget(library);
            return Ok(language.into());
        }
        Err(err) => {
            return Err(MordantConfigError::LanguageSource {
                symbol_name: symbol_name.into(),
                error: err,
            });
        }
    }
}

pub fn get_builtin_language(name: &str) -> MordantConfigResult<Language> {
    match name {
        #[cfg(feature = "javascript")]
        "javascript" => {
            use tree_sitter_javascript;
            return Ok(tree_sitter_javascript::LANGUAGE.into());
        }
        #[cfg(feature = "python")]
        "python" => {
            use tree_sitter_python;
            return Ok(tree_sitter_python::LANGUAGE.into());
        }
        #[cfg(feature = "lua")]
        "lua" => {
            use tree_sitter_lua;
            return Ok(tree_sitter_lua::LANGUAGE.into());
        }
        #[cfg(feature = "json")]
        "json" => {
            use tree_sitter_json;
            return Ok(tree_sitter_json::LANGUAGE.into());
        }
        #[cfg(feature = "typescript")]
        "typescript" => {
            use tree_sitter_typescript;
            return Ok(tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into());
        }
        #[cfg(feature = "html")]
        "html" => {
            use tree_sitter_html;
            return Ok(tree_sitter_html::LANGUAGE.into());
        }
        #[cfg(feature = "css")]
        "css" => {
            use tree_sitter_css;
            return Ok(tree_sitter_css::LANGUAGE.into());
        }
        #[cfg(feature = "sql")]
        "sql" => {
            use tree_sitter_sequel;
            return Ok(tree_sitter_sequel::LANGUAGE.into());
        }
        #[cfg(feature = "rust")]
        "rust" => {
            use tree_sitter_rust;
            return Ok(tree_sitter_rust::LANGUAGE.into());
        }
        _ => {
            eprintln!(
                "{} is not a builtin language! Either recompile with this feature flag enabled, or configure this language in mordant.toml!",
                name
            );
            return Err(MordantConfigError::NotSupported(name.into()));
        }
    }
}
pub fn get_builtin_highlights(name: &str) -> MordantConfigResult<String> {
    match name {
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
        #[cfg(feature = "lua")]
        "lua" => {
            use tree_sitter_lua;
            return Ok(tree_sitter_lua::HIGHLIGHTS_QUERY.into());
        }
        #[cfg(feature = "json")]
        "json" => {
            use tree_sitter_json;
            return Ok(tree_sitter_json::HIGHLIGHTS_QUERY.into());
        }
        #[cfg(feature = "typescript")]
        "typescript" => {
            use tree_sitter_typescript;
            return Ok(tree_sitter_typescript::HIGHLIGHTS_QUERY.into());
        }
        #[cfg(feature = "html")]
        "html" => {
            use tree_sitter_html;
            return Ok(tree_sitter_html::HIGHLIGHTS_QUERY.into());
        }
        #[cfg(feature = "css")]
        "css" => {
            use tree_sitter_css;
            return Ok(tree_sitter_css::HIGHLIGHTS_QUERY.into());
        }
        #[cfg(feature = "rust")]
        "rust" => {
            use tree_sitter_rust;
            return Ok(tree_sitter_rust::HIGHLIGHTS_QUERY.into());
        }
        #[cfg(feature = "sql")]
        "sql" => {
            use tree_sitter_sequel;
            return Ok(tree_sitter_sequel::HIGHLIGHTS_QUERY.into());
        }
        _ => {
            return Err(MordantConfigError::NotSupported(name.into()));
        }
    }
}
pub fn get_builtin_locals(name: &str) -> MordantConfigResult<String> {
    match name {
        #[cfg(feature = "javascript")]
        "javascript" => {
            return Ok(tree_sitter_javascript::LOCALS_QUERY.into());
        }
        #[cfg(feature = "lua")]
        "lua" => {
            return Ok(tree_sitter_lua::LOCALS_QUERY.into());
        }
        #[cfg(feature = "typescript")]
        "typescript" => {
            return Ok(tree_sitter_typescript::LOCALS_QUERY.into());
        }
        _ => return Err(MordantConfigError::NotSupported(name.into())),
    }
}
pub fn strip_nonstandard_predicates(mut query: Query) -> Query {
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
