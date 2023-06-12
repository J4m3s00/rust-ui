use gui::{
    widget::{SizePolicy, Widget},
    widget_builder::WidgetBuilder,
};
use rust_graphics::{
    app::App,
    color::{Color, COLOR_BLACK, COLOR_BLUE},
    draw_command::{DrawCommand, Fill, Stroke},
    rect::Rect,
    run_app, run_draw_command,
    vec::Vec2,
};

pub mod error;
pub mod gui;

use error::Result;

pub struct UIApp {
    main_container: Option<Box<dyn Widget>>,
    builder: WidgetBuilder,
}

impl UIApp {
    pub fn new() -> Self {
        Self {
            main_container: None,
            builder: WidgetBuilder::new(Rect::new_from_xy(100., 100., 800., 600.)),
        }
    }

    pub fn main_container<T, W>(mut self, builder: T) -> Self
    where
        T: FnOnce() -> W,
        W: Widget + 'static,
    {
        self.main_container = Some(Box::new(builder()));
        self
    }

    pub fn start(self) -> Result<()> {
        run_app(self);
        Ok(())
    }
}

impl App for UIApp {
    fn on_start(&mut self) {
        if let Some(container) = &self.main_container {
            container.build(&mut self.builder, SizePolicy::Percentage(1.0).into());
        }
    }

    fn on_draw(&mut self) {
        for node in self.builder.iter() {
            let area = node.content_area;
            run_draw_command(&DrawCommand::Rect {
                left: area.left,
                top: area.top,
                width: area.width(),
                height: area.height(),
                fill: None,
                stroke: Some(Stroke {
                    width: 2.,
                    color: COLOR_BLUE,
                }),
            });
            /*if let Some(_) = &node.interaction {
                run_draw_command(&DrawCommand::Rect {
                    left: area.left,
                    top: area.top,
                    width: area.width(),
                    height: area.height(),
                    fill: Some(Fill {
                        color: Color::new(55, 55, 55, 255),
                    }),
                    stroke: None,
                });
            }

            if let Some(text) = &node.text {
                run_draw_command(&DrawCommand::Text {
                    text: text.clone(),
                    position: area.top_left() + Vec2::new(0., 16.),
                    color: COLOR_BLACK,
                });
            }*/
        }
    }
}
