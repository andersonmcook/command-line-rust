use clap::Parser;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug, Parser)]
#[command(about, author, long_about = None, version)]
pub struct Args {
    #[arg(default_values_t = ["-".to_string()], help = "Input files")]
    files: Vec<String>,

    #[arg(
        conflicts_with = "number_nonblank",
        help = "Number lines",
        long,
        short = 'n'
    )]
    number: bool,

    #[arg(help = "Number non-blank lines", short = 'b', long)]
    number_nonblank: bool,
}

pub fn run(args: Args) {
    args.files.iter().for_each(|file| match open(&file) {
        Ok(_) => println!("Opened {}", file),
        Err(err) => eprintln!("Failed to open {}: {}", file, err),
    });
}

pub fn get_args() -> Args {
    Args::parse()
}

fn open(filename: &str) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
