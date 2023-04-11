use crate::{
    clickable::Clickable,
    label::Label,
    quit_editor, run_draw_command,
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
                    Box::new(Clickable::default()),
                    Box::new(Clickable::default()),
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
                let mut builder = WidgetBuilder::new();
                builder.begin(Vec2::new(400., 300.));
                self.v_stack.build(&mut builder);
                builder.end();
                for command in builder.commands.iter() {
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
