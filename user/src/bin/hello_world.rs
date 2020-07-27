#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::sys_tid;
use user_lib::sys_fork;
use user_lib::sys_open;
use user_lib::sys_read;

#[no_mangle]
pub fn main() -> usize {
    println!("Hello world from user thread {}!", sys_tid());
    
    let f = sys_open("tmp.txt".as_bytes());
    println!("file desc: {}", f);

    if sys_fork() == 1 {
        println!("hello from parent!");
    } else {
        println!("hello from child!");
    }

    if f > 0 {
        let mut buf = [0 as u8; 3];
        let ret = sys_read(f as usize, &mut buf);
        println!("read ret: {}", ret);
        println!("get buf {}", core::str::from_utf8(&buf).unwrap());
    }
    
    // for i in 1..0x8fffffusize {
    //     if i % 0x200000 == 0 {
    //         println!("hello");
    //     }
    // }
    0
}
