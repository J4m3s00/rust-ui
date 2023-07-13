use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use rust_graphics::{cursor::SystemCursor, vec::Vec2};

use crate::{gui::widget::theme::theme::Theme, prelude::WidgetInstance};

pub enum AppInterfaceEvent {
    Quit,
    OpenPanel(WidgetInstance, Vec2),
    ChangeTheme(Theme),
    ChangeCursor(SystemCursor),
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

    pub fn change_theme(&self, theme: Theme) {
        self.push_event(AppInterfaceEvent::ChangeTheme(theme));
    }

    pub fn change_cursor(&self, cursor: SystemCursor) {
        self.push_event(AppInterfaceEvent::ChangeCursor(cursor));
    }

    fn push_event(&self, event: AppInterfaceEvent) {
        if let Some(events) = Weak::upgrade(&self.events) {
            RefCell::borrow_mut(Rc::as_ref(&events)).push(event);
        }
    }
}
