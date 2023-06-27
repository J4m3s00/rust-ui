use std::rc::Weak;

use super::{observable::Observer, state::StateInner};

pub struct StateRef<T> {
    pub(super) value: Weak<StateInner<T>>,
}

impl<T> StateRef<T>
where
    T: Clone + 'static,
{
    pub fn get(&self) -> Option<T> {
        self.value.upgrade().map(|v| v.borrow().clone())
    }

    pub fn set(&self, val: T) {
        if let Some(value) = self.value.upgrade() {
            value.replace(val);
        }
    }

    pub fn observe(&self) -> Observer<T> {
        Observer::state_ref(self)
    }

    pub fn map<M, F>(&self, func: F) -> Observer<M>
    where
        F: Fn(&T) -> M + 'static,
        M: Clone + 'static,
    {
        Observer::map(self.observe(), func)
    }
}
