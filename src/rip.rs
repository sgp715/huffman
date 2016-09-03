#![allow(unstable)]
use std::env;
use std::process::exit;

extern crate huffman;
use huffman::utils::*;
use huffman::node::{create_tree, encode_string, decode_string};
use huffman::serialize::{write_binary, read_binary, string_to_binary, binary_to_string};

use std::fs::{DirBuilder};

use std::io::prelude::*;
use std::fs::File;
use std::io::{self, BufReader};

extern crate serde;
extern crate bincode;
use bincode::serde::{serialize, deserialize_from};

use std::collections::HashMap;

fn compress(file_name: &str) {

    // read the string from the file
    let s = read_file_to_string(file_name);

    // get filename w/o extension
    let mut filename_no_extension = "".to_string();
    for c in file_name.chars() {
        if c == '.' {
            break;
        }

        filename_no_extension = filename_no_extension + &c.to_string();
    }
    let path = &(filename_no_extension + ".rip");
    DirBuilder::new().recursive(true).create(path).unwrap();

    // encode the key we will use to decode the file
    let key: HashMap<String, f32> = create_probability_dictionary(&s);
    for (letter, prob) in &key {
        println!("letter: {}, prob: {}", *letter, *prob);
    }
    let encoded_key = serialize(&key, bincode::SizeLimit::Infinite).expect("Could not serialize key");
    write_binary(&(path.to_string() + "/key"), &encoded_key);

    // encode the contents of the file
    let tree_tuple = create_tree(&key);
    let encoded = encode_string(&tree_tuple, &s);
    let encoded_data = string_to_binary(&encoded);
    write_binary(&(path.to_string() + "/data"), &encoded_data);

}


fn decompress(path: &str) {

    let file = File::open(&(path.to_string() + "/key")).expect("Could not open file");
    let mut reader = BufReader::new(file);
    let key: HashMap<String, f32> = deserialize_from(&mut reader, bincode::SizeLimit::Infinite).expect("Could not deserialize key");
    for (letter, prob) in &key {
        println!("letter: {}, prob: {}", *letter, *prob);
    }

    let tree_tuple = create_tree(&key);
    let binary_data = read_binary(&(path.to_string() + "/data"));
    let encoded_data = binary_to_string(&binary_data);
    let data = decode_string(&tree_tuple, &encoded_data);

    println!("data: {}", data);

    //write_

}


fn main() {

    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: [OPTIONS] [FILE]");
        exit(1);
    }

    let option = &args[1];
    let file_name = &args[2];

    if option == "-c" {
        compress(file_name);
    } else if option == "-d" {
        decompress(file_name);
    } else {
        panic!("Not a valid option");
    }

}
