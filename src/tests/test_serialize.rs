#![cfg(test)]


use serialize::*;
use std::collections::HashMap;


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

    let s = "1100110001001110";
    let expected: Vec<u8> = vec![0b11001100, 0b01001110];
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

#[test]
pub fn test_byte_to_string() {

    let mut binary: u8= 0b0;
    let mut actual = byte_to_string(binary);
    let mut expected = "00000000".to_string();
    assert_eq!(actual, expected);

    let binary: u8 = 0b01;
    let actual = byte_to_string(binary);
    let expected = "00000001".to_string();
    assert_eq!(actual, expected);

    let binary: u8 = 0b11;
    let actual = byte_to_string(binary);
    let expected = "00000011".to_string();
    assert_eq!(actual, expected);

    let binary: u8 = 0b10;
    let actual = byte_to_string(binary);
    let expected = "00000010".to_string();
    assert_eq!(actual, expected);

    let binary: u8 = 0b101;
    let actual = byte_to_string(binary);
    let expected = "00000101".to_string();
    assert_eq!(actual, expected);

    let binary: u8 = 0b11111111;
    let actual = byte_to_string(binary);
    let expected = "11111111".to_string();
    assert_eq!(actual, expected);

    let binary: u8 = 0b01001110;
    let actual = byte_to_string(binary);
    let expected = "01001110".to_string();
    assert_eq!(actual, expected);

}

#[test]
pub fn test_binary_to_string() {

    let mut binary: Vec<u8> = vec![0b0];
    let mut actual = binary_to_string(&binary);
    let mut expected = "00000000".to_string();
    assert_eq!(actual, expected);

    let binary: Vec<u8> = vec![0b1];
    let actual = binary_to_string(&binary);
    let expected = "00000001".to_string();
    assert_eq!(actual, expected);

    let binary: Vec<u8> = vec![0b10101010];
    let actual = binary_to_string(&binary);
    let expected = "10101010".to_string();
    assert_eq!(actual, expected);

    let binary: Vec<u8> = vec![0b11111111];
    let actual = binary_to_string(&binary);
    let expected = "11111111".to_string();
    assert_eq!(actual, expected);

    let binary: Vec<u8> = vec![0b10000000, 0b11111111, 0b11111111];
    let actual = binary_to_string(&binary);
    let expected = "100000001111111111111111".to_string();
    assert_eq!(actual, expected);

    let binary: Vec<u8> = vec![0b11001100, 0b01001110];
    let actual = binary_to_string(&binary);
    let expected = "1100110001001110".to_string();
    assert_eq!(actual, expected);

}
