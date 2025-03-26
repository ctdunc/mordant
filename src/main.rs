use clap::Parser;
use std::fs::read_to_string;
use tree_sitter::{self, QueryCapture, StreamingIterator};
use tree_sitter_facade::create_highlights;
use tree_sitter_highlight::{HighlightEvent, Highlighter};
use tree_sitter_md;
mod tree_sitter_facade;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    file: String,
}
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
fn main() {
    let args = Args::parse();
    let file_contents = read_to_string(args.file).unwrap();

    let mut md_parser = tree_sitter::Parser::new();

    let _ = md_parser.set_language(&tree_sitter_md::LANGUAGE.into());
    let tree = md_parser.parse(&file_contents, None).unwrap();
    let mut cursor = tree_sitter::QueryCursor::new();
    let code_block_query = tree_sitter::Query::new(
        &tree_sitter_md::LANGUAGE.into(),
        "(fenced_code_block
          (info_string
            (language) @injection.language)
              (code_fence_content) @injection.content)"
            .into(),
    )
    .unwrap();
    let mut new_contents = file_contents.clone();
    let mut code_blocks = cursor.matches(
        &code_block_query,
        tree.root_node(),
        file_contents.as_bytes(),
    );
    let mut highlighter = Highlighter::new();
    let mut offset: usize = 0;
    let highlight_configs = create_highlights();
    while let Some(query_match) = code_blocks.next() {
        // TODO are captures always in order?
        let captures: Vec<&QueryCapture> = query_match.captures.iter().collect();
        let lang_cap = captures.get(0).unwrap();
        let lang_start = lang_cap.node.start_byte();
        let lang_end = lang_cap.node.end_byte();
        let lang = &file_contents[lang_start..lang_end];
        if let Some(hl_cfg) = highlight_configs.get(lang) {
            let code_cap = captures.get(1).unwrap();
            let code_start = code_cap.node.start_byte();
            let code_end = code_cap.node.end_byte();
            let code_block = &file_contents[code_start..code_end];
            let highlights = highlighter.highlight(&hl_cfg, code_block.as_bytes(), None, |lang| {
                // TODO how do i deal with this?
                //println!("getting lang: {}", lang);
                return highlight_configs.get(lang);
            });

            let mut formatted: String = "<pre><code>\n".into();
            for event in highlights.unwrap() {
                match event.unwrap() {
                    HighlightEvent::Source { start, end } => {
                        formatted += format!("{}", &code_block[start..end]).as_str();
                    }
                    HighlightEvent::HighlightStart(s) => {
                        let classname = format!("code-{}", HIGHLIGHT_NAMES[s.0]);
                        formatted += format!("<span class=\"{}\">", classname).as_str();
                    }
                    HighlightEvent::HighlightEnd => {
                        formatted += format!("</span>").as_str();
                    }
                }
            }
            formatted += "\n</code></pre>\n";

            let range = (code_start - (lang.len() + 6) + offset)..(code_end + 3 + offset);
            offset += formatted.len() - range.len();
            new_contents.replace_range(range, &formatted.as_str());
        }
    }
    println!("{}", new_contents);
}
