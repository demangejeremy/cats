extern crate unicode_normalization;

use unicode_normalization::UnicodeNormalization;
use std::fs;

pub fn stop(text: &str) -> String { 
    let filename = "./lexique/fr/stopwords.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let split = contents.split("\n");

    let mut sort_text = text.to_ascii_lowercase();
    sort_text = text.nfd().filter(char::is_ascii).collect();

    sort_text = sort_text.replace(".", "");
    sort_text = sort_text.replace(",", "");
    sort_text = sort_text.replace(";", "");
    sort_text = sort_text.replace("!", "");
    sort_text = sort_text.replace("?", "");

    for s in split {
        let word = s.to_ascii_lowercase();
        let ascii: String = word.nfd().filter(char::is_ascii).collect();
        let cut = [" ".to_string(), ascii, " ".to_string()].join("");
        sort_text = sort_text.replace(&cut, " ");
    }

    return sort_text;
}
