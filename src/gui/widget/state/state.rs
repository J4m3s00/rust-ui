use std::{cell::RefCell, rc::Rc};

use super::observable::Observer;

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

    pub fn map<M, F>(&self, func: F) -> Observer<M>
    where
        F: Fn(&T) -> M + 'static,
        M: Clone + 'static,
    {
        Observer::map(self.observe(), func)
    }
}

impl<T> From<T> for State<String>
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self::new(value.into())
    }
}
