use slow_loom::sync::mutex::Mutex;
use slow_loom::thread;
use std::sync::Arc;

#[test]
#[should_panic(expected = "deadlock detected")]
fn check_dead_lock() {
    slow_loom::check(|| {
        let lock1 = Arc::new(Mutex::new(0));
        let lock2 = Arc::new(Mutex::new(0));

        let t1 = {
            let lock1 = lock1.clone();
            let lock2 = lock2.clone();
            thread::spawn(move || {
                let _guard1 = lock1.lock().unwrap();
                let _guard2 = lock2.lock().unwrap();
            })
        };

        let t2 = {
            let lock1 = lock1.clone();
            let lock2 = lock2.clone();
            thread::spawn(move || {
                let _guard2 = lock2.lock().unwrap();
                let _guard1 = lock1.lock().unwrap();
            })
        };

        t1.join().unwrap();
        t2.join().unwrap();
    });
}

#[test]
fn check_mutex() {
    slow_loom::check(|| {
        let lock1 = Arc::new(Mutex::new(0));
        let lock2 = Arc::new(Mutex::new(0));

        let t1 = {
            let lock1 = lock1.clone();
            let lock2 = lock2.clone();
            thread::spawn(move || {
                let _guard1 = lock1.lock().unwrap();
                let _guard2 = lock2.lock().unwrap();
            })
        };

        let t2 = {
            let lock1 = lock1.clone();
            let lock2 = lock2.clone();
            thread::spawn(move || {
                let _guard1 = lock1.lock().unwrap();
                let _guard2 = lock2.lock().unwrap();
            })
        };

        t1.join().unwrap();
        t2.join().unwrap();
    });
}
