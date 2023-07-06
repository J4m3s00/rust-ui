use rust_graphics::{
    app::App,
    color::Color,
    draw_command::{DrawCommand, Stroke},
    events::app_events::AppEvent,
    font::Font,
    init_app,
    keycodes::KeyCode,
    rect::Rect,
    run_draw_command,
    vec::Vec2,
};

use crate::{
    gui::widget::{
        builder::build_context::{BuildContext, CursorDirection},
        style::space::ApplySpace,
        theme::theme::Theme,
        widget::MouseEvent,
    },
    prelude::WidgetInstance,
};

use super::{input::InputState, interface::AppInterface};

pub struct FontManager {
    default_font: Font,
}

impl FontManager {
    pub fn default_font(&self) -> &Font {
        &self.default_font
    }
}

pub struct UIApp {
    main_container: Option<WidgetInstance>,
    font_manager: FontManager,
    input_state: InputState,
    quit: bool,
    theme: Theme,
}

impl UIApp {
    pub fn new() -> Self {
        init_app::<Self>().expect("Failed to initialize app")
    }

    pub fn main_container(mut self, widget: WidgetInstance) -> Self {
        //print_widget_tree(&widget, 0);
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
            font_manager: FontManager {
                default_font: Font::from_file("Roboto.ttf", 16),
            },
            input_state: InputState::default(),
            quit: false,
            theme: Theme::default(),
        }
    }

    fn on_start(&mut self) {
        self.rebuild_main_container(800., 600.);
    }

    fn on_event(&mut self, event: AppEvent) {
        let interface = AppInterface::new();
        match event {
            AppEvent::WindowResize(width, height) => {
                self.rebuild_main_container(width as f32, height as f32);
            }
            AppEvent::MouseMove { x, y } => {
                let mouse_pos: Vec2 = (x as f32, y as f32).into();
                let event = MouseEvent {
                    absolute_pos: (x as f32, y as f32).into(),
                    delta: mouse_pos - self.input_state.mouse_pos,
                    ..Default::default()
                };

                if let Some(container) = &self.main_container {
                    for item in container.iter() {
                        let (_, area) = item.build_result();
                        let mouse_inside = area.contains(mouse_pos);
                        let last_mouse_inside = area.contains(self.input_state.mouse_pos);

                        let inside_event = MouseEvent {
                            relative_pos: mouse_pos - area.top_left(),
                            ..event
                        };

                        if mouse_inside && !last_mouse_inside {
                            item.widget()
                                .on_mouse_enter(inside_event.clone(), interface.clone());
                        } else if last_mouse_inside && !mouse_inside {
                            item.widget()
                                .on_mouse_leave(inside_event.clone(), interface.clone());
                        }
                        item.widget()
                            .on_mouse_move(inside_event.clone(), interface.clone());
                    }
                }
                self.input_state.mouse_pos = (x as f32, y as f32).into();
            }
            AppEvent::MouseDown { x, y, .. } => {
                if let Some(container) = &self.main_container {
                    for item in container.iter() {
                        let (_, area) = item.build_result();
                        let mouse_inside = area.contains((x as f32, y as f32).into());
                        item.widget().on_mouse_down(
                            MouseEvent {
                                relative_pos: (x as f32 - area.left, y as f32 - area.top).into(),
                                absolute_pos: (x as f32, y as f32).into(),
                                delta: Vec2::default(),
                                button: 0,
                                inside: mouse_inside,
                            },
                            interface.clone(),
                        );
                    }
                }
            }
            AppEvent::MouseUp { x, y, .. } => {
                let mouse_pos = (x as f32, y as f32).into();
                if let Some(container) = &self.main_container {
                    for item in container.iter() {
                        let (_, area) = item.build_result();
                        let mouse_inside = area.contains(mouse_pos);
                        item.widget().on_mouse_up(
                            MouseEvent {
                                relative_pos: (x as f32 - area.left, y as f32 - area.top).into(),
                                absolute_pos: (x as f32, y as f32).into(),
                                delta: Vec2::default(),
                                button: 0,
                                inside: mouse_inside,
                            },
                            interface.clone(),
                        );
                    }
                }
            }
            AppEvent::KeyDown(KeyCode::Escape, _) => self.quit(),
            _ => (),
        };

        if interface.inner.borrow().should_quit {
            self.quit();
        }
    }

    fn on_draw(&mut self) {
        if let Some(container) = &self.main_container {
            for item in container.iter() {
                if !item.visible() {
                    continue;
                }
                let (result, area) = item.build_result();
                let mut padded_area = area.clone();
                padded_area.apply_space(&item.style().padding);

                for item in result.render_items().iter() {
                    item.get_draw_command(&padded_area, &self.font_manager, &self.theme)
                        .iter()
                        .for_each(DrawCommand::run);
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
