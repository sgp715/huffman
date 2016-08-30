#![allow(unstable)]
use std::env;
use std::process::exit;

extern crate huffman;
use huffman::utils::*;
use huffman::node::*;
use huffman::serialize::*;

use std::fs::{self, DirBuilder};

extern crate rustc_serialize;
use rustc_serialize::json;

extern crate bincode;
use bincode::rustc_serialize::{encode, decode};

use std::collections::HashMap;

fn compress(file_name: &str) {


    // read the string from the file
    let s = utils::read_file_to_string(file_name);

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
    let key = utils::create_probability_dictionary(&s);
    let encoded_key = encode(&key, bincode::SizeLimit::Infinite).unwrap();
    serialize::write_binary(&(path.to_string() + "/key"), &encoded_key);

    // create the graph tuple
    let tuple = node::create_tree(&key);

    // iterate through each letter and encode it
    let mut encoded: String = "".to_string();
    for c in s.chars(){

        let current = &node::encode(&tuple, &c.to_string());
        encoded = encoded + &current;

    }

    // convert string to encoded_data
    let encoded_data = serialize::string_to_binary(&encoded);

    // write binary string
    serialize::write_binary(&(path.to_string() + "/data"), &encoded_data);

}


fn decompress(s: &str) {

    // 

    /*
    let encoded_key = serialize::read_binary(&(path.to_string() + "/key"));
    let decoded_key: HashMap<String, f32> = decode(&b_key).unwrap();

    let encoded_data = serialize::read_binary(&(path.to_string() + "/data"));
    */

}


fn main() {


    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: [OPTIONS] [FILE]");
        exit(1);
    }

    let option = &args[1];
    let file_name = &args[2];

    // get the extentsion
    /*
    let mut ext = "".to_string();
    let mut before = true;
    for c in file_name.chars() {
        if c != '.' && before {
            continue;
        }

        if c == '.' {
            before = false;
        }

        ext = ext + &c.to_string();
    }



    // get option



    if ext.to_string() == ".txt" {
        compress(&file_name);
    } else if ext.to_string() == ".rip" {
        decompress(&file_name);
    }
    */


    if option == "-c" {
        compress(file_name);
    } else if option == "-d" {
        decompress(file_name);
    } else {
        panic!("Not a valid option");
    }

}
