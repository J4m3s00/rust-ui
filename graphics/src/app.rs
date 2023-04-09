use crate::{
    cursor::{Cursor, CursorDirection},
    draw_command::DrawCommand,
    label::Label,
    quit_editor,
    rect::Rect,
    run_draw_command,
    stack::VStack,
    start_editor, update_editor,
    vec::Vec2,
    version::Version,
    widget::{Widget, WidgetBuilder},
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
                let mut builder = WidgetBuilder {
                    commands: vec![],
                    cursor: Cursor {
                        position: Vec2::new(100., 200.),
                        direction: CursorDirection::Down,
                    },
                    panel_rect: Rect::new(0., 400., 400., 0.),
                };
                run_draw_command(DrawCommand::Rect(builder.panel_rect.clone()));
                self.v_stack.build(&mut builder);
                for command in builder.commands {
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
