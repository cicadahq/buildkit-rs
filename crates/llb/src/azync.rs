use std::sync::Mutex;

use crate::state::Constraints;
use crate::state::State;
use crate::Error;

pub struct AsyncState {
    f: Box<dyn Fn(State, &Constraints) -> Result<State, Error> + Send + Sync>,
    prev: State,
    target: Mutex<Option<State>>,
    set: Mutex<bool>,
    err: Mutex<Option<Error>>,
    g: Group<Key, State>,
}

pub struct ErrVertex {
    err: Error,
}
