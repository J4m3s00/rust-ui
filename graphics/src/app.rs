use crate::{
    bindings::c_draw_rect, quit_editor, start_editor, update_editor, version::Version, AppEvent,
};

pub struct App {
    pub name: String,
    pub version: Version,
}

impl App {
    pub fn new(name: impl Into<String>, version: impl Into<Version>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
        }
    }

    pub fn run(self) -> Self {
        println!("Running {} v{}", self.name, self.version);
        start_editor(self.name.as_str());
        loop {
            match update_editor(|| {
                unsafe { c_draw_rect(0., 0., 200., 200.) };
            }) {
                AppEvent::Quit => break,
                AppEvent::None => {}
            }
        }
        quit_editor();
        self
    }
}
