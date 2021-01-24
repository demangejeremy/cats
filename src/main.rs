mod lem;
mod token;

use std::time::{Instant};

fn main() {
    let now = Instant::now();

    // lem::hello();
    let result = token::stop("Bonjour à tous les amis !");

    println!("=========");
    println!("{}", result);

    println!("(Execution time: {}ms)", now.elapsed().as_millis());
}