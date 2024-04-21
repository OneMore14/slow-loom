#![allow(deprecated)]

use std::sync::Arc;

use generator::{yield_with, Generator, Gn};

use crate::rt::state::State;
use crate::rt::state::State::{Finish, Spawn};

pub(crate) struct Runtime {
    generators: Vec<Generator<'static, (), State>>,
}

impl Runtime {
    pub fn new<F>(f: Arc<F>) -> Self
    where
        F: Fn() + Sync + Send + 'static,
    {
        let body = move || -> State {
            f();
            Finish
        };
        Runtime {
            generators: vec![Gn::<()>::new(body)],
        }
    }

    pub fn new_with_operations<F>(f: Arc<F>, operations: &Vec<usize>) -> Self
    where
        F: Fn() + Sync + Send + 'static,
    {
        let mut rt = Self::new(f);
        for op in operations {
            let _ = rt.tick_at(*op);
        }
        rt
    }

    /// run the chosen generator
    pub fn tick_at(&mut self, pos: usize) -> Result<(), RuntimeError> {
        let state = self.generators[pos].resume();
        if state.is_none() {
            self.generators.swap_remove(pos);
            return Ok(());
        }
        let state = state.unwrap();

        match state {
            Spawn(f) => {
                let g = Gn::<()>::new(f);
                self.generators.push(g);
            }
            State::Blocking => {
                return Err(RuntimeError::Blocking);
            }
            _ => {}
        };
        Ok(())
    }

    pub fn is_finished(&self) -> bool {
        self.generators.is_empty()
    }

    pub fn available_operations(&self) -> usize {
        self.generators.len()
    }
}

pub(crate) enum RuntimeError {
    Blocking,
}

pub(crate) fn schedule(caller: State) {
    yield_with(caller);
}
