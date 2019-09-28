extern crate libc;
use std::env;

#[link(name = "compile")]
extern {
    fn compile(input: *mut u8);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Incorrect number of arguments\n");
    }
    let mut input = args[1].clone();
    unsafe { compile(input.as_mut_ptr()) }
}