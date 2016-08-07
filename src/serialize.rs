pub fn calculate_binary(s: &str) -> u8 {

    assert!(s.len() <= 8);
    
    let mut i = 0;
    let mut value: u8 = 0;
    let base: u8 = 2;
    for c in s.chars().rev() {
       
        let leading = match c {
            '0' => 0,
            '1' => 1,
             _  => panic!("String should only have 1's and 0's")
        };
        
        value = value + (leading * base.pow(i));
        
        i = i + 1;
    }
    
    value 
    
}


pub fn string_to_binary(s: &str) -> Vec<u8> {
    //! iterate through the string and convert it into a bytes vector

    let mut binary_vector = vec![];
    
    let length = s.len() as f32;
    let num_slices = (length / (8 as f32)).ceil() as i32;

    let mut front = 0;
    let mut back = 8;
    for i in 0..num_slices{
    
        if i == (num_slices - 1) {
        
            // if we are at the end then just do it for everything left
            binary_vector.push(calculate_binary(&s[front..]));
           
        } else {
        
            // otherwise pass in the 8 character string the represents a byte
            let slice = &s[front..back];
            let binary_value = calculate_binary(slice);
            binary_vector.push(binary_value);
            
        }
        
        front = front + 8;
        back = back + 8;

    }
    
    binary_vector    

}


use std::io::prelude::*;
use std::fs::File;
use std::io::{self, BufReader};

pub fn write_binary(filename: &str, data: &Vec<u8>) {
    //! write the buffer vector to a vile

    let mut file = match File::create(filename) {
        Ok(F) => F,
        Err(e) => panic!("Could not create file.")   
    };
    
    file.write(&*data);

}


pub fn read_binary(filename: &str) -> Vec<u8> {
    //! read the buffer into a vector
    
    let mut file = match File::open(filename) {
        Ok(F) => F,
        Err(e) => panic!("Could not open file.")  
    };
    
    let mut reader = BufReader::new(file);
    let mut vec = vec![];
    reader.read_until(b'a', &mut vec);
    
    vec

}
