use std::cell::RefCell;

use gui::{
    container::Container,
    widget::{SizePolicy, Widget},
    widget_builder::{PushChild, WidgetBuilder},
};
use rust_graphics::{
    app::App,
    color::{Color, COLOR_BLACK},
    draw_command::{DrawCommand, Fill, Stroke},
    rect::Rect,
    run_app, run_draw_command,
    vec::Vec2,
};

pub mod error;
pub mod gui;

use error::Result;

pub struct UIApp {
    main_container: Container,
    builder: WidgetBuilder,
}

impl UIApp {
    pub fn new() -> Self {
        Self {
            main_container: Container::default(),
            builder: WidgetBuilder::new(Rect::new_from_xy(100., 100., 800., 600.)),
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
    fn on_start(&mut self) {
        self.main_container.build(
            &mut PushChild::new(RefCell::new(&mut self.builder)),
            SizePolicy::Fill.into(),
        );
    }

    fn on_draw(&mut self) {
        let main_content = self.builder.child_content_area(1).unwrap();
        run_draw_command(&DrawCommand::Rect {
            left: main_content.left,
            top: main_content.top,
            width: main_content.width(),
            height: main_content.height(),
            fill: None,
            stroke: Some(Stroke {
                width: 4.,
                color: COLOR_BLACK,
            }),
        });

        for interaction in self.builder.interactions() {
            let area = interaction.interaction_rect;
            run_draw_command(&DrawCommand::Rect {
                left: area.left,
                top: area.top,
                width: area.width(),
                height: area.height(),
                fill: Some(Fill {
                    color: Color::new(55, 55, 55, 255),
                }),
                stroke: None,
            })
        }

        for (text, pos) in self.builder.texts() {
            run_draw_command(&DrawCommand::Text {
                text: text.clone(),
                position: *pos + Vec2::new(0., 16.),
                color: COLOR_BLACK,
            });
        }
    }
}
