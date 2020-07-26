#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
pub fn main() -> usize {
    println!("Hello world from user mode program!");
    for i in 1..0xffffffusize {
        if i % 0x200000 == 0 {
            println!("hello");
        }
    }
    0
}
