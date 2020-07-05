mod core;
mod syscall;

#[macro_use]
extern crate lazy_static;

use std::env;

fn main() {
    syscall::parse_syscall_table();
    let args: Vec<String> = env::args().collect();
    std::process::exit(core::run(args[1..].to_vec()));
}
