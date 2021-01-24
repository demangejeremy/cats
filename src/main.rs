mod lem;
mod token;
mod theme;

use std::time::{Instant};

fn main() {
    let now = Instant::now();

    // Commande line
    let phrase = std::env::args().nth(1).expect("no sentence in CLI.");

    // lem::hello();
    let mut result = token::stop(&phrase);
    result = lem::lematizer(&result);
    result = theme::detection(&result);

    println!("=========");
    if result != "" {
        println!("Thematique : {}", result);
    } else {
        println!("Thematique : none");
    }

    println!("(Execution time: {}ms)", now.elapsed().as_millis());
}