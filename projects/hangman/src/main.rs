use std::fs::File;
use std::io::prelude::*; //  this will allow read_line
use std::io::{self, BufReader};
use std::path::Path;
use std::result;
use std::vec::Vec;

fn main() {
    let mut dictionary: Vec<String> = Vec::new();
    match prepare_dictionary() {
        Ok(l) => dictionary = l,
        Err(err) => panic!(err),
    }

    // for line in dictionary {
    //     println!("{}", line)
    // }

    //  Forever until you die son of a bitch
    loop {}
}

/// I think I've never written so much code for something that basic xD
fn prepare_dictionary() -> Result<Vec<String>, String> {
    let somefile = File::open("dictionary.txt");
    match somefile {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut dict: Vec<String> = Vec::new();
            for line in reader.lines() {
                match line {
                    Ok(data) => dict.push(data),
                    Err(_) => return Result::Err(String::from("Error")),
                }
            }
            return Ok(dict);
        }
        Err(_) => return Result::Err(String::from("Error")),
    }
}
