use std::env;
use std::process::exit;

extern crate huffman;
use huffman::utils::*;
use huffman::node::*;
use huffman::serialize::*;


fn main() {


    let mut args = env::args();
    if args.len() != 2 {
        println!("Usage: example filename");
        exit(2);
    }
    let file_name = args.nth(1).expect("no filename given");

    // read the string from the file
    let s = utils::read_file_to_string(&file_name);
    
    
    // create the graph tuple
    let tuple = node::create_tree(&s);
    
    
    // iterate through each letter and encode it
    let mut encoded: String = "".to_string();
    for c in s.chars(){
        let current = node::encode(&tuple, &c.to_string());
        encoded = encoded + &current;
    }
    
    
    // convert string to binary
    let binary = serialize::string_to_binary(&encoded);
    
    // write binary string
    serialize::write_binary("test.rip", &binary);
    
}

