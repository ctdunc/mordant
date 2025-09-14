#[cfg(test)]
mod tests;

mod error;
pub(crate) mod file_highlighter;
mod user_config;
use clap::Parser;
use error::MordantResult;
use file_highlighter::MarkdownFile;
use rayon::prelude::*;
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
    let config_path = Path::new(&args.config_file);
    let mut config: MordantConfig = toml::from_str(read_to_string(config_path)?.as_str())?;
    config = config.with_base_dir(config_path.parent().unwrap().canonicalize().unwrap().into());
    let highlighters = config.get_highlight_configurations()?;

    let _ = &args.file.par_iter().for_each(|f| {
        if let Ok(file_contents) = read_to_string(f) {
            let mut file = MarkdownFile::new(file_contents, &highlighters);
            file.format();

            let out_path = Path::new(&args.output_dir).join(f.clone());
            create_dir_all(&out_path.parent().unwrap()).unwrap();
            if let Ok(_) = write(&out_path, &file.contents()) {
            } else {
                eprintln!("Couldn't write {:?}", &out_path);
            }
        } else {
            eprintln!("Couldn't read {}", f);
        }
    });

    return Ok(());
}
