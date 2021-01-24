extern crate rust_stemmers;
extern crate unicode_normalization;

use rust_stemmers::{Algorithm, Stemmer};
use unicode_normalization::UnicodeNormalization;
use std::fs;

pub fn detection(text: &str) -> String { 
    let fr_stemmer = Stemmer::create(Algorithm::French);

    let paths = fs::read_dir("./lexique/fr/thematiques/").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }

    let split = text.split(" ");
    let mut sort_text: String = "".to_string();

    for s in split {
        let w = s.replace(" ", "");
        if w != "" {
            let result = fr_stemmer.stem(&w);
            sort_text.push_str(&result.to_ascii_lowercase());
            sort_text.push_str(&" ".to_string());
        }
    }

    return sort_text;
}
