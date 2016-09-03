#![cfg(test)]

use node::*;
use serialize::*;

#[test]
pub fn test_encode_and_serialize() {


    fn test_phrase(s: &str){

        let dict = create_probability_dictionary(s);
        let tree_tuple = create_tree(&dict);

        let mut encoded_before = encode_string(&tree_tuple, s);
        let diff = encoded_before.len() % 8;
        for i in 0..diff {
            encoded_before = encoded_before + "0";
        }
        let binary = string_to_binary(&encoded_before);

        write_binary("foo", &binary);
        let read_binary = read_binary("foo");

        // convert binary to a string and decode
        let mut encoded_after = binary_to_string(&read_binary);
        assert_eq!(encoded_before, encoded_after);
        let hey = decode_string(&tree_tuple, &encoded_after);
        assert_eq!(s.to_string(), hey);
    }
    // encode the string and then convert to binary
    test_phrase("Hey what's up?\n");
    test_phrase("000000000\n");
    //test_phrase("aaaaaaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbbbbb");




}
