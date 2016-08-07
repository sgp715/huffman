#![cfg(test)]


use utils::*;
use serialize::*;
use std::collections::HashMap;


#[test]
pub fn test_read_file_to_string() {

	let s = "my name is sebastian.\n";
	let t = read_file_to_string("test.txt");
	assert_eq!(t, s);

}


#[test]
pub fn test_create_string_dictionary() {

	let d0 = create_string_dictionary("hello");
	let mut dict0: HashMap<String, i32> = HashMap::new();
	dict0.insert("h".to_string(), 1);
	dict0.insert("e".to_string(), 1);
	dict0.insert("l".to_string(), 2);
	dict0.insert("o".to_string(), 1);
	assert_eq!(d0, dict0);

	let d1 = create_string_dictionary("aa bbbb cccccc ??? .");
	let mut dict1: HashMap<String, i32> = HashMap::new();
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
	let mut dict0: HashMap<String, f32> = HashMap::new();
	dict0.insert("h".to_string(), 0.2);
	dict0.insert("e".to_string(), 0.2);
	dict0.insert("l".to_string(), 0.4);
	dict0.insert("o".to_string(), 0.2);
	assert_eq!(d0, dict0);

	let d1 = create_probability_dictionary("aa bbbb cccccc ?? ..");
	let mut dict1: HashMap<String, f32> = HashMap::new();
	dict1.insert("a".to_string(), 0.1);
	dict1.insert("b".to_string(), 0.2);
	dict1.insert("c".to_string(), 0.3);
	dict1.insert(" ".to_string(), 0.2);
	dict1.insert("?".to_string(), 0.1);
	dict1.insert(".".to_string(), 0.1);
	assert_eq!(d1, dict1);

}


pub use petgraph::Graph;
#[test]
pub fn test_create_graph(){
    
    let mut gr = Graph::new();
    let a = gr.add_node(0.);
    let b = gr.add_node(0.);
    let c = gr.add_node(0.);
    gr.add_edge(a, b, 3.);
    gr.add_edge(b, c, 2.);
    gr.add_edge(c, b, 1.);    

}

#[test]
pub fn test_add(){

    let mut gr = Graph::<Option<String>, bool>::default();
    let a = gr.add_node(None);
    let b = gr.add_node(Some("m".to_string()));
    let c = gr.add_node(Some("e".to_string()));
    gr.add_edge(a, b, true);
    gr.add_edge(a, c, true);

}


#[test]
pub fn test_minimum(){

    let mut dict: HashMap<String, f32> = HashMap::new();
    dict.insert("h".to_string(), 0.1);
	dict.insert("e".to_string(), 0.2);
	dict.insert("l".to_string(), 0.3);
	dict.insert("o".to_string(), 0.4);
	
	let actual = minimum(&dict);
	let expected = "h";
	
	assert_eq!(actual, expected);
	
}


#[test]
pub fn test_fuse(){

    let mut dict: HashMap<String, f32> = HashMap::new();
    dict.insert("h".to_string(), 0.1); 
	let mut actual = fuse(&dict);
	assert_eq!(actual, dict);
	
	
	dict.insert("e".to_string(), 0.2);
	dict.insert("l".to_string(), 0.3);
	dict.insert("o".to_string(), 0.4);
	actual = fuse(&dict);
	
	let mut expected: HashMap<String, f32> = HashMap::new();
	expected.insert("he".to_string(), 0.3); 
	expected.insert("l".to_string(), 0.3);
	expected.insert("o".to_string(), 0.4);
    assert_eq!(actual, expected);

}


#[test]
pub fn test_calculate_binary(){

    let mut s = "0";
    let mut expected = 0;
    let mut actual = calculate_binary(s);
    assert_eq!(actual, expected);
    
    s = "1";
    expected = 1;
    actual = calculate_binary(&s);
    assert_eq!(actual, expected);
    
    s = "01";
    expected = 1;
    actual = calculate_binary(s);
    assert_eq!(actual, expected);
    
    s = "10";
    expected = 2;
    actual = calculate_binary(s);
    assert_eq!(actual, expected);
    
    s = "11";
    expected = 3;
    actual = calculate_binary(s);
    assert_eq!(actual, expected);

    let s = "00000000";
    expected = 0;
    actual = calculate_binary(s);
    assert_eq!(actual, expected);
    
    let s = "11111111";
    expected = 255;
    actual = calculate_binary(s);
    assert_eq!(actual, expected);
    

}

#[test]
pub fn test_string_to_binary(){

    let mut s = "0";
    let mut expected: Vec<u8> = vec![0b0];
    let mut actual = string_to_binary(s);
    assert_eq!(actual, expected);
    
    let s = "01";
    let expected: Vec<u8> = vec![0b01];
    let actual = string_to_binary(s);
    assert_eq!(actual, expected);
    
    let s = "10";
    let expected: Vec<u8> = vec![0b10];
    let actual = string_to_binary(s);
    assert_eq!(actual, expected);
    
    let s = "11";
    let expected: Vec<u8> = vec![0b11];
    let actual = string_to_binary(s);
    assert_eq!(actual, expected);
    
    let s = "00000000";
    let expected: Vec<u8> = vec![0b0];
    let actual = string_to_binary(s);
    assert_eq!(actual, expected);

    let s = "11111111";
    let expected: Vec<u8> = vec![0b11111111];
    let actual = string_to_binary(s);
    assert_eq!(actual, expected);
    
    let s = "111111111";
    let expected: Vec<u8> = vec![0b11111111, 0b1];
    let actual = string_to_binary(s);
    assert_eq!(actual, expected);
    
    let s = "1111111101";
    let expected: Vec<u8> = vec![0b11111111, 0b1];
    let actual = string_to_binary(s);
    assert_eq!(actual, expected);
    
    let s = "1111111110";
    let expected: Vec<u8> = vec![0b11111111, 0b10];
    let actual = string_to_binary(s);
    assert_eq!(actual, expected);
    
    let s = "111111111111111111111111";
    let expected: Vec<u8> = vec![0b11111111, 0b11111111, 0b11111111];
    let actual = string_to_binary(s);
    assert_eq!(actual, expected);

}

#[test]
pub fn test_write_and_read_binary(){

    let expected: Vec<u8> = vec![0b11111111, 0b11111111, 0b11111111];
    write_binary("foo.txt", &expected);
    let actual = read_binary("foo.txt");
    assert_eq!(expected, actual);

}


