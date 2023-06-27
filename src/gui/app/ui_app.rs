use rust_graphics::{
    app::App,
    color::{Color, COLOR_BLACK},
    draw_command::{DrawCommand, Stroke},
    events::app_events::AppEvent,
    font::Font,
    init_app,
    keycodes::KeyCode,
    rect::Rect,
    run_draw_command,
};

use crate::{
    gui::widget::builder::{
        build_context::{BuildContext, CursorDirection},
        build_results::WidgetRenderItem,
        relative_size::RelativeSize,
    },
    prelude::{AlignH, AlignV, WidgetInstance},
    print_widget_tree,
};

use super::input::InputState;

pub struct UIApp {
    main_container: Option<WidgetInstance>,
    default_font: Font,
    input_state: InputState,
    quit: bool,
}

impl UIApp {
    pub fn new() -> Self {
        init_app::<Self>().expect("Failed to initialize app")
    }

    pub fn main_container(mut self, widget: WidgetInstance) -> Self {
        print_widget_tree(&widget, 0);
        self.main_container = Some(widget);
        self
    }

    fn rebuild_main_container(&mut self, width: f32, height: f32) {
        //self.builder = WidgetBuilder::new(Rect::new_from_xy(0., 0., width, height));
        if let Some(container) = &mut self.main_container {
            let mut build_context = BuildContext::new(
                Rect::new_from_xy(0., 0., width, height),
                CursorDirection::Vertical,
            );

            container.build(&mut build_context);
        }
    }

    pub fn quit(&mut self) {
        self.quit = true;
    }
}

impl App for UIApp {
    fn init() -> Self {
        Self {
            main_container: None,
            default_font: Font::from_file("Roboto.ttf", 16),
            input_state: InputState::default(),
            quit: false,
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
            AppEvent::MouseMove { x, y } => {
                if let Some(container) = &self.main_container {
                    for item in container.iter() {
                        let (_, area) = item.build_result();
                        if area.contains((x as f32, y as f32).into())
                            && !area.contains(self.input_state.mouse_pos)
                        {
                            item.widget().on_mouse_enter();
                        } else if area.contains(self.input_state.mouse_pos)
                            && !area.contains((x, y).into())
                        {
                            item.widget().on_mouse_leave();
                        }
                    }
                }
                self.input_state.mouse_pos = (x as f32, y as f32).into();
            }
            AppEvent::MouseDown { x, y, .. } => {
                if let Some(container) = &self.main_container {
                    for item in container.iter() {
                        let (_, area) = item.build_result();
                        if area.contains((x as f32, y as f32).into()) {
                            item.widget().on_mouse_down();
                        }
                    }
                }
            }
            AppEvent::MouseUp { x, y, .. } => {
                if let Some(container) = &self.main_container {
                    for item in container.iter() {
                        let (_, area) = item.build_result();
                        if area.contains((x as f32, y as f32).into()) {
                            item.widget().on_mouse_up();
                        }
                    }
                }
            }
            AppEvent::KeyDown(KeyCode::Escape, _) => self.quit(),
            _ => (),
        };
    }

    fn on_draw(&mut self) {
        if let Some(container) = &self.main_container {
            for item in container.iter() {
                let (result, area) = item.build_result();

                for item in result.render_items().iter() {
                    match item {
                        WidgetRenderItem::Text(text) => {
                            let text = text.get().unwrap_or_default();
                            let line_top = self.default_font.get_line_top() as f32;
                            let line_bottom = self.default_font.get_line_bottom() as f32;
                            let text_width = self.default_font.get_text_width(&text.text) as f32;
                            let text_height = self.default_font.get_text_height(&text.text) as f32;

                            let text_base_line = match text.alignment_v {
                                AlignV::Top => area.top + line_top,
                                AlignV::Center => {
                                    area.center().y + line_bottom + (text_height / 2.)
                                }
                                AlignV::Bottom => area.bottom + line_bottom,
                            };
                            let text_left = match text.alignment_h {
                                AlignH::Left => area.left,
                                AlignH::Center => area.center().x - (text_width / 2.),
                                AlignH::Right => area.right - text_width,
                            };

                            run_draw_command(&DrawCommand::Text {
                                font: self.default_font,
                                text: text.text.clone(),
                                position: (text_left, text_base_line).into(),
                                color: COLOR_BLACK,
                            });
                        }
                        WidgetRenderItem::Rect(rect) => {
                            let width = match rect.width.get().unwrap() {
                                RelativeSize::Percent(percent) => area.width() * percent / 100.,
                                RelativeSize::PercentageH(percent) => area.width() * percent / 100.,
                                RelativeSize::PercentageV(percent) => {
                                    area.height() * percent / 100.
                                }
                            };
                            let height = match rect.height.get().unwrap() {
                                RelativeSize::Percent(percent) => area.height() * percent / 100.,
                                RelativeSize::PercentageH(percent) => area.width() * percent / 100.,
                                RelativeSize::PercentageV(percent) => {
                                    area.height() * percent / 100.
                                }
                            };

                            run_draw_command(&DrawCommand::Rect {
                                left: area.center().x - (width / 2.),
                                top: area.center().y - (height / 2.),
                                width: width,
                                height: height,
                                fill: rect.fill.get().unwrap_or(None),
                                stroke: rect.stroke.get().unwrap_or(None),
                            });
                        }
                    }
                }

                run_draw_command(&DrawCommand::Rect {
                    left: area.left,
                    top: area.top,
                    width: area.width(),
                    height: area.height(),
                    fill: None,
                    stroke: Some(Stroke {
                        width: 1.,
                        color: Color::new(
                            0, //((node.id & 0xff000000) >> 24) as u8,
                            0, //((node.id & 0xff0000) >> 16) as u8,
                            0, //((node.id & 0xff00) >> 8) as u8,
                            255,
                        ),
                    }),
                });
            }
        }
    }
    fn should_quit(&self) -> bool {
        self.quit
    }
}
