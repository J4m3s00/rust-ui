use rust_graphics::draw_command::Fill;

use crate::{
    gui::{
        app::interface::AppInterface,
        widget::{
            builder::{build_context::BuildContext, build_results::BuildResult},
            rendering::drawable::rectangle::DrawRect,
            state::state::State,
            widget::MouseEvent,
        },
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
    fn build(&mut self, ctx: &mut BuildContext) -> BuildResult {
        let mut res = BuildResult::default();
        let normal_color = ctx.theme().primary_color;
        let hoverd_color = ctx.theme().primary_color_variant;
        let clicked_color = ctx.theme().secondary_color;

        res.draw_rect(DrawRect::fill(self.mouse_state.map(move |v| match v {
            MouseState::Normal => Some(Fill {
                color: normal_color,
            }),
            MouseState::Hovered => Some(Fill {
                color: hoverd_color,
            }),
            MouseState::Pressed => Some(Fill {
                color: clicked_color,
            }),
        })));
        res
    }

    fn on_mouse_down(&self, event: MouseEvent, _interface: AppInterface) {
        if event.inside {
            self.mouse_state.set(MouseState::Pressed);
        }
    }

    fn on_mouse_up(&self, _event: MouseEvent, interface: AppInterface) {
        if let MouseState::Pressed = self.mouse_state.get() {
            self.mouse_state.set(MouseState::Hovered);
            self.on_click.action(Clicked, interface);
        }
    }

    fn on_mouse_enter(&self, _event: MouseEvent, _interface: AppInterface) {
        self.mouse_state.set(MouseState::Hovered);
    }

    fn on_mouse_leave(&self, _event: MouseEvent, _interface: AppInterface) {
        self.mouse_state.set(MouseState::Normal);
    }
}
