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

    //let new_file_name = filename_no_extension + ".rip";
    let path = &(filename_no_extension + ".rip");
    DirBuilder::new().recursive(true).create(path).unwrap();

    // encode the key we will use to traverse
    let key: HashMap<String, f32> = create_probability_dictionary(&s);
    let encoded_key = match serialize(&key, bincode::SizeLimit::Infinite){
        Ok(ec) => ec,
        Err(e) => panic!("Could not encode"),
    };
    write_binary(&(path.to_string() + "/key"), &encoded_key);

    // create the graph tuple
    let tuple = create_tree(&key);

    // iterate through each letter and encode it
    let mut encoded: String = "".to_string();
    for c in s.chars(){

        let current = &encode_string(&tuple, &c.to_string());
        encoded = encoded + &current;

    }

    // convert string to encoded_data
    let encoded_data = string_to_binary(&encoded);

    // write binary string
    write_binary(&(path.to_string() + "/data"), &encoded_data);

}


fn decompress(path: &str) {

    let file = File::open(&(path.to_string() + "/key")).expect("Could not open file");
    let mut reader = BufReader::new(file);

    //let decoded_key: HashMap<String, f32> = deserialize(&binary_key[..]).unwrap();
    let decoded_key: HashMap<String, f32> = deserialize_from(&mut reader, bincode::SizeLimit::Infinite).expect("Could not deserialize key");

    /*
    for (letter, prob) in decoded_key.iter() {

        println!("letter: {}, prob {}", letter, prob);
    }
    */

    /*
    let tree_tuple = create_tree(&decoded_key);

    let binary_data = read_binary(&(path.to_string() + "/data"));
    let decoded_data: HashMap<String, f32> = match decode(&binary_data) {
        Ok(bd) => bd,
        Err(e) => panic!("Could not decode data"),
    };
    */

    // now pass the data along with the key into the decode function

    // create the new file with the decoded data


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
