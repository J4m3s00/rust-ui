use rust_graphics::{events::app_events::AppEvent, keycodes::KeyCode};

pub enum KeyboardEvent {
    KeyDown(KeyCode),
    KeyUp(KeyCode),
    Text(String),
}

impl KeyboardEvent {
    pub fn from_app_event(event: &AppEvent) -> Option<Self> {
        match event {
            AppEvent::KeyDown(key, _) => Some(KeyboardEvent::KeyDown(*key)),
            AppEvent::KeyUp(key, _) => Some(KeyboardEvent::KeyUp(*key)),
            AppEvent::TextInput(text) => Some(KeyboardEvent::Text(text.clone())),
            _ => None,
        }
    }
}
