use std::{cell::RefCell, rc::Rc};

use rust_graphics::{
    app::App,
    color::Color,
    draw_command::{DrawCommand, Fill, Stroke},
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
    prelude::{ColorId, WidgetInstance},
};

use super::{
    input::InputState,
    interface::{AppInterface, AppInterfaceEvent},
};

pub struct FontManager {
    default_font: Font,
}

impl FontManager {
    pub fn default_font(&self) -> &Font {
        &self.default_font
    }
}

struct Panel {
    widget: WidgetInstance,
    position: Vec2,
    size: Vec2,
}

impl Panel {
    fn build(&mut self) {
        let mut build_context = BuildContext::new(
            Rect::new_from_xy(self.position.x, self.position.y, self.size.x, self.size.y),
            CursorDirection::Vertical,
        );

        self.widget.build(&mut build_context);
    }

    fn draw(&self, font_manager: &FontManager, theme: &Theme) {
        DrawCommand::rect_fill(
            self.position.x,
            self.position.y,
            self.size.x,
            self.size.y,
            Fill::new(theme.colors.from_id(ColorId::Background)),
        )
        .run();
        for item in self.widget.iter() {
            if !item.visible() {
                continue;
            }
            let (result, area) = item.build_result();
            let mut padded_area = *area;
            padded_area.apply_space(&item.style().padding);

            for item in result.render_items().iter() {
                item.get_draw_command(&padded_area, font_manager, theme)
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
pub struct UIApp {
    panels: Vec<Panel>,

    font_manager: FontManager,
    input_state: InputState,
    quit: bool,
    theme: Theme,
}

impl Default for UIApp {
    fn default() -> Self {
        init_app::<Self>().expect("Failed to initialize app")
    }
}

impl UIApp {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn main_container(mut self, widget: WidgetInstance) -> Self {
        if !self.panels.is_empty() {
            panic!("Main container must be set before any other panel");
        }
        self.panels.push(Panel {
            widget,
            position: (0., 0.).into(),
            size: (0., 0.).into(),
        });
        self
    }

    fn rebuild_main_container(&mut self, width: f32, height: f32) {
        let Some(main_container) = self.panels.first_mut() else {
            println!("No main container set. Cant rebuild");
            return;
        };
        main_container.size = (width, height).into();
        main_container.build();
    }

    pub fn quit(&mut self) {
        self.quit = true;
    }
}

impl App for UIApp {
    fn init() -> Self {
        Self {
            panels: Vec::new(),
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
        let interface_events = Rc::new(RefCell::new(Vec::new()));

        let interface = AppInterface::new(&interface_events);

        for panel in self.panels.iter_mut() {
            for widget in panel.widget.iter() {
                if let Some(event) =
                    MouseEvent::from_app_event(&event, widget, self.input_state.mouse_pos)
                {
                    widget.handle_mouse_event(&event, interface.clone());
                }
            }
        }

        match event {
            AppEvent::WindowResize(width, height) => {
                self.rebuild_main_container(width as f32, height as f32);
            }
            AppEvent::MouseMove { x, y } => {
                self.input_state.mouse_pos = (x as f32, y as f32).into();
            }
            AppEvent::MouseDown { x, y, .. } => {
                let mut first = true;

                self.panels.retain(|panel| {
                    if first {
                        first = false;
                        return true;
                    }
                    Rect::new_from_xy(
                        panel.position.x,
                        panel.position.y,
                        panel.size.x,
                        panel.size.y,
                    )
                    .contains((x as f32, y as f32).into())
                })
            }
            AppEvent::KeyDown(KeyCode::Escape, _) => self.quit(),
            _ => (),
        };

        for event in match Rc::try_unwrap(interface_events) {
            Ok(this) => this,
            Err(_) => panic!(
                "Failed to unwrap Rc. This should never hapen. Something went wrong in the API. Some event still has access to the event queue of the ipc."
            ),
        }
        .into_inner()
        .into_iter()
        {
            match event {
                AppInterfaceEvent::Quit => self.quit(),
                AppInterfaceEvent::OpenPanel(panel, pos) => {
                    println!("Open panel");
                    let mut panel = Panel { widget: panel, position: pos, size: (250., 24.).into() };
                    panel.build();
                    self.panels.push(panel);
                }
                AppInterfaceEvent::ChangeTheme(theme) => self.theme = theme,
            }
        }
    }

    fn on_draw(&mut self) {
        for panel in self.panels.iter() {
            panel.draw(&self.font_manager, &self.theme);
        }
    }
    fn should_quit(&self) -> bool {
        self.quit
    }
}
