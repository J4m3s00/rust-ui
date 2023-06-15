use std::any::TypeId;

use gui::{
    events::action::Action,
    text::{TextAlignH, TextAlignV},
    widget::Widget,
    widget_builder::{interactions::Click, WidgetBuilder},
};
use rust_graphics::{
    app::App,
    color::{Color, COLOR_BLACK},
    draw_command::{DrawCommand, Stroke},
    events::app_events::AppEvent,
    font::Font,
    init_app,
    rect::Rect,
    run_draw_command,
};

pub mod error;
pub mod gui;
pub mod prelude;

pub struct UIApp {
    main_container: Option<Box<dyn Widget>>,
    builder: WidgetBuilder,
    default_font: Font,
}

impl UIApp {
    pub fn new() -> Self {
        init_app::<Self>().expect("Failed to initialize app")
    }

    pub fn main_container<T, W>(mut self, builder: T) -> Self
    where
        T: FnOnce() -> W,
        W: Widget + 'static,
    {
        self.main_container = Some(Box::new(builder()));
        self
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
    fn init() -> Self {
        Self {
            main_container: None,
            builder: WidgetBuilder::new(Rect::new_from_xy(100., 100., 800., 300.)),
            default_font: Font::from_file("Roboto.ttf", 16),
        }
    }

    fn on_start(&mut self) {
        self.rebuild_main_container(800., 600.);
    }

    fn on_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::WindowResize(width, height) => {
                self.rebuild_main_container(width as f32, height as f32);
            }
            AppEvent::MouseDown { key, x, y } => {
                println!("Mouse down: {} {} {}", key, x, y);
                for node in self.builder.iter() {
                    if node.content_area.contains((x, y).into()) {
                        if let Some(interaction) = &node.interactions.get(&TypeId::of::<Click>()) {
                            //interaction.on_mouse_down(key, x, y);
                        }
                    }
                }
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
                let line_top = self.default_font.get_line_top() as f32;
                let line_bottom = self.default_font.get_line_bottom() as f32;
                let text_width = self.default_font.get_text_width(&text.text) as f32;
                let text_height = self.default_font.get_text_height(&text.text) as f32;

                let text_base_line = match text.alignment_v {
                    TextAlignV::Top => area.top + line_top,
                    TextAlignV::Center => area.center().y + line_bottom + (text_height / 2.),
                    TextAlignV::Bottom => area.bottom + line_bottom,
                };
                let text_left = match text.alignment_h {
                    TextAlignH::Left => area.left,
                    TextAlignH::Center => area.center().x - (text_width / 2.),
                    TextAlignH::Right => area.right - text_width,
                };

                run_draw_command(&DrawCommand::Text {
                    font: self.default_font,
                    text: text.text.clone(),
                    position: (text_left, text_base_line).into(),
                    color: COLOR_BLACK,
                });
            }
        }
    }
}
