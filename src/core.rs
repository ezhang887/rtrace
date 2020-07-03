extern crate nix;

use std::ffi::{CStr, CString};

use nix::sys::ptrace;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{execvp, fork, ForkResult, Pid};

pub fn run(args: Vec<String>) {
    match fork() {
        Ok(ForkResult::Parent { child, .. }) => {
            parent(child);
        }
        Ok(ForkResult::Child) => {
            child(args);
        }
        Err(_) => println!("Fork failed"),
    }
}

fn parent(pid: Pid) {
    loop {
        match waitpid(pid, None) {
            Ok(WaitStatus::Exited(_, code)) => {
                break;
            }
            Ok(WaitStatus::Stopped(_, signal)) => (),
            Ok(s) => println!("Unexpected stop reason: {:?}", s),
            Err(e) => println!("waitpid()"),
        }
        //ptrace::getregs(pid);
        ptrace::syscall(pid, None);
    }
    ptrace::detach(pid, None);
}

fn child(args: Vec<String>) {
    ptrace::traceme();
    exec(args);
}

fn exec(args: Vec<String>) {
    let args_cstring: Vec<CString> = args
        .iter()
        .map(|t| CString::new(t.as_bytes()).unwrap())
        .collect();
    let args_cstr: Vec<&CStr> = args_cstring.iter().map(|c| c.as_c_str()).collect();
    execvp(args_cstr[0], &args_cstr);
}
