use std::{cell::RefCell, rc::Rc};

use rust_graphics::{
    app::App,
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
    gui::{
        events::{keyboard::KeyboardEvent, mouse::MouseEvent},
        widget::{
            builder::build_context::{BuildContext, CursorDirection},
            style::space::ApplySpace,
            theme::theme::Theme,
        },
    },
    prelude::{ColorId, WidgetInstance},
    print_widget_tree,
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
    fn new(widget: WidgetInstance, position: Vec2, size: Vec2) -> Self {
        Self {
            widget,
            position,
            size,
        }
    }

    fn build(&mut self) {
        let mut build_context = BuildContext::new(
            Rect::new_from_xy(self.position.x, self.position.y, self.size.x, self.size.y),
            CursorDirection::Vertical,
        );

        self.widget.build(&mut build_context);
    }

    fn draw(&self, font_manager: &FontManager, theme: &Theme, input_state: &InputState) {
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
            let (result, area, _) = item.build_result();
            let mut padded_area = *area;
            padded_area.apply_space(&item.style().padding);

            for item in result.render_items().iter() {
                item.get_draw_command(&padded_area, font_manager, theme)
                    .iter()
                    .for_each(DrawCommand::run);
            }
            if let Some(id) = input_state.focused_input {
                if id == item.id() && false {
                    run_draw_command(&DrawCommand::Rect {
                        left: area.left,
                        top: area.top,
                        width: area.width(),
                        height: area.height(),
                        fill: None,
                        stroke: Some(Stroke {
                            width: 1.,
                            color: theme.colors.from_id(ColorId::OnPrimary),
                        }),
                    });
                }
            }
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
        print_widget_tree(&widget, 0);
        self.panels
            .push(Panel::new(widget, (0., 0.).into(), (0., 0.).into()));
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

    pub fn focus_next_input(&mut self) {
        if let Some(top_panel) = self.panels.last() {
            if let Some(current_active) = self.input_state.focused_input {
                let mut current_active_iter = top_panel
                    .widget
                    .iter()
                    .filter(|widget| widget.does_accept_input())
                    .skip_while(|widget| widget.id() != current_active);
                let last_selected = current_active_iter.next();
                let next_selected = current_active_iter.next();
                self.input_state.focused_input = next_selected.map(WidgetInstance::id);
                if let Some(last_selected) = last_selected {
                    last_selected.on_lose_focus();
                }
                if let Some(next_selected) = next_selected {
                    next_selected.on_focus();
                }
            } else {
                self.input_state.focused_input = top_panel.widget.iter().find_map(|widget| {
                    if widget.does_accept_input() {
                        widget.on_focus();
                        Some(widget.id())
                    } else {
                        None
                    }
                });
            }
        }
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
                // If the widget is input active, we send over the key events
                if let Some(id) = self.input_state.focused_input {
                    if id == widget.id() {
                        if let Some(keyboard_event) = KeyboardEvent::from_app_event(&event) {
                            widget.handle_keyboard_event(&keyboard_event, interface.clone());
                        }
                    }
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
            AppEvent::KeyDown(KeyCode::Escape, _) => self.input_state.focused_input = None,
            AppEvent::KeyDown(KeyCode::Tab, _) => self.focus_next_input(),
            AppEvent::TextInput(text) => {
                println!("Text input: {}", text);
            }
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
                    let mut panel = Panel::new(panel, pos, (250., 24.).into());
                    panel.build();
                    self.panels.push(panel);
                }
                AppInterfaceEvent::ChangeTheme(theme) => self.theme = theme,
            }
        }
    }

    fn on_draw(&mut self) {
        for panel in self.panels.iter() {
            panel.draw(&self.font_manager, &self.theme, &self.input_state);
        }
    }
    fn should_quit(&self) -> bool {
        self.quit
    }
}
