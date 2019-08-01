use chrono::{DateTime, Local};
use colored::*;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

/**
 * TODO: add type checking (append / and change color for directories)
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
            let file_name = entry
                .file_name()
                .into_string()
                .or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
            let metadata = entry.metadata()?;
            let size = metadata.len().to_string() + " B";
            let modified: DateTime<Local> = DateTime::from(metadata.modified()?);

            println!(
                "{:>10} {} {}",
                size.bright_blue(),
                modified.format("%_d %b %H:%M").to_string().green(),
                file_name.bright_yellow()
            );
        }
    }
    Ok(())
}
