use rust_graphics::{
    events::app_events::{AppEvent, KeyMods},
    keycodes::KeyCode,
};

pub enum KeyboardEvent {
    KeyDown(KeyCode, KeyMods),
    KeyUp(KeyCode, KeyMods),
    Text(String),
}

impl KeyboardEvent {
    pub fn from_app_event(event: &AppEvent) -> Option<Self> {
        match event {
            AppEvent::KeyDown(key, mods) => Some(KeyboardEvent::KeyDown(*key, *mods)),
            AppEvent::KeyUp(key, mods) => Some(KeyboardEvent::KeyUp(*key, *mods)),
            AppEvent::TextInput(text) => Some(KeyboardEvent::Text(text.clone())),
            _ => None,
        }
    }
}
