use rust_graphics::{events::app_events::KeyMods, keycodes::KeyCode};

use crate::{
    gui::{app::interface::AppInterface, events::mouse::MouseEvent},
    prelude::Observer,
};

use super::{
    builder::{build_context::BuildContext, build_results::BuildResult},
    widget_instance::WidgetInstance,
};

#[derive(Clone, Copy, Debug)]
pub enum WidgetMouseState {
    Normal,
    Hovered,
    Pressed,
}

pub trait Widget {
    fn build(
        &mut self,
        _ctx: &mut BuildContext,
        _mouse_state: Observer<WidgetMouseState>,
        _focused: Observer<bool>,
    ) -> BuildResult {
        BuildResult::default()
    }
    fn children(&self) -> &[WidgetInstance] {
        &[]
    }

    fn on_mouse_down(&self, _event: &MouseEvent, _interface: AppInterface) {}
    fn on_mouse_up(&self, _event: &MouseEvent, _interface: AppInterface) {}
    fn on_mouse_move(&self, _event: &MouseEvent, _interface: AppInterface) {}
    fn on_mouse_enter(&self, _event: &MouseEvent, _interface: AppInterface) {}
    fn on_mouse_leave(&self, _event: &MouseEvent, _interface: AppInterface) {}
    fn on_key_down(&self, _key: KeyCode, _mods: KeyMods, _interface: AppInterface) {}
    fn on_key_up(&self, _key: KeyCode, _mods: KeyMods, _interface: AppInterface) {}
    fn on_text_input(&self, _text: String, _interface: AppInterface) {}

    /// Tab focus
    fn on_gain_focus(&self) {}
    fn on_lose_focus(&self) {}
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
