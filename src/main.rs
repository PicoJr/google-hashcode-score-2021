#[macro_use]
extern crate clap;
extern crate anyhow;

use crate::parser::{parse_input, parse_output};
use crate::score::compute_score;
use anyhow::bail;
use log::info;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::str::FromStr;

mod cli;
mod data;
mod parser;
mod score;

fn main() -> anyhow::Result<()> {
    // cf https://crates.io/crates/env_logger
    env_logger::init();

    // parse command line arguments
    let matches = cli::get_app().get_matches();
    let input_files = matches.values_of("input").expect("input files compulsory");
    let output_files = matches
        .values_of("output")
        .expect("output files compulsory");
    if input_files.len() != output_files.len() {
        bail!(
            "{} output files provided but expected {}",
            output_files.len(),
            input_files.len()
        );
    }
    let input_output_files = input_files.zip(output_files);
    for (input_file_path, output_file_path) in input_output_files {
        let path = PathBuf::from_str(output_file_path)?;
        let output_content = read_to_string(path)?;
        info!("parsing {}", output_file_path);
        // parsing output first since it is most likely to fail
        let output_data = parse_output(&output_content)?;

        let path = PathBuf::from_str(input_file_path)?;
        let input_content = read_to_string(path)?;
        info!("parsing {}", input_file_path);
        let input_data = parse_input(&input_content)?;

        let score = compute_score(&input_data, &output_data)?;
        println!("{} score: {}", output_file_path, score);
    }
    Ok(())
}
