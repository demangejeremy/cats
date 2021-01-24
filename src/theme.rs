extern crate rust_stemmers;
extern crate unicode_normalization;

use rust_stemmers::{Algorithm, Stemmer};
use unicode_normalization::UnicodeNormalization;
use std::fs;

pub fn detection(text: &str) -> String { 
    let fr_stemmer = Stemmer::create(Algorithm::French);

    let original: String = text.to_string(); 
    let mut all_occurences: String = "".to_string();

    let paths = fs::read_dir("./lexique/fr/thematiques/").unwrap();
    println!("-----");

    for path in paths {

        let get_path: String = path.unwrap().path().display().to_string();

        if get_path.contains(".txt") {

            let contents = fs::read_to_string(&get_path)
                .expect("Something went wrong reading the file");
    
            let split = contents.split("\n");

            let mut cat = (&get_path).replace("./lexique/fr/thematiques/", "");
            let mut cat = cat.replace(".txt", "");

            // println!("{}", cat);
            // println!("-");

            for s in split {
                let mut word: String = s.to_ascii_lowercase();
                word = word.nfd().filter(char::is_ascii).collect();
                let stemming: String = fr_stemmer.stem(&word).to_string();
                if original.contains(&stemming) {
                    all_occurences.push_str(&cat);
                    all_occurences.push_str("/");
                }
                // println!("{}", stemming);
            }
        } 

    }

    println!("{}", all_occurences);
    let split = text.split(" ");
    let mut sort_text: String = "".to_string();


    return sort_text;
}
