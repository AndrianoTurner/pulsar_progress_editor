use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};
mod error;
mod models;
use error::*;
use models::*;
mod binary_utils;
use binary_utils::*;
use clap::{Parser, Subcommand};
use serde_json;

#[derive(Parser, Debug)]
#[command(name = "Pulsar : LC Progress Editor")]
#[command(author = "Andriano Turner")]
#[command(version = "0.1")]
#[command(about = "Dumps binary data to json for easy editing", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand, Debug)]
enum Commands {
    /// Dumps binary to json
    Dump { input: String, out: String },
    /// Writes json to binary
    Write { input: String, out: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Dump { input, out } => {
            let data = read_progress_binary(input)?;
            let json = serde_json::to_string_pretty(&data)?;
            let mut out_file = File::create(&out)?;
            out_file.write_all(json.as_bytes())?;
            println!("Successfully wrote json to: {}", out)
        }
        Commands::Write { input, out } => {
            let mut in_file = File::open(input)?;
            let mut buf = String::new();
            in_file.read_to_string(&mut buf)?;
            let data: ProgressData = serde_json::from_str(&buf)?;
            let out_file = File::create(&out)?;
            write_data_to_file(out_file, data)?;
            println!("Successfully wrote binary data to: {}", out)
        }
    }
    Ok(())
}
