use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug, Clone, Default)]
pub struct State<T: Clone> {
    value: Rc<RefCell<T>>,
}

impl<T> State<T>
where
    T: Clone,
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

    pub fn observe(&self) -> Observable<T> {
        Observable::from(self)
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

#[derive(Debug, Clone)]
pub struct Observable<T> {
    value: Weak<RefCell<T>>,
}

impl<T> From<&State<T>> for Observable<T>
where
    T: Clone,
{
    fn from(state: &State<T>) -> Self {
        Self {
            value: Rc::downgrade(&state.value),
        }
    }
}

impl<T> Observable<T>
where
    T: Clone,
{
    pub fn get(&self) -> Option<T> {
        Some(self.value.upgrade()?.as_ref().borrow().clone())
    }
}
