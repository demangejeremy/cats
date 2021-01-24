mod lem;

use std::time::{Instant};

fn main() {
    let now = Instant::now();

    lem::hello();

    println!("Execution time: {}ms", now.elapsed().as_millis());
}