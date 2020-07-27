

use alloc::collections::BinaryHeap;
use core::cell::Cell;
use core::cmp::Ordering;
use super::Scheduler;

struct StrideInner<ThreadType: Clone + Eq> {
    pub thread: ThreadType,
    pass: usize,
    priority: Cell<usize>,
}

const BIG_STRIDE: usize =  1 << (core::mem::size_of::<usize>() * 8 - 2);

pub struct StrideScheduler<ThreadType: Clone + Eq> {
    pool: BinaryHeap<StrideInner<ThreadType>>,
    global_pass: usize
}

impl<ThreadType: Clone + Eq> PartialEq for StrideInner<ThreadType> {
    fn eq(&self, other: &Self) -> bool {
        self.pass == other.pass
    }
}

impl<ThreadType: Clone + Eq> Eq for StrideInner<ThreadType> {}

impl<ThreadType: Clone + Eq> Ord for StrideInner<ThreadType> {
    fn cmp(&self, other: &Self) -> Ordering {
        // 将大端堆转小端堆
        let ret1 = (self.pass as isize).cmp(&(other.pass as isize)).reverse();
        let diff = self.pass.wrapping_sub(other.pass) as isize;
        let ret2 = if diff < 0 { Ordering::Greater } else if diff == 0 { Ordering::Equal } else { Ordering::Less };
        if ret1 != ret2 {
            panic!("compare error bet {} and {}", self.pass, other.pass);
        }
        ret1
    }
}

impl<ThreadType: Clone + Eq> PartialOrd for StrideInner<ThreadType> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<ThreadType: Clone + Eq> Default for StrideScheduler<ThreadType> {
    fn default() -> Self {
        Self {
            pool: BinaryHeap::new(),
            global_pass: 0,
        }
    }
}

impl<ThreadType: Clone + Eq> Scheduler<ThreadType> for StrideScheduler<ThreadType> {
    type Priority = usize;

    fn add_thread(&mut self, thread: ThreadType) {
        self.pool.push(StrideInner {
            pass: self.global_pass,
            priority: Cell::new(1024),
            thread,
        })
    }
    fn get_next(&mut self) -> Option<ThreadType> {
        if let Some(mut selected) = self.pool.pop() {
            selected.pass += BIG_STRIDE / selected.priority.get();
            let ret = Some(selected.thread.clone());
            self.pool.push(selected);
            // update global pass
            return ret;
        }
        None
    }
    fn remove_thread(&mut self, thread: &ThreadType) {
        // 移除相应的线程并且确认恰移除一个线程
        let mut found = false;
        for inner in self.pool.iter() {
            if inner.thread == *thread {
                assert_eq!(found, false);
                found = true;
            }
        }
        assert_eq!(found, true);
        // 下面这句可以移除而不确认
        self.pool.retain(|t| t.thread != *thread);
    }
    fn set_priority(&mut self, thread: ThreadType, priority: usize) {
        if priority == 0 {
            panic!("priority cannot be 0")
        }
        let mut found = false;
        for inner in self.pool.iter() {
            if inner.thread == thread {
                assert_eq!(found, false);
                found = true;
                inner.priority.set(priority);
            }
        }
        assert_eq!(found, true);
    }
}
