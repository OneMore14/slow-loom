use crate::rt::runtime::schedule;
use crate::rt::state::State::{Blocking, ContextSwitch};
use std::cell::RefCell;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{AcqRel, Acquire};
use std::sync::LockResult;

pub struct Mutex<T> {
    locked: AtomicBool,
    _data: RefCell<T>, // for simplicity, we don't use it
}

/// safety: only be accessed by one thread
unsafe impl<T> Sync for Mutex<T> {}

pub struct MutexGuard<'a, T> {
    data: &'a Mutex<T>,
}

impl<T> Mutex<T> {
    pub fn new(data: T) -> Self {
        Mutex {
            locked: AtomicBool::new(false),
            _data: RefCell::new(data),
        }
    }

    pub fn lock(&self) -> LockResult<MutexGuard<'_, T>> {
        while self
            .locked
            .compare_exchange(false, true, AcqRel, Acquire)
            .is_err()
        {
            schedule(Blocking);
        }
        schedule(ContextSwitch);
        Ok(MutexGuard { data: self })
    }

    fn unlock(&self) {
        self.locked
            .compare_exchange(true, false, AcqRel, Acquire)
            .unwrap();
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        self.data.unlock();
    }
}
