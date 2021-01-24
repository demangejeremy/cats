mod lem;
mod token;

use std::time::{Instant};

fn main() {
    let now = Instant::now();

    // lem::hello();
    let mut result = token::stop("Bonjour à tous les amis !");
    result = lem::lematizer(&result);

    println!("=========");
    println!("{}", result);

    println!("(Execution time: {}ms)", now.elapsed().as_millis());
}