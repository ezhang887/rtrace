mod core;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    core::run(args[1..].to_vec());
}
