use std::collections::HashMap;
use std::fs;

const SYSCALL_TABLE_PATH: &str = "/usr/include/x86_64-linux-gnu/asm/unistd_64.h";
const SYSCALL_DEFINE_HEADER: &str = "#define __NR_";

lazy_static! {
    static ref SYSCALL_TABLE: HashMap<u64, String> = parse_syscall_table();
}

pub fn get_syscall_name(num: libc::c_ulonglong) -> &'static str {
    return &SYSCALL_TABLE[&num];
}

pub fn parse_syscall_table() -> HashMap<u64, String> {
    let data = fs::read_to_string(SYSCALL_TABLE_PATH).expect("Unable to read syscall table");
    let lines: Vec<&str> = data
        .split('\n')
        .into_iter()
        .filter(|line| line.starts_with(SYSCALL_DEFINE_HEADER))
        .map(|line| &line[SYSCALL_DEFINE_HEADER.len()..])
        .collect();
    let mut rv = HashMap::new();
    for line in lines {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        rv.insert(
            u64::from_str_radix(tokens[1], 10).unwrap(),
            tokens[0].to_string(),
        );
    }
    return rv;
}
