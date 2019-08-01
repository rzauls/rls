use chrono::{DateTime, Local};
use colored::*;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

/**
 * TODO: print dirs first, then files
 * TODO: convert bytes into more readable format (mb, gb, etc...)
 */
#[derive(StructOpt, Debug)]
struct Opt {
    /// Output file
    #[structopt(default_value = ".", parse(from_os_str))]
    path: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
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
            let mut file_name_colored: colored::ColoredString;
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
            let size = metadata.len().to_string() + " B";
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
