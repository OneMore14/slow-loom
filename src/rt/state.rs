pub(crate) enum State {
    Spawn(Box<dyn FnOnce() -> State + Send + 'static>),

    ContextSwitch,

    Blocking,

    Finish,
}
