use std::{cell::RefCell, rc::Rc};

use super::observable::{MapObserver, Observer};

pub(super) type StateInner<T> = RefCell<T>;

#[derive(Debug, Clone, Default)]

pub struct State<T: Clone> {
    pub(super) value: Rc<StateInner<T>>,
}

impl<T> State<T>
where
    T: Clone + 'static,
{
    pub fn new(value: T) -> Self {
        Self {
            value: Rc::new(RefCell::new(value)),
        }
    }

    pub fn get(&self) -> T {
        // TODO: Dont return copy here
        self.value.as_ref().borrow().clone()
    }

    pub fn set(&self, value: T) {
        self.value.as_ref().replace(value);
    }

    pub fn observe(&self) -> Observer<T> {
        Observer::state(self)
    }
}

impl<T> From<T> for State<T>
where
    T: Clone + 'static,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> MapObserver for State<T>
where
    T: Clone + 'static,
{
    type Value = T;
    fn map<M, F>(&self, func: F) -> Observer<M>
    where
        F: Fn(&T) -> M + 'static,
        M: Clone + 'static,
    {
        self.observe().map(func)
    }
}

impl<T, U> MapObserver for (&State<T>, &State<U>)
where
    T: Clone + 'static,
    U: Clone + 'static,
{
    type Value = (T, U);
    fn map<M, F>(&self, func: F) -> Observer<M>
    where
        F: Fn(&Self::Value) -> M + 'static,
        M: Clone + 'static,
    {
        (self.0.observe(), self.1.observe()).map(func)
    }
}
