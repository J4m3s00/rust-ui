use crate::{
    draw_command::DrawCommand, quit_editor, rect::Rect, run_draw_command, start_editor, text::Text,
    update_editor, version::Version, AppEvent,
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
                run_draw_command(DrawCommand::Rect(Rect::new(0.0, 100.0, 100.0, 0.0)));
                run_draw_command(DrawCommand::Text(Text::new((0., 100.), "Hello World!")));
            }) {
                AppEvent::Quit => break,
                AppEvent::None => {}
            }
        }
        quit_editor();
        self
    }
}
