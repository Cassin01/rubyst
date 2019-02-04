use std::env;
use std::fs::File;
use std::io::prelude::*;

pub fn read_code() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut file = File::open(filename).expect("file not found");
    let mut texts = String::new();

    file.read_to_string(&mut texts)
        .expect("something went wrong reading the file");

    texts
}