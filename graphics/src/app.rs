use crate::{
    cursor::{Cursor, CursorDirection},
    label::Label,
    quit_editor, run_draw_command,
    stack::VStack,
    start_editor, update_editor,
    vec::Vec2,
    version::Version,
    widget::Widget,
    AppEvent,
};

pub struct App {
    pub name: String,
    pub version: Version,

    v_stack: VStack,
}

impl App {
    pub fn new(name: impl Into<String>, version: impl Into<Version>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            v_stack: VStack {
                children: vec![
                    Box::new(Label {
                        text: "Hello, world!".to_string(),
                        style: Default::default(),
                    }),
                    Box::new(Label {
                        text: "Hello, world 2!".to_string(),
                        style: Default::default(),
                    }),
                ],
                style: Default::default(),
            },
        }
    }

    pub fn run(self) -> Self {
        println!("Running {} v{}", self.name, self.version);
        start_editor(self.name.as_str());
        loop {
            match update_editor(|| {
                for command in self
                    .v_stack
                    .build(&Cursor {
                        direction: CursorDirection::Down,
                        position: Vec2::new(0.0, 0.0),
                    })
                    .commands
                {
                    run_draw_command(command);
                }
            }) {
                AppEvent::Quit => break,
                AppEvent::None => {}
            }
        }
        quit_editor();
        self
    }
}
