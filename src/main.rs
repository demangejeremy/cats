mod lem;
mod token;

use std::time::{Instant};

fn main() {
    let now = Instant::now();

    // lem::hello();
    token::stop();

    println!("Execution time: {}ms", now.elapsed().as_millis());
}