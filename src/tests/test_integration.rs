#![cfg(test)]

use node::*;
use serialize::*;

#[test]
pub fn test_encode_and_serialize() {

    // encode the string and then convert to binary
    let s = "Hey what's up?\n";
    let dict = create_probability_dictionary(s);
    let tree_tuple = create_tree(&dict);
    let encoded_before = encode_string(&tree_tuple, s);
    let binary = string_to_binary(&encoded_before);

    // convert binary to a string and decode
    let mut encoded_after = binary_to_string(&binary);
    assert_eq!(encoded_before, encoded_after);

    // let hey = decode_string(&tree_tuple, &encoded_after);
    // assert_eq!(s.to_string(), hey);

}
