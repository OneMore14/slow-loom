use crate::rt::runtime::schedule;
use crate::rt::state::State::{Blocking, Finish, Spawn};
use std::sync::{Arc, Mutex};

pub struct JoinHandle<T> {
    data: Arc<Mutex<Option<T>>>,
}

impl<T> JoinHandle<T> {
    pub fn join(self) -> std::thread::Result<T> {
        loop {
            let mut data_guard = self.data.lock().unwrap();
            if data_guard.is_none() {
                drop(data_guard);
                schedule(Blocking);
            } else {
                let data = data_guard.take().unwrap();
                break Ok(data);
            }
        }
    }
}

pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    let return_value = Arc::new(Mutex::new(None));
    let return_value_handle = return_value.clone();
    let body = move || {
        let value = f();
        let mut guard = return_value_handle.lock().unwrap();
        *guard = Some(value);
        Finish
    };
    schedule(Spawn(Box::new(body)));
    JoinHandle { data: return_value }
}
