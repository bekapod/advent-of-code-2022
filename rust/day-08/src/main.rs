

use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let time = Instant::now();
    println!(
        "answer: {:?}, time: {:.2?}",
        day_08::solve(filename),
        time.elapsed()
    );
}