use std::cell::Cell;
use std::sync::atomic::Ordering;

use crate::rt::runtime::schedule;
use crate::rt::state::State::ContextSwitch;

pub struct AtomicUsize {
    value: Cell<usize>,
}

/// safety: value will be accessed by a single thread
unsafe impl Sync for AtomicUsize {}

impl AtomicUsize {
    pub fn new(value: usize) -> Self {
        AtomicUsize {
            value: Cell::new(value),
        }
    }

    pub fn load(&self, _ordering: Ordering) -> usize {
        let val = self.get_inner_value();
        schedule(ContextSwitch);
        val
    }

    fn get_inner_value(&self) -> usize {
        unsafe { self.value.as_ptr().read() }
    }

    pub fn store(&self, val: usize, _ordering: Ordering) {
        self.value.set(val);
        schedule(ContextSwitch);
    }

    pub fn fetch_add(&self, val: usize, _ordering: Ordering) {
        let old = self.get_inner_value();
        self.value.set(old + val);
        schedule(ContextSwitch);
    }
}
