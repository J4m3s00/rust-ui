use gui::widget::build_context::BuildContext;
use prelude::Widget;
use rust_graphics::{app::App, events::app_events::AppEvent, font::Font, init_app, rect::Rect};

pub mod actions;
pub mod error;
pub mod gui;
pub mod prelude;

pub struct UIApp {
    main_container: Option<Box<dyn Widget>>,
    default_font: Font,
}

impl UIApp {
    pub fn new() -> Self {
        init_app::<Self>().expect("Failed to initialize app")
    }

    pub fn main_container(mut self, builder: impl Widget + 'static) -> Self {
        self.main_container = Some(Box::new(builder));
        self
    }

    fn rebuild_main_container(&mut self, width: f32, height: f32) {
        let build_context = BuildContext::new(Rect::new_from_xy(0., 0., width, height));

        //self.builder = WidgetBuilder::new(Rect::new_from_xy(0., 0., width, height));
        //
        //if let Some(container) = &self.main_container {
        //    let size = self.builder.root_node().content_area.size();
        //    container.build(&mut self.builder, size);
        //}
    }
}

impl App for UIApp {
    fn init() -> Self {
        Self {
            main_container: None,
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
                unimplemented!()
            }
            _ => {}
        };
    }

    fn on_draw(&mut self) {
        /*
        let area = item.rect;
           run_draw_command(&DrawCommand::Rect {
               left: area.left,
               top: area.top,
               width: area.width(),
               height: area.height(),
               fill: Some(Fill {
                   color: self.style.background_color,
               }),
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

           if let Some(text) = &item.text {
               let line_top = self.style.font.get_line_top() as f32;
               let line_bottom = self.style.font.get_line_bottom() as f32;
               let text_width = self.style.font.get_text_width(&text.text) as f32;
               let text_height = self.style.font.get_text_height(&text.text) as f32;

               let text_base_line = match text.alignment_v {
                   AlignV::Top => area.top + line_top,
                   AlignV::Center => area.center().y + line_bottom + (text_height / 2.),
                   AlignV::Bottom => area.bottom + line_bottom,
               };
               let text_left = match text.alignment_h {
                   AlignH::Left => area.left,
                   AlignH::Center => area.center().x - (text_width / 2.),
                   AlignH::Right => area.right - text_width,
               };

               run_draw_command(&DrawCommand::Text {
                   font: self.style.font,
                   text: text.text.clone(),
                   position: (text_left, text_base_line).into(),
                   color: self.style.foreground_color,
               });
           } */
    }
}
