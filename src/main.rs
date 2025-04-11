mod file_highlighter;
mod user_config;

use clap::Parser;
use file_highlighter::MarkdownFile;
use std::fs::read_to_string;
use user_config::MordantConfig;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    file: Vec<String>,
    #[arg(long, short, default_value_t = String::from("./mordant.toml"))]
    config_file: String,
}

fn main() {
    let args = Args::parse();
    let config: MordantConfig = toml::from_str(
        read_to_string(args.config_file)
            .unwrap_or_default()
            .as_str(),
    )
    .unwrap();

    let highlighters = config.get_highlight_configurations().unwrap();

    let file_contents = read_to_string(args.file.get(0).unwrap()).unwrap();
    let mut file = MarkdownFile::new(file_contents, &highlighters);

    file.format();

    println!("{}", &file.contents());
}
