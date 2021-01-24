extern crate rust_stemmers;
use rust_stemmers::{Algorithm, Stemmer};

pub fn hello() { 
    let en_stemmer = Stemmer::create(Algorithm::French);

    // Stemm the word "fruitlessly"
    // Please be aware that all algorithms expect their input to only contain lowercase characters.
    let result = en_stemmer.stem("ballon");
    
    println!("{}", result);
}
