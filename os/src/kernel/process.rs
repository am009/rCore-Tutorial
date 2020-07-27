//! 进程相关的内核功能

use super::*;

pub(super) fn sys_exit(code: usize) -> SyscallResult {
    println!(
        "thread {} exit with code {}",
        PROCESSOR.lock().current_thread().id,
        code
    );
    SyscallResult::Kill
}

pub(super) fn sys_tid() -> SyscallResult {
    let tid = PROCESSOR.lock().current_thread().id;
    SyscallResult::Proceed(tid)
}

pub(super) fn sys_fork() -> SyscallResult {
    SyscallResult::Fork
}