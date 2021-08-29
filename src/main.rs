use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("Searches for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("File to search")
                .takes_value(true)
                .required(false),
        )
        .get_matches();
    let pattern = args
        .value_of("pattern")
        .expect("Please enter a string pattern you search for");
    let re = Regex::new(pattern).unwrap();

    if let Some(input) = args.value_of("input") {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re);
    } else {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    }
}

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line in reader.lines() {
        let line = line.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            _ => (),
        }
    }
}
