extern crate nix;

use std::ffi::{CStr, CString};

//use nix::sys::ptrace;
use nix::sys::wait::waitpid;
use nix::unistd::{execvp, fork, ForkResult, Pid};

pub fn run(command: &str) {
    match fork() {
        Ok(ForkResult::Parent { child, .. }) => {
            parent(child);
        }
        Ok(ForkResult::Child) => {
            child(command);
        }
        Err(_) => println!("Fork failed"),
    }
}

fn parent(pid: Pid) {
    waitpid(pid, None);
}

fn child(command: &str) {
    exec(command);
}

fn exec(command: &str) {
    let tokens: Vec<&str> = command.split_whitespace().collect();
    let args: Vec<CString> = tokens
        .iter()
        .map(|t| CString::new(t.as_bytes()).unwrap())
        .collect();
    let args_cstr: Vec<&CStr> = args.iter().map(|c| c.as_c_str()).collect();
    execvp(args_cstr[0], &args_cstr);
}
