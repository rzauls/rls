use crate::OutputColor::{Green, Yellow};
use colored::*;
use std::error::Error;
use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;
use time::{format_description::well_known::Rfc2822, OffsetDateTime};

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

fn run(dir: &PathBuf, all: &bool) -> Result<(), Box<dyn Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            // TODO: doing a single loop loses directories-first ordering
            let item = entry?;
            match item.path().is_dir() {
                true => print_dir(item, all),
                false => print_item(item, all),
            }
        }
    }
    Ok(())
}

fn print_in_color(input: DirEntry, all: &bool, color: OutputColor, decorative_postfix: &str) {
    let file_name = input.file_name().into_string();
    if let Ok(mut item) = file_name {
        // check if file is dotfile
        if (all == &false) && (item.starts_with(".")) {
            return;
        }

        item += decorative_postfix;

        print_with_metadata(
            input,
            match color {
                Green => item.green(),
                Yellow => item.yellow(),
            },
        );
    } else {
        return;
    }
}

fn print_dir(input: DirEntry, all: &bool) {
    print_in_color(input, all, Green, "/")
}

fn print_item(input: DirEntry, all: &bool) {
    print_in_color(input, all, Yellow, "")
}

fn print_with_metadata(input: DirEntry, file_name: ColoredString) {
    let size: String;
    let timestamp: String;
    let timestamp_fallback = "".to_string();

    match input.metadata() {
        Ok(metadata) => {
            match metadata.len() {
                0 => size = "...".to_string(),
                1..=1024 => size = bytefmt::format_to(metadata.len(), bytefmt::Unit::B),
                1025..=1048567 => size = bytefmt::format_to(metadata.len(), bytefmt::Unit::KB),
                1048576..=1073741824 => {
                    size = bytefmt::format_to(metadata.len(), bytefmt::Unit::MB)
                }
                _ => size = bytefmt::format_to(metadata.len(), bytefmt::Unit::GB),
            };

            if let Ok(ts) = metadata.modified() {
                timestamp = OffsetDateTime::from(ts)
                    .format(&Rfc2822)
                    .unwrap_or(timestamp_fallback);
            } else {
                timestamp = timestamp_fallback
            }
        }
        Err(_) => {
            timestamp = timestamp_fallback;
            size = "-".to_string();
        }
    };

    println!(
        // left padding 15 chars to fit large filesizes
        "{:>15} {} {}",
        size.bright_blue(),
        timestamp,
        file_name
    );
}
