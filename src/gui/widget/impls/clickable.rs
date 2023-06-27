use rust_graphics::{color::Color, draw_command::Fill};

use crate::{
    gui::widget::{
        builder::{
            build_context::BuildContext,
            build_results::{BuildResult, WidgetRenderItem, WidgetRenderRect},
        },
        state::state::State,
    },
    prelude::{Receiver, ToInstance, Widget, WidgetInstance},
};

pub struct Clicked;

#[derive(Clone, Copy)]
enum MouseState {
    Normal,
    Hovered,
    Pressed,
}

pub struct Clickable {
    mouse_state: State<MouseState>,
    on_click: Box<dyn Receiver<Clicked>>,
}

impl Clickable {
    pub fn new<T>(on_click: T) -> WidgetInstance
    where
        T: Receiver<Clicked> + 'static,
    {
        Self {
            mouse_state: State::new(MouseState::Normal),
            on_click: Box::new(on_click),
        }
        .instance()
    }
}

impl Widget for Clickable {
    fn build(&mut self, _ctx: &mut BuildContext) -> BuildResult {
        BuildResult::default().with_render_item(WidgetRenderItem::Rect(WidgetRenderRect {
            fill: self.mouse_state.map(|v| match v {
                MouseState::Normal => Some(Fill {
                    color: Color::new(64, 64, 64, 255),
                }),
                MouseState::Hovered => Some(Fill {
                    color: Color::new(128, 128, 128, 255),
                }),
                MouseState::Pressed => Some(Fill {
                    color: Color::new(255, 255, 255, 255),
                }),
            }),
            ..Default::default()
        }))
    }

    fn on_mouse_down(&self) {
        self.mouse_state.set(MouseState::Pressed);
    }

    fn on_mouse_up(&self) {
        if let MouseState::Pressed = self.mouse_state.get() {
            self.mouse_state.set(MouseState::Hovered);
            self.on_click.action(Clicked);
        }
    }

    fn on_mouse_enter(&self) {
        self.mouse_state.set(MouseState::Hovered);
    }

    fn on_mouse_leave(&self) {
        self.mouse_state.set(MouseState::Normal);
    }
}
