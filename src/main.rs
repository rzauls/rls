use chrono::{DateTime, Local};
use colored::*;
use std::error::Error;
use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;
use crate::OutputColor::{Green, Yellow};

/**
 * Simple ls clone with windows-friendly coloured formatting
 *
 * TODO: fix alias/symlink issues (~/ etc)
 * TODO: fix directory-first ordering
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

enum OutputColor {
    Green,
    Yellow,
}

fn main() {
    let opt = Options::from_args();
    if let Err(ref e) = run(&opt.path, &opt.all) {
        println!("{}", e);
        process::exit(1);
    }
}

fn print_in_color(input: DirEntry, all: &bool, color: OutputColor, postfix: &str) {
    let file_name_colored: ColoredString;
    let file_name = input.file_name().into_string();
    let mut item = file_name.expect("could not read filename");
    // check if file is dotfile
    if (all == &false) && (item.starts_with(".")) {
        return;
    }

    item += postfix;
    match color {
        Green => { file_name_colored = item.green() }
        Yellow => { file_name_colored = item.yellow() }
    }
    print_with_metadata(input, file_name_colored);
}

fn print_dir(input: DirEntry, all: &bool) {
    print_in_color(input, all, Green, "/")
}

fn print_item(input: DirEntry, all: &bool) {
    print_in_color(input, all, Yellow, "")
}

fn print_with_metadata(input: DirEntry, file_name: ColoredString) {
    let metadata = input.metadata().
        expect("file has no metadata");

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
    let modified: DateTime<Local> = DateTime::from(metadata.modified().
        expect("unknown modification date"));

    println!(
        // left padding 15 chars to fit large filesizes
        "{:>15} {} {}",
        size.bright_blue(),
        modified.format("%_d %b %Y %H:%M").to_string(),
        file_name
    );
}

fn run(dir: &PathBuf, all: &bool) -> Result<(), Box<dyn Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            // TODO: doing a single loop loses directories-first ordering
            let item = entry?;
            match item.path().is_dir() {
                true => { print_dir(item, all) }
                false => { print_item(item, all) }
            }
        }
    }
    Ok(())
}
