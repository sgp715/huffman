use std::env;
use std::process::exit;

extern crate huffman;
use huffman::utils;


fn main() {


    let mut args = env::args();
    
    if args.len() != 2 {
        println!("Usage: example filename");
        exit(2);
    }
    
    let file_name = args.nth(1).expect("no filename given");

    let s = utils::utils::read_file_to_string(&file_name);
    
    
}

