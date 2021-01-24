extern crate rust_stemmers;
use rust_stemmers::{Algorithm, Stemmer};

pub fn lematizer(text: &str) -> String { 
    let fr_stemmer = Stemmer::create(Algorithm::French);

    let split = text.split(" ");
    let mut sort_text = text.to_ascii_lowercase();

    for s in split {
        let result = fr_stemmer.stem(s);
        println!("{}", result);
    }

    
    return sort_text;
}
