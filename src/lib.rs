use gui::{
    container::Container,
    widget::{SizePolicy, Widget},
    widget_builder::WidgetBuilder,
};
use rust_graphics::{
    app::App, color::COLOR_BLACK, draw_command::DrawCommand, run_app, run_draw_command,
};

pub mod error;
pub mod gui;

use error::Result;

pub struct UIApp {
    main_container: Container,
}

impl UIApp {
    pub fn new() -> Self {
        Self {
            main_container: Container::new((800., 600.).into()),
        }
    }

    pub fn main_container<T>(mut self, builder: T) -> Self
    where
        T: FnOnce(&mut Container) -> Result<()>,
    {
        if let Err(err) = builder(&mut self.main_container) {
            println!("Error Creating main container: {}", err);
        }
        self
    }

    pub fn start(self) -> Result<()> {
        run_app(self);
        Ok(())
    }
}

impl App for UIApp {
    fn on_start(&mut self) {}

    fn on_draw(&mut self) {
        let mut builder = WidgetBuilder::default();

        self.main_container
            .build(&mut builder, (SizePolicy::Fill, SizePolicy::Fixed(20.)));

        for (text, pos) in builder.texts() {
            run_draw_command(&DrawCommand::Text {
                text: text.clone(),
                position: *pos,
                color: COLOR_BLACK,
            });
        }
    }
}
