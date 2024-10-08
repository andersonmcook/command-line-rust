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

pub fn run() {
    let args = Args::parse();
    let mut line_number = 0;

    args.files.iter().for_each(|file| match open(&file) {
        Ok(buf) => buf.lines().for_each(|line| {
            let line = line.expect("failed to read line");

            // NOTE: can decide this flow once before iterating but requires a Box
            if args.number || (args.number_nonblank && !line.is_empty()) {
                line_number += 1;
                println!("{:>6}\t{}", line_number, line)
            } else {
                println!("{}", line)
            }
        }),
        Err(err) => eprintln!("Failed to open {}: {}", file, err),
    });
}

fn open(filename: &str) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
