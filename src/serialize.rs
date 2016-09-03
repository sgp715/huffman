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
            let left_over = &s[front..];
            let mut string = "".to_string();
            string = string + left_over;
            let add = 8 - left_over.len();
            for i in 0..add {
                string = string + "0";
            }


            binary_vector.push(calculate_binary(&string));

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

    let mut file = File::create(filename).expect("Could not create file.");

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
    reader.read_to_end(&mut vec);

    vec

}


pub fn byte_to_string(byte: u8, fill_front: bool) -> String {
    // takes in a byte and outputs the string value

    let mut s: String = "".to_string();

    let mut num = byte;
    let mut old_num = byte;
    loop {

        //println!("before: {}", num);
        // divide by two and store the result
        num = num / 2;
        //println!("after: {}", num);

        // find the remainder
        let remainder = old_num - (num * 2);
        //println!("remainder: {}", remainder);

        let letter = match remainder {

            0 => "0",
            1 => "1",
            _ => panic!("remainder should either be 0 or 1")

        };
        s = letter.to_string() + &s;

        // if the letter is 1 or 0 we are done
        if num == 0 {
            break;
        }

        old_num = num;

    }

    let add = 8 - s.len();
    if fill_front == true {
        for i in 0..add {
            s = "0".to_string() + &s;
        }
    } else {
        for i in 0..add {
            s = s + "0";
        }
    }

    s

}

pub fn binary_to_string(binary: &Vec<u8>) -> String {
    //! take the binary array and translate it into a string


    let mut s: String = "".to_string();

    let mut byte_num = 1;
    let length = binary.len();

    for byte in binary {

        /*
        if byte_num == length {
            let string_byte = byte_to_string(*byte, false);
            s = s + &string_byte;
            break;
        }
        */

        let string_byte = byte_to_string(*byte, true);
        s = s + &string_byte;

        byte_num += 1;

    }

    s

}
