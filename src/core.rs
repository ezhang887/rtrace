extern crate nix;

use crate::syscall;

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

fn parent(pid: Pid) -> i32 {
    loop {
        match waitpid(pid, None) {
            Ok(WaitStatus::Exited(_, code)) => {
                break code;
            }
            Ok(_) => (),
            Err(e) => println!("waitpid() failed: {:?}", e),
        }
        match ptrace::getregs(pid) {
            Ok(libc::user_regs_struct { orig_rax, .. }) => {
                println!("{:?}", syscall::get_syscall_name(orig_rax));
            }
            Err(e) => println!("ptrace::getregs() failed: {:?}", e),
        }
        ptrace::syscall(pid, None).expect("ptrace::syscall() failed");
    }
}

fn child(args: Vec<String>) {
    ptrace::traceme().expect("traceme() failed");
    exec(args);
}

fn exec(args: Vec<String>) {
    let args_cstring: Vec<CString> = args
        .iter()
        .map(|t| CString::new(t.as_bytes()).unwrap())
        .collect();
    let args_cstr: Vec<&CStr> = args_cstring.iter().map(|c| c.as_c_str()).collect();
    execvp(args_cstr[0], &args_cstr).expect("execvp() failed");
}
