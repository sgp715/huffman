use std::env;
use std::process::exit;

extern crate huffman;
use huffman::utils::*;
use huffman::node::*;
use huffman::serialize::*;


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

    // create the graph tuple
    let tuple = node::create_tree(&s);


    // iterate through each letter and encode it
    let mut encoded: String = "".to_string();
    for c in s.chars(){

        let current = &node::encode(&tuple, &c.to_string());
        encoded = encoded + &current;

    }


    // convert string to binary
    let binary = serialize::string_to_binary(&encoded);


    // write binary string
    serialize::write_binary(&(filename_no_extension + ".rip"), &binary);

}


fn decompress(s: &str) {

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
