use gui::{widget::Widget, widget_builder::WidgetBuilder};
use rust_graphics::{
    app::App,
    color::{Color, COLOR_BLACK},
    draw_command::{DrawCommand, Stroke},
    events::app_events::AppEvent,
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
            builder: WidgetBuilder::new(Rect::new_from_xy(100., 100., 800., 300.)),
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

    fn rebuild_main_container(&mut self, width: f32, height: f32) {
        self.builder = WidgetBuilder::new(Rect::new_from_xy(0., 0., width, height));

        if let Some(container) = &self.main_container {
            let size = self.builder.root_node().content_area.size();
            container.build(&mut self.builder, size);
        }
    }
}

impl App for UIApp {
    fn on_start(&mut self) {
        self.rebuild_main_container(800., 600.);
    }

    fn on_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::WindowResize(width, height) => {
                self.rebuild_main_container(width as f32, height as f32);
            }
            _ => {}
        };
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
                    color: Color::new(
                        0, //((node.id & 0xff000000) >> 24) as u8,
                        0, //((node.id & 0xff0000) >> 16) as u8,
                        0, //((node.id & 0xff00) >> 8) as u8,
                        255,
                    ),
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
            }*/

            if let Some(text) = &node.text {
                run_draw_command(&DrawCommand::Text {
                    text: text.clone(),
                    position: area.top_left() + Vec2::new(0., 16.),
                    color: COLOR_BLACK,
                });
            }
        }
    }
}
