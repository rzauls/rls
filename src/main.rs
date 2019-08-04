use chrono::{DateTime, Local};
use colored::*;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

/**
 * TODO: print dirs first, then files
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
            if metadata.len() == 0 {
                size = "".to_string();
            } else if metadata.len() <= 1024 {
                size = bytefmt::format_to(metadata.len(), bytefmt::Unit::B);
            } else if metadata.len() <= 1048576 {
                size = bytefmt::format_to(metadata.len(), bytefmt::Unit::KB);
            } else if metadata.len() <= 1073741824 {
                size = bytefmt::format_to(metadata.len(), bytefmt::Unit::MB);
            } else {
                size = bytefmt::format_to(metadata.len(), bytefmt::Unit::GB);
            }
            // let size = bytefmt::format_to(metadata.len(), bytefmt::Unit::KB);
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
