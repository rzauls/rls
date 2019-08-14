use chrono::{DateTime, Local};
use colored::*;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

/**
 * TODO: print dirs first, then files
 * TODO: figure out line wrapping (cut filenames after x chars?)
 * TODO: hide dotfiles by default, show with -a flag
 * TODO: hide owner by default, show with -o flag
 */
#[derive(StructOpt, Debug)]
struct Options {
    /// Output file
    #[structopt(default_value = ".", parse(from_os_str))]
    path: PathBuf,
}

fn main() {
    let opt = Options::from_args();
    if let Err(ref e) = run(&opt.path) {
        println!("{}", e);
        process::exit(1);
    }
}

fn run(dir: &PathBuf) -> Result<(), Box<Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let file_name_colored: colored::ColoredString;
            let mut file_name = entry
                .file_name()
                .into_string()
                .or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
            // check if file is dir
            if path.is_dir() {
                file_name += "/";
                file_name_colored = file_name.green();
            } else {
                file_name_colored = file_name.yellow();
            }
            let metadata = entry.metadata()?;

            // format size
            let size: String;
            match metadata.len() {
                0 => size = "...".to_string(),
                1...1024 => size = bytefmt::format_to(metadata.len(), bytefmt::Unit::B),
                1025...1048567 => size = bytefmt::format_to(metadata.len(), bytefmt::Unit::KB),
                1048576...1073741824 => {
                    size = bytefmt::format_to(metadata.len(), bytefmt::Unit::MB)
                }
                _ => size = bytefmt::format_to(metadata.len(), bytefmt::Unit::GB),
            };
            let modified: DateTime<Local> = DateTime::from(metadata.modified()?);

            println!(
                // left padding 15 chars to fit large filesizes
                "{:>15} {} {}",
                size.bright_blue(),
                modified.format("%_d %b %H:%M").to_string(),
                file_name_colored
            );
        }
    }
    Ok(())
}
