//! 文件相关的内核功能

use super::*;
use core::slice::from_raw_parts_mut;
use crate::fs::{new_pipe, ROOT_INODE};

/// 从指定的文件中读取字符
///
/// 如果缓冲区暂无数据，返回 0；出现错误返回 -1
pub(super) fn sys_read(fd: usize, buffer: *mut u8, size: usize) -> SyscallResult {
    // 从进程中获取 inode
    let process = PROCESSOR.lock().current_thread().process.clone();
    if let Some(inode) = process.inner().descriptors.get(fd) {
        // 从系统调用传入的参数生成缓冲区
        let buffer = unsafe { from_raw_parts_mut(buffer, size) };
        // 尝试读取
        if let Ok(ret) = inode.read_at(0, buffer) {
            let ret = ret as isize;
            if ret > 0 {
                return SyscallResult::Proceed(ret);
            }
            if ret == 0 {
                return SyscallResult::Park(ret);
            }
        }
    }
    SyscallResult::Proceed(-1)
}

/// 将字符写入指定的文件
pub(super) fn sys_write(fd: usize, buffer: *mut u8, size: usize) -> SyscallResult {
    // 从进程中获取 inode
    let process = PROCESSOR.lock().current_thread().process.clone();
    if let Some(inode) = process.inner().descriptors.get(fd) {
        // 从系统调用传入的参数生成缓冲区
        let buffer = unsafe { from_raw_parts_mut(buffer, size) };
        // 尝试写入
        if let Ok(ret) = inode.write_at(0, buffer) {
            let ret = ret as isize;
            if ret > 0 {
                return SyscallResult::Proceed(ret);
            }
            if ret == 0 {
                return SyscallResult::Park(ret);
            }
        }
    }
    SyscallResult::Proceed(-1)
}

pub(super) fn sys_open(str_buf: *mut u8, size: usize) -> SyscallResult {
    let buffer = unsafe { from_raw_parts_mut(str_buf, size) };
    let mut ret = SyscallResult::Proceed(-1);
    if let Ok(name) = core::str::from_utf8(buffer) {
        if let Ok(inode) = ROOT_INODE.find(name) {
            let current = PROCESSOR.lock().current_thread();
            let vec = &mut current.process.inner().descriptors;
            vec.push(inode);
            ret = SyscallResult::Proceed((vec.len() - 1) as isize);
        }
    }
    ret
}

pub(super) fn sys_pipe() -> SyscallResult {
    let (in_node, out_node) = new_pipe();
    let current = PROCESSOR.lock().current_thread();
    let vec = &mut current.process.inner().descriptors;
    vec.push(in_node);
    vec.push(out_node);
    let len = vec.len();
    SyscallResult::Proceed2((len - 2) as isize, (len - 1) as isize)
}