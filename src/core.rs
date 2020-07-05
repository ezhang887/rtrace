extern crate nix;

use crate::syscall;

use std::ffi::{CStr, CString};

use nix::sys::ptrace;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{execvp, fork, ForkResult, Pid};

pub fn run(args: Vec<String>) -> i32 {
    match fork() {
        Ok(ForkResult::Parent { child, .. }) => {
            return parent(child);
        }
        Ok(ForkResult::Child) => {
            child(args);
        }
        Err(_) => println!("Fork failed"),
    }
    return 0;
}

fn parent(pid: Pid) -> i32 {
    let mut print = true;
    loop {
        match waitpid(pid, None) {
            Ok(WaitStatus::Exited(_, code)) => {
                break code;
            }
            Ok(_) => (),
            Err(e) => println!("waitpid() failed: {:?}", e),
        }
        match ptrace::getregs(pid) {
            Ok(libc::user_regs_struct { orig_rax, rax, .. }) => {
                if print {
                    print!("{}", syscall::get_syscall_name(orig_rax));
                }
                // negate ENOSYS => flip bits and add 1
                if rax as i32 == !libc::ENOSYS + 1 {
                    print = false;
                } else {
                    print = true;
                    let mut returnval = rax as i64;
                    if returnval < 0 {
                        returnval += 1;
                    }
                    println!("={}", returnval);
                }
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
