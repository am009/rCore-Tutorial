#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::sys_tid;
use user_lib::sys_fork;
use user_lib::sys_open;
use user_lib::sys_read;
use user_lib::sys_write;
use user_lib::sys_pipe;

#[no_mangle]
pub fn main() -> usize {
    println!("Hello world from user thread {}!", sys_tid());
    
    let f = sys_open("tmp.txt".as_bytes());
    println!("file desc: {}", f);

    // if sys_fork() == 1 {
    //     
    // } else {
    //     println!("hello from child!");
    // }

    if f > 0 {
        let mut buf = [0 as u8; 3];
        let ret = sys_read(f as usize, &mut buf);
        println!("read ret: {}", ret);
        println!("get buf {}", core::str::from_utf8(&buf).unwrap());
    }
    
    let (write_fd, read_fd) = sys_pipe();
    if sys_fork() == 1 {
        // 子进程
        println!("hello from child!");
        // sys_close(read_fd); // 不一定需要实现
        let ret = sys_write(write_fd as usize, "hello_world".as_bytes());
        if ret > 0 {
            println!("write pipe success: {}", ret);
        } else {
            println!("write pipe failed.");
        }
        for i in 0..10 {
            println!("write once.");
            let ret = sys_write(write_fd as usize, "abcdefgh".as_bytes());
            if ret < 0 {
                println!("write pipe failed.");
                break;
            }
            if ret == 0 {
                println!("pipe full on write {}.", i);
            }
        }

    } else {
        // 父进程
        println!("hello from parent!");
        // sys_close(write_fd); // 不一定需要实现
        let mut buffer = [0u8; 11];
        let len = sys_read(read_fd as usize, &mut buffer);
        if len > 0 {
            println!("read pipe success: {}", len);
            println!("content: {}", core::str::from_utf8(&buffer).unwrap());
        } else {
            println!("read pipe failed.");
        }
        let mut buffer2 = [0u8; 8];
        for i in 0..7 {
            let ret = sys_read(read_fd as usize, &mut buffer2);
            if ret < 0 {
                println!("read pipe failed.");
                break;
            }
            if ret == 0 {
                println!("pipe full on read {}.", i);
            } else {
                println!("read {} bytes : {}.", ret, core::str::from_utf8(&buffer2).unwrap());
            }
        }
    }
    // for i in 1..0x8fffffusize {
    //     if i % 0x200000 == 0 {
    //         println!("hello");
    //     }
    // }
    0
}
