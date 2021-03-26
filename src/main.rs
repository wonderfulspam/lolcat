extern crate atty;
extern crate rand_distr;
extern crate structopt;

use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::{fs::File, path::PathBuf};
use structopt::StructOpt;

mod cat;

#[derive(StructOpt, Debug)]
#[structopt(name = "lolcat")]
/// The good ol' lolcat, now with fearless concurrency.
pub struct Control {
    #[structopt(long, default_value = "0.0")]
    /// A seed for your lolcat
    pub seed: f64,
    #[structopt(short, long, default_value = "3.0")]
    /// How much should we spread dem colors?
    pub spread: f64,
    #[structopt(short, long, default_value = "0.1")]
    /// Speed of the colour change
    pub frequency: f64,
    #[structopt(short = "B", long)]
    /// lolcat the background instead
    pub background_mode: bool,
    #[structopt(short = "D", long)]
    /// Simulate slow connection
    pub dialup_mode: bool,
    #[structopt(short, long, default_value = "3.0")]
    /// Speed of the dialup mode
    pub baud: f64,
    /// Input file. Reads from STDIN if missing
    pub filename: Option<PathBuf>,
}

fn main() {
    let mut c = Control::from_args();
    let stdin = io::stdin(); // For lifetime reasons

    let filename = c.filename.clone();
    if let Some(filename) = filename {
        if lolcat_file(&filename, &mut c).is_err() {
            println!("Error opening file {:?}.", filename)
        }
    } else {
        for line in stdin.lock().lines() {
            cat::print_with_lolcat(line.unwrap(), &mut c);
        }
    }
}

fn lolcat_file(filename: &PathBuf, c: &mut Control) -> Result<(), io::Error> {
    let f = File::open(filename)?;
    let file = BufReader::new(&f);
    for line in file.lines() {
        cat::print_with_lolcat(line.unwrap(), c);
    }
    Ok(())
}
