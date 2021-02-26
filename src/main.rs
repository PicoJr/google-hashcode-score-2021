#[macro_use]
extern crate clap;
extern crate anyhow;

use crate::parser::parse_input;
use log::info;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::str::FromStr;

mod cli;
mod data;
mod parser;

fn main() -> anyhow::Result<()> {
    // cf https://crates.io/crates/env_logger
    env_logger::init();

    // parse command line arguments
    let matches = cli::get_app().get_matches();
    let input_files = matches.values_of("input");
    if let Some(input_files) = input_files {
        for (i, input_file_path) in input_files.enumerate() {
            let path = PathBuf::from_str(input_file_path)?;
            let input_content = read_to_string(path)?;
            info!("parsing {}", input_file_path);
            let input_data = parse_input(&input_content)?;
        }
    }
    Ok(())
}
