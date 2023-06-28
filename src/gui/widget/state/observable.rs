use std::{
    ops::Deref,
    rc::{Rc, Weak},
};

use super::state::{State, StateInner};

/// This is used to have a weak reference to a state
/// In essence this is just a wrapper to get a value
/// Either it is a reference to a state, a constant value
/// or a calculated value
///
/// The calculated value is used to map a state to another value

pub(super) trait Observable {
    type Value;

    fn get(&self) -> Option<Self::Value>;
}

#[derive(Clone)]
pub struct Observer<T>
where
    T: Clone + 'static,
{
    state: Rc<dyn Observable<Value = T>>,
}

impl<T> Default for Observer<T>
where
    T: Default + Clone + 'static,
{
    fn default() -> Self {
        Self::value_default()
    }
}

impl<T> Observer<T>
where
    T: Clone + 'static,
{
    pub(super) fn new(state: Rc<dyn Observable<Value = T>>) -> Self {
        Self { state }
    }

    pub fn value(val: T) -> Self {
        Self::new(Rc::new(ObserveValue::new(val)))
    }

    pub fn value_default() -> Self
    where
        T: Default,
    {
        Self::value(T::default())
    }

    pub fn state(state: &State<T>) -> Self {
        Self::new(Rc::new(ObserveState {
            state: Rc::downgrade(&state.value),
        }))
    }

    pub fn map<M, F>(&self, func: F) -> Observer<M>
    where
        F: Fn(&T) -> M + 'static,
        M: Clone + 'static,
    {
        Observer::new(Rc::new(ObserveMap {
            state: self.clone(),
            map: Box::new(func),
        }))
    }

    pub fn get(&self) -> Option<T> {
        self.state.get()
    }
}

impl<T> From<T> for Observer<T>
where
    T: Clone + 'static,
{
    fn from(value: T) -> Self {
        Self {
            state: Rc::new(ObserveValue { value }),
        }
    }
}

struct ObserveState<T: Clone> {
    state: Weak<StateInner<T>>,
}

impl<T> Observable for ObserveState<T>
where
    T: Clone,
{
    type Value = T;
    fn get(&self) -> Option<Self::Value> {
        self.state.upgrade().map(|state| {
            let state = state.borrow();
            state.deref().clone()
        })
    }
}

struct ObserveValue<T: Clone> {
    value: T,
}

impl<T> ObserveValue<T>
where
    T: Clone,
{
    fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T> Observable for ObserveValue<T>
where
    T: Clone,
{
    type Value = T;
    fn get(&self) -> Option<Self::Value> {
        Some(self.value.clone())
    }
}

pub struct ObserveMap<T, M>
where
    T: Clone + 'static,
{
    state: Observer<T>,
    map: Box<dyn Fn(&T) -> M>,
}

impl<T, M> Observable for ObserveMap<T, M>
where
    T: Clone + 'static,
    M: Clone,
{
    type Value = M;
    fn get(&self) -> Option<Self::Value> {
        Some((self.map)(&self.state.get()?))
    }
}
