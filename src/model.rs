use std::collections::VecDeque;
use std::sync::Arc;

use crate::rt::runtime::{Runtime, RuntimeError};

struct Context<F> {
    rt: Runtime,
    operations: Vec<usize>,
    f: Arc<F>,
}

impl<F: Fn() + Sync + Send + 'static> Context<F> {
    fn new(f: Arc<F>) -> Self {
        Context {
            rt: Runtime::new(f.clone()),
            operations: vec![],
            f,
        }
    }

    fn new_with_operations(f: Arc<F>, operations: Vec<usize>) -> Self {
        Context {
            rt: Runtime::new_with_operations(f.clone(), &operations),
            operations,
            f,
        }
    }

    fn tick_all(&self) -> Vec<Context<F>> {
        if self.rt.is_finished() {
            return vec![];
        }
        let mut new_contexts = vec![];
        let operations = self.rt.available_operations();
        for i in 0..operations {
            let mut new_context = self.clone();
            if new_context.tick_at(i).is_ok() {
                new_contexts.push(new_context);
            }
        }
        new_contexts
    }

    fn tick_at(&mut self, pos: usize) -> Result<(), RuntimeError> {
        self.operations.push(pos);
        self.rt.tick_at(pos)
    }

    fn is_finished(&self) -> bool {
        self.rt.is_finished()
    }
}

impl<F: Fn() + Sync + Send + 'static> Clone for Context<F> {
    fn clone(&self) -> Self {
        Context::new_with_operations(self.f.clone(), self.operations.clone())
    }
}

pub fn check<F>(f: F)
where
    F: Fn() + Sync + Send + 'static,
{
    let f = Arc::new(f);
    let init_context = Context::new(f);

    let mut contexts = VecDeque::new();
    contexts.push_back(init_context);

    while let Some(ctx) = contexts.pop_front() {
        let new_ctxs = ctx.tick_all();
        if !ctx.is_finished() && new_ctxs.is_empty() {
            panic!("deadlock detected");
        }
        for new_ctx in new_ctxs {
            contexts.push_back(new_ctx);
        }
    }
}
