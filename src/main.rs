mod lem;
mod token;
mod theme;

use std::time::{Instant};

fn main() {
    let now = Instant::now();

    // lem::hello();
    let mut result = token::stop("Je suis un lecteur régulier de ce magazine de sport.");
    result = lem::lematizer(&result);
    result = theme::detection(&result);

    println!("=========");
    println!("{}", result);

    println!("(Execution time: {}ms)", now.elapsed().as_millis());
}