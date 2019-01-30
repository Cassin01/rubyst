use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_code() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("file not found");

    let mut texts = String::new();
    for line in BufReader::new(file).lines() {
        texts.push_str(&line.unwrap().to_owned());
        /*
        texts.push_str(&line.unwrap().trim_right().to_owned());
        */
    }
    texts
}