use crate::{MarkdownFile, MordantConfig};
use prettydiff::text::{ContextConfig, diff_lines};

#[cfg(test)]
#[macro_export]
macro_rules! doc_test {
    ($($folder:literal,)+ $test_fn:ident) => {
        paste::paste! {$(
            #[cfg(feature = "language_all")]
            #[test]
            fn [<$test_fn _ $folder>]() {
                let mordant_config = include_str!(concat!("./", $folder, "/mordant.toml"));
                let unformatted = include_str!(concat!("./", $folder, "/input.md"));
                let formatted = include_str!(concat!("./", $folder, "/output.md"));
                $test_fn(mordant_config, unformatted, formatted);
            }
        )+}
    }
}
fn pretty_assert_eq(v1: &str, v2: &str) {
    if v1 != v2 {
        let diff = diff_lines(v1, v2);
        panic!(
            "\n{}",
            diff.format_with_context(
                Some(ContextConfig {
                    context_size: 2,
                    skipping_marker: "...",
                }),
                true,
            )
        )
    }
}

// each folder should contain `input.md`, and `output.md`.
fn format_doc(config: &str, unformatted: &str, formatted: &str) {
    let default_config: MordantConfig = toml::from_str(config).unwrap();

    let highlighters = default_config.get_highlight_configurations().unwrap();

    let mut file = MarkdownFile::new(unformatted.into(), &highlighters);

    file.format();

    pretty_assert_eq(&file.contents(), formatted);
}

doc_test!(
    "python",
    "javascript",
    "lua",
    "json",
    "multiple-langs",
    "injected",
    format_doc
);
