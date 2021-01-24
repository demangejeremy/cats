extern crate rust_stemmers;
use rust_stemmers::{Algorithm, Stemmer};

pub fn lematizer(text: &str) -> String { 
    let fr_stemmer = Stemmer::create(Algorithm::French);

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
