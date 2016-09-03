#![cfg(test)]

use utils::*;
use std::collections::BTreeMap;

#[test]
/*
pub fn test_read_file_to_string() {

	let s = "My name is sebastian.\n";
	let t = read_file_to_string("test.txt");
	assert_eq!(t, s);

}
*/


#[test]
pub fn test_create_string_dictionary() {

	let d0 = create_string_dictionary("hello");
	let mut dict0: BTreeMap<String, i32> = BTreeMap::new();
	dict0.insert("h".to_string(), 1);
	dict0.insert("e".to_string(), 1);
	dict0.insert("l".to_string(), 2);
	dict0.insert("o".to_string(), 1);
	assert_eq!(d0, dict0);

	let d1 = create_string_dictionary("aa bbbb cccccc ??? .");
	let mut dict1: BTreeMap<String, i32> = BTreeMap::new();
	dict1.insert("a".to_string(), 2);
	dict1.insert("b".to_string(), 4);
	dict1.insert("c".to_string(), 6);
	dict1.insert(" ".to_string(), 4);
	dict1.insert("?".to_string(), 3);
	dict1.insert(".".to_string(), 1);
	assert_eq!(d1, dict1);

}


#[test]
pub fn test_create_probability_dictionary() {

	let d0 = create_probability_dictionary("hello");
	let mut dict0: BTreeMap<String, f32> = BTreeMap::new();
	dict0.insert("h".to_string(), 0.2);
	dict0.insert("e".to_string(), 0.2);
	dict0.insert("l".to_string(), 0.4);
	dict0.insert("o".to_string(), 0.2);
	assert_eq!(d0, dict0);

	let d1 = create_probability_dictionary("aa bbbb cccccc ?? ..");
	let mut dict1: BTreeMap<String, f32> = BTreeMap::new();
	dict1.insert("a".to_string(), 0.1);
	dict1.insert("b".to_string(), 0.2);
	dict1.insert("c".to_string(), 0.3);
	dict1.insert(" ".to_string(), 0.2);
	dict1.insert("?".to_string(), 0.1);
	dict1.insert(".".to_string(), 0.1);
	assert_eq!(d1, dict1);

}

#[test]
pub fn test_minimum() {

    let mut dict: BTreeMap<String, f32> = BTreeMap::new();
    dict.insert("h".to_string(), 0.1);
	dict.insert("e".to_string(), 0.2);
	dict.insert("l".to_string(), 0.3);
	dict.insert("o".to_string(), 0.4);

	let actual = minimum(&dict);
	let expected = "h";

	assert_eq!(actual, expected);

}

#[test]
pub fn test_reverse_string() {

    let s = "Hello, world!";
    let rs = reverse_string(s);
    let el ="!dlrow ,olleH";
    assert_eq!(el, rs);

}

/*
#[test]
pub fn test_fuse(){

    let mut dict: BTreeMap<String, f32> = BTreeMap::new();
    dict.insert("h".to_string(), 0.1);
	let mut actual = fuse(&dict);
	assert_eq!(actual, dict);


	dict.insert("e".to_string(), 0.2);
	dict.insert("l".to_string(), 0.3);
	dict.insert("o".to_string(), 0.4);
	actual = fuse(&dict);

	let mut expected: BTreeMap<String, f32> = BTreeMap::new();
	expected.insert("he".to_string(), 0.3);
	expected.insert("l".to_string(), 0.3);
	expected.insert("o".to_string(), 0.4);
    assert_eq!(actual, expected);


    actual = fuse(&actual);
    expected = BTreeMap::new();
	expected.insert("lhe".to_string(), 0.6);
	expected.insert("o".to_string(), 0.4);
	assert_eq!(actual, expected);

}*/
