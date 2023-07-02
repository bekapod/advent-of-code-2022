use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let time = Instant::now();
    println!(
        "answer #1: {:?}, answer #2: {:?}, time: {:.2?}",
        day_02::solve_first(filename),
        day_02::solve_second(filename),
        time.elapsed()
    );
}