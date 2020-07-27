#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::sys_tid;

#[no_mangle]
pub fn main() -> usize {
    println!("Hello world from user thread {}!", sys_tid());
    0
}
