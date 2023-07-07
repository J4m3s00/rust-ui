use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use rust_graphics::vec::Vec2;

use crate::prelude::WidgetInstance;

pub enum AppInterfaceEvent {
    Quit,
    OpenPanel(WidgetInstance, Vec2),
}

#[derive(Clone)]
pub struct AppInterface {
    pub(super) events: Weak<RefCell<Vec<AppInterfaceEvent>>>,
}

impl AppInterface {
    pub fn new(events: &Rc<RefCell<Vec<AppInterfaceEvent>>>) -> Self {
        Self {
            events: Rc::downgrade(events),
        }
    }

    pub fn quit(&self) {
        self.push_event(AppInterfaceEvent::Quit)
    }

    pub fn open_panel(&self, panel: WidgetInstance, position: Vec2) {
        self.push_event(AppInterfaceEvent::OpenPanel(panel, position));
    }

    fn push_event(&self, event: AppInterfaceEvent) {
        if let Some(events) = Weak::upgrade(&self.events) {
            RefCell::borrow_mut(Rc::as_ref(&events)).push(event);
        }
    }
}
