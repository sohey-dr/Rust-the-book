use clap::Clap;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Opts {
    #[clap(short, long)]
    verbose: bool,
    #[clap(short, long)]
    input: String,
}

fn main() {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.unwrap();
            println!("{}", line);
        }
    } else {
        println!("No formula file specified");
    }
}