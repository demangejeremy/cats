extern crate native_stemmers;
use native_stemmers::{Algorithm, Stemmer};

// Create a stemmer for the english language
let en_stemmer = Stemmer::create(Algorithm::English);

// Stemm the word "fruitlessly"
// Please be aware that all algorithms expect their input to only contain lowercase characters.
assert_eq!(en_stemmer.stem("fruitlessly"), "fruitless");
