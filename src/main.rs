use chrono::{DateTime, Local};
use colored::*;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

/**
 * Simple ls clone with windows-friendly coloured formatting
 *
 * TODO: reformat the obvious code duplication away
 * TODO: fix alias/symlinm issues (~/ etc)
 * TODO: figure out line wrapping (cut filenames after x chars?)
 */
#[derive(StructOpt, Debug)]
struct Options {
    /// Show all files (including dotfiles)
    #[structopt(short = "a", long = "all")]
    all: bool,
    /// Output file
    #[structopt(default_value = ".", parse(from_os_str))]
    path: PathBuf,
}

fn main() {
    let opt = Options::from_args();
    if let Err(ref e) = run(&opt.path, &opt.all) {
        println!("{}", e);
        process::exit(1);
    }
}

fn _type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

fn print_dir(input: std::fs::DirEntry, all: &bool) {
    let file_name_colored: colored::ColoredString;
    let file_name = input.file_name().into_string();
    // check if file is dotfile
    match file_name {
        Ok(mut item) => {
            if (all == &false) && (item.starts_with(".")) {
                return;
            }

            item += "/";
            file_name_colored = item.green();

            match input.metadata() {
                Ok(metadata) => {
                    // format size
                    let size: String;
                    match metadata.len() {
                        0 => size = "...".to_string(),
                        1..=1024 => size = bytefmt::format_to(metadata.len(), bytefmt::Unit::B),
                        1025..=1048567 => {
                            size = bytefmt::format_to(metadata.len(), bytefmt::Unit::KB)
                        }
                        1048576..=1073741824 => {
                            size = bytefmt::format_to(metadata.len(), bytefmt::Unit::MB)
                        }
                        _ => size = bytefmt::format_to(metadata.len(), bytefmt::Unit::GB),
                    };
                    let modified: DateTime<Local> = match metadata.modified() {
                        Ok(time) => DateTime::from(time),
                        Err(e) => panic!("PANIC: {:?}", e),
                    };

                    println!(
                        // left padding 15 chars to fit large filesizes
                        "{:>15} {} {}",
                        size.bright_blue(),
                        modified.format("%_d %b %Y %H:%M").to_string(),
                        file_name_colored
                    );
                }
                Err(e) => panic!("PANIC: {:?}", e),
            }
        }

        Err(e) => panic!("PANIC: {:?}", e),
    }
}

fn print_item(input: std::fs::DirEntry, all: &bool) {
    let file_name_colored: colored::ColoredString;
    let file_name = input.file_name().into_string();
    // check if file is dotfile
    match file_name {
        Ok(item) => {
            if (all == &false) && (item.starts_with(".")) {
                return;
            }

            file_name_colored = item.yellow();

            match input.metadata() {
                Ok(metadata) => {
                    // format size
                    let size: String;
                    match metadata.len() {
                        0 => size = "...".to_string(),
                        1..=1024 => size = bytefmt::format_to(metadata.len(), bytefmt::Unit::B),
                        1025..=1048567 => {
                            size = bytefmt::format_to(metadata.len(), bytefmt::Unit::KB)
                        }
                        1048576..=1073741824 => {
                            size = bytefmt::format_to(metadata.len(), bytefmt::Unit::MB)
                        }
                        _ => size = bytefmt::format_to(metadata.len(), bytefmt::Unit::GB),
                    };
                    let modified: DateTime<Local> = match metadata.modified() {
                        Ok(time) => DateTime::from(time),
                        Err(e) => panic!("PANIC: {:?}", e),
                    };

                    println!(
                        // left padding 15 chars to fit large filesizes
                        "{:>15} {} {}",
                        size.bright_blue(),
                        modified.format("%_d %b %Y %H:%M").to_string(),
                        file_name_colored
                    );
                }
                Err(e) => panic!("PANIC: {:?}", e),
            }
        }

        Err(e) => panic!("PANIC: {:?}", e),
    }
}

fn run(dir: &PathBuf, all: &bool) -> Result<(), Box<dyn Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let item = entry?;
            if item.path().is_dir() {
                print_dir(item, all);
            }
        }
        for entry in fs::read_dir(dir)? {
            let item = entry?;
            if !item.path().is_dir() {
                print_item(item, all);
            }
        }
    }
    Ok(())
}
