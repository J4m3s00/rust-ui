use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
pub(super) struct Inner {
    pub(super) should_quit: bool,
}

#[derive(Clone)]
pub struct AppInterface {
    pub(super) inner: Rc<RefCell<Inner>>,
}

impl AppInterface {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(Inner { should_quit: false })),
        }
    }

    pub fn quit(&self) {
        Rc::as_ref(&self.inner).borrow_mut().should_quit = true;
    }
}
