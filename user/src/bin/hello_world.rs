#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::sys_tid;
use user_lib::sys_fork;

#[no_mangle]
pub fn main() -> usize {
    println!("Hello world from user thread {}!", sys_tid());
    
    if sys_fork() == 1 {
        println!("hello from parent!");
    } else {
        println!("hello from child!");
    }
    
    // for i in 1..0x8fffffusize {
    //     if i % 0x200000 == 0 {
    //         println!("hello");
    //     }
    // }
    0
}
