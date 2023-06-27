use rust_graphics::vec::Vec2;

use crate::gui::app::interface::AppInterface;

use super::{
    builder::{build_context::BuildContext, build_results::BuildResult},
    widget_instance::WidgetInstance,
};

#[derive(Default, Clone)]
pub struct MouseEvent {
    pub relative_pos: Vec2,
    pub absolute_pos: Vec2,
    pub delta: Vec2,
    pub button: u8,
    pub inside: bool,
}

pub trait Widget {
    fn build(&mut self, _ctx: &mut BuildContext) -> BuildResult {
        BuildResult::default()
    }
    fn children(&self) -> &[WidgetInstance] {
        &[]
    }

    fn on_mouse_down(&self, event: MouseEvent, _interface: AppInterface) {}
    fn on_mouse_up(&self, event: MouseEvent, _interface: AppInterface) {}
    fn on_mouse_move(&self, event: MouseEvent, _interface: AppInterface) {}
    fn on_mouse_enter(&self, event: MouseEvent, _interface: AppInterface) {}
    fn on_mouse_leave(&self, event: MouseEvent, _interface: AppInterface) {}
}

pub trait ToInstance {
    fn instance(self) -> WidgetInstance;
}

impl<T> ToInstance for T
where
    T: Widget + 'static,
{
    fn instance(self) -> WidgetInstance {
        WidgetInstance::new(self)
    }
}
