extern crate unicode_normalization;

use unicode_normalization::char::compose;
use unicode_normalization::UnicodeNormalization;
use std::fs;

pub fn stop() { 
    let filename = "./lexique/fr/stopwords.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let split = contents.split("\n");

    println!("Stop:");

    for s in split {
        let word = s.to_ascii_lowercase();
        let ascii: String = word.nfd().filter(char::is_ascii).collect();
        println!("{}", ascii);
    }
}
