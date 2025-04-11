use super::user_config::treesitter_util::HIGHLIGHT_NAMES;
use core::slice::Iter;
use std::collections::BTreeMap;
use tree_sitter::{
    InputEdit, Parser, Point, Query, QueryCapture, QueryCursor, StreamingIteratorMut, Tree,
};
use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter};
use tree_sitter_md;
fn get_edit_for_block(block_capture: &QueryCapture, formatted: &String) -> InputEdit {
    let start_byte = block_capture.node.start_byte();
    let old_end_byte = block_capture.node.end_byte();
    let new_end_byte = start_byte + formatted.as_bytes().len();

    let start_position = block_capture.node.start_position();
    let old_end_position = block_capture.node.end_position();

    let new_end_row = start_position.row + formatted.lines().count();
    let new_end_column = formatted.lines().last().unwrap().len();

    let new_end_position = Point {
        row: new_end_row,
        column: new_end_column,
    };
    return InputEdit {
        start_byte,
        old_end_byte,
        new_end_byte,
        start_position,
        old_end_position,
        new_end_position,
    };
}
#[derive(Debug)]
struct CodeBlockCapture<'b> {
    language_capture: &'b QueryCapture<'b>,
    code_block_capture: &'b QueryCapture<'b>,
    full_block_capture: &'b QueryCapture<'b>,
    file_contents: &'b String,
}

impl<'b> CodeBlockCapture<'b> {
    fn get_capture_contents(&self, capture: &QueryCapture) -> &str {
        let start = capture.node.start_byte();
        let end = capture.node.end_byte();
        return &self.file_contents[start..end];
    }
    pub fn new(
        captures: &'b mut Iter<QueryCapture>,
        file_contents: &'b String,
    ) -> CodeBlockCapture<'b> {
        assert!(captures.len() == 3);
        let full_block_capture = captures.next().unwrap();
        let language_capture = captures.next().unwrap();
        let code_block_capture = captures.next().unwrap();
        return CodeBlockCapture {
            language_capture,
            code_block_capture,
            full_block_capture,
            file_contents,
        };
    }
    pub fn language(&self) -> &str {
        return &self.get_capture_contents(self.language_capture);
    }
    pub fn code_contents(&self) -> &str {
        return &self.get_capture_contents(self.code_block_capture);
    }
    pub fn full_capture(&self) -> &QueryCapture {
        return &self.full_block_capture;
    }
}
#[derive(Debug)]
pub struct BlockReplacement {
    pub input_edit: InputEdit,
    pub formatted: String,
}
pub struct MarkdownFile<'a> {
    file_contents: String,
    highlighters: &'a BTreeMap<String, HighlightConfiguration>,
    tree: Tree,
    code_block_query: Query,
}
impl<'a> MarkdownFile<'a> {
    pub fn new(
        file_contents: String,
        highlighters: &'a BTreeMap<String, HighlightConfiguration>,
    ) -> MarkdownFile {
        let mut parser = Parser::new();
        let _ = parser
            .set_language(&tree_sitter_md::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(&file_contents, None).unwrap();

        let code_block_query = tree_sitter::Query::new(
            &tree_sitter_md::LANGUAGE.into(),
            "(fenced_code_block
              (info_string
                (language) @injection.language)
                  (code_fence_content) @injection.content
            ) @block"
                .into(),
        )
        .unwrap();

        return MarkdownFile {
            file_contents,
            highlighters,
            tree,
            code_block_query,
        };
    }

    pub fn parse_markdown_file(&mut self) -> Vec<BlockReplacement> {
        let mut parser = Parser::new();
        let _ = parser
            .set_language(&tree_sitter_md::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(&self.file_contents, None).unwrap();

        let mut cursor = QueryCursor::new();

        let mut code_blocks = cursor.matches(
            &self.code_block_query,
            tree.root_node(),
            self.file_contents.as_bytes(),
        );
        let mut highlighter = Highlighter::new();
        let mut edits: Vec<BlockReplacement> = Vec::new();
        while let Some(query_match) = code_blocks.next_mut() {
            let mut captures = query_match.captures.iter();
            let capture = CodeBlockCapture::new(&mut captures, &self.file_contents);
            let lang = capture.language();
            if let Some(hl_cfg) = self.highlighters.get(lang) {
                let code_block_contents = capture.code_contents();
                let highlights =
                    highlighter.highlight(&hl_cfg, code_block_contents.as_bytes(), None, |lang| {
                        return self.highlighters.get(lang);
                    });
                let mut formatted: String = "<pre><code>".into();

                for event in highlights.unwrap() {
                    match event.unwrap() {
                        HighlightEvent::Source { start, end } => {
                            formatted += format!("{}", &code_block_contents[start..end]).as_str();
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
                formatted += "\n</code></pre>\n\n";

                let input_edit = get_edit_for_block(capture.full_capture(), &formatted);
                edits.push(BlockReplacement {
                    input_edit,
                    formatted,
                });
            };
        }
        return edits;
    }

    pub fn apply_edits(&mut self, edits: Vec<BlockReplacement>) {
        let mut offset = 0;
        for edit in edits.iter() {
            let range =
                (edit.input_edit.start_byte + offset)..(edit.input_edit.old_end_byte + offset);

            offset += edit.formatted.len() - range.len();
            self.file_contents
                .replace_range(range, edit.formatted.as_str())
        }
    }

    pub fn format(&mut self) {
        let edits = self.parse_markdown_file();
        self.apply_edits(edits);
    }

    pub fn contents(&self) -> String {
        return self.file_contents.clone();
    }
}
