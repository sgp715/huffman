use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;


pub fn read_file_to_string(file_name: &str) -> String {
	//! read a string from a file

	// read the file
	let mut file = match File::open(file_name) {
		Ok(file) => file,
		Err(..) => panic!("Could not find file"),
	};

	// create a string to put the file contents in
	let mut s = String::new();
	file.read_to_string(&mut s);

	s
}


pub fn create_string_dictionary(s: &str) -> HashMap<String, i32> {
	//! create a dictionary that holds the count of each letter

	// if the key is in the dictionary then add one to the value
	let mut dict: HashMap<String, i32> = HashMap::new();

	// iterate through all of the characters in the string
	for c in s.chars(){
	
		let s = c.to_string();

		// insert if it does not already exist
		let current_value: i32;
		{
			current_value = match dict.get(&s) {
				Some(n) => *n,
				None => 0,
			};
		}

		let updated_value = current_value + 1;
		
		{
		    dict.insert(s, updated_value);
		}

	}

	dict

}


pub fn create_probability_dictionary(s: &str) -> HashMap<String, f32> {
	//! create a dictionary that holds the probability of each letter in a document

	// create the count dictionary 
	let dict: HashMap<String, i32> = create_string_dictionary(&s);
	
	let mut probability_dict: HashMap<String, f32> = HashMap::new();

	let mut total_letters: f32 = 0.0;

	// count the total number of letters
	for count in dict.values() {
		total_letters = total_letters + *count as f32;
	}

	// calculate the probability of each letter and place it in a dictionary
	for (letter, count) in dict {
		let probability: f32 = count as f32 / total_letters;
		probability_dict.insert(letter, probability);
	}

	probability_dict
}


pub fn minimum(dict: &HashMap<String, f32>) -> String {
    //! return the minimum value from HashMap
    
    // iterate through everything and return the minimum letter
    let mut minimum_prob: f32 = 1.0f32 / 0.0f32;
    let mut minimum_letter: String = " ".to_string();
    for (letter, prob) in dict {
    
        if *prob < minimum_prob {
            minimum_prob = *prob;
            minimum_letter = letter.clone().to_string();
        }
    }
    
    minimum_letter
}


/*
pub fn fuse(dict: &HashMap<String, f32>) -> HashMap<String, f32> {
    //! takes in a hash map and adds the probability of the minimum two
    //! and returns a new hash map with the combined values
    
    let mut new_dict: HashMap<String, f32> = dict.clone();
    
    if new_dict.len() <= 1 {
        return new_dict;
    }
    
    // remove two letters
    let first_letter = minimum(&new_dict);
    let mut new_prob: f32;
    {
        let first_prob = new_dict.get(&first_letter).unwrap();
        new_prob =  *first_prob;
    }
    new_dict.remove(&first_letter);
    
    let second_letter = minimum(&new_dict);
    {
        let second_prob = new_dict.get(&second_letter).unwrap();
        new_prob = new_prob + *second_prob;
    }
    new_dict.remove(&second_letter);
    let new_letter = first_letter + &second_letter;
    
    // create the new entry
    new_dict.insert(new_letter, new_prob);
    
    new_dict

}*/



