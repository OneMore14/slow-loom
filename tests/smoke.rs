use std::sync::atomic::Ordering;
use std::sync::atomic::Ordering::{AcqRel, Acquire, Relaxed, Release};
use std::sync::Arc;

use slow_loom::sync::atomic::AtomicUsize;
use slow_loom::thread;

#[test]
#[should_panic]
fn check_fail() {
    struct BuggyInc {
        num: AtomicUsize,
    }

    impl BuggyInc {
        fn new() -> BuggyInc {
            BuggyInc {
                num: AtomicUsize::new(0),
            }
        }

        fn inc(&self) {
            let curr = self.num.load(Acquire);
            self.num.store(curr + 1, Release);
        }
    }

    slow_loom::check(|| {
        let buggy_inc = Arc::new(BuggyInc::new());

        let ths: Vec<_> = (0..2)
            .map(|_| {
                let buggy_inc = buggy_inc.clone();
                thread::spawn(move || {
                    buggy_inc.inc();
                })
            })
            .collect();

        for th in ths {
            th.join().unwrap();
        }
        assert_eq!(2, buggy_inc.num.load(Relaxed));
    });
}

#[test]
fn check_fetch_add() {
    slow_loom::check(|| {
        let inc = Arc::new(AtomicUsize::new(0));

        let ths: Vec<_> = (0..2)
            .map(|_| {
                let inc = inc.clone();
                thread::spawn(move || {
                    inc.fetch_add(1, AcqRel);
                })
            })
            .collect();

        for th in ths {
            th.join().unwrap();
        }
        assert_eq!(2, inc.load(Relaxed));
    });
}

#[test]
fn single_thread_success() {
    slow_loom::check(|| {
        let v = AtomicUsize::new(1);
        let value = v.load(Acquire);
        v.store(value + 1, Release);

        assert_eq!(2, v.load(Acquire));
    });
}
