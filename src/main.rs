mod lem;
mod token;

use std::time::{Instant};

fn main() {
    let now = Instant::now();

    // lem::hello();
    let mut result = token::stop("Je suis un lecteur régulier de ce magazine de sport.");
    result = lem::lematizer(&result);

    println!("=========");
    println!("{}", result);

    println!("(Execution time: {}ms)", now.elapsed().as_millis());
}