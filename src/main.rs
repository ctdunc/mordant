mod error;
pub(crate) mod file_highlighter;
#[cfg(test)]
mod tests;
mod user_config;
use clap::Parser;
use error::MordantResult;
use file_highlighter::MarkdownFile;
use std::path::Path;
use std::{
    fs::{create_dir_all, read_to_string, write},
    process::ExitCode,
};
use user_config::MordantConfig;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(num_args=1..)]
    file: Vec<String>,
    #[arg(long, short, default_value_t = String::from("./mordant.toml"))]
    config_file: String,
    #[arg(long, short, default_value_t = String::from("./mordant.out"))]
    output_dir: String,
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
    let config: MordantConfig = toml::from_str(read_to_string(args.config_file)?.as_str())?;
    let highlighters = config.get_highlight_configurations()?;

    for filename in &args.file {
        // eprintln!("{}", filename);
        let file_contents = read_to_string(filename)?;
        let mut file = MarkdownFile::new(file_contents, &highlighters);
        file.format();

        let out_path = Path::new(&args.output_dir).join(filename.clone());

        create_dir_all(&out_path.parent().unwrap()).unwrap();
        write(out_path, &file.contents())?;
    }

    // print!("{}", &file.contents());
    return Ok(());
}
