mod tokenizer;
use tokenizer::tokenize;

use std::fs;

pub fn build(argv: Vec<String>) {
    println!("{:?}", tokenize(&fs::read_to_string(&argv[2]).unwrap()));
}