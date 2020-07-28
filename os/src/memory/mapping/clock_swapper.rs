
use super::*;
use crate::memory::{frame::FrameTracker, *};
use alloc::vec::Vec;

/// 页面置换算法基础实现：FIFO
pub struct ClockSwapper {
    /// 记录映射和添加的顺序
    queue: Vec<(VirtualPageNumber, FrameTracker, *mut PageTableEntry)>,
    /// 映射数量上限
    quota: usize,
    pointer: usize,
}

impl Swapper for ClockSwapper {
    fn new(quota: usize) -> Self {
        Self {
            queue: Vec::new(),
            quota,
            pointer: 0,
        }
    }
    fn full(&self) -> bool {
        self.queue.len() == self.quota
    }
    fn pop(&mut self) -> Option<(VirtualPageNumber, FrameTracker)> {
        if self.queue.is_empty() {
            return None;
        }
        let mut ret = self.pointer;
        let len = self.queue.len();
        for ind in self.pointer..(self.pointer + len) {
            let item = &self.queue[ind % len];
            let pte = item.2;
            let accessed = unsafe { (*pte).flags().contains(Flags::ACCESSED) };
            if !accessed {
                ret = ind % len;
                self.pointer %= len - 1;
                break;
            } else {
                unsafe { (*pte).clear_accessed(); assert_eq!((*pte).flags().contains(Flags::ACCESSED), false) };
            }
        }
        let item = self.queue.remove(ret);
        Some((item.0, item.1))
    }
    fn push(&mut self, vpn: VirtualPageNumber, frame: FrameTracker, entry: *mut PageTableEntry) {
        self.queue.insert(self.pointer, (vpn, frame, entry));
        self.pointer = (self.pointer + 1) % self.queue.len();
    }
    fn retain(&mut self, predicate: impl Fn(&VirtualPageNumber) -> bool) {
        self.queue.retain(|(vpn, _, _)| predicate(vpn));
    }
}

unsafe impl Send for ClockSwapper {}
