mod error;
pub(crate) mod file_highlighter;
#[cfg(test)]
mod tests;
mod user_config;
use clap::Parser;
use error::MordantResult;
use file_highlighter::MarkdownFile;
use std::{fs::read_to_string, process::ExitCode};
use user_config::MordantConfig;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    file: Vec<String>,
    #[arg(long, short, default_value_t = String::from("./mordant.toml"))]
    config_file: String,
}

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("{}", e);
        return ExitCode::FAILURE;
    }

    return ExitCode::SUCCESS;
}

fn run() -> MordantResult<()> {
    let args = Args::parse();
    let config: MordantConfig = toml::from_str(
        read_to_string(args.config_file)
            .unwrap_or_default()
            .as_str(),
    )
    .unwrap();

    let highlighters = config.get_highlight_configurations()?;

    let file_contents = read_to_string(args.file.get(0).unwrap())?;
    let mut file = MarkdownFile::new(file_contents, &highlighters);

    file.format();

    print!("{}", &file.contents());
    return Ok(());
}
