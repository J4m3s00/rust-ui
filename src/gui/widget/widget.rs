use crate::gui::app::interface::AppInterface;

use super::{
    builder::{build_context::BuildContext, build_results::BuildResult},
    widget_instance::WidgetInstance,
};

pub trait Widget {
    fn build(&mut self, _ctx: &mut BuildContext) -> BuildResult {
        BuildResult::default()
    }
    fn children(&self) -> &[WidgetInstance] {
        &[]
    }

    fn on_mouse_down(&self, _interface: AppInterface) {}
    fn on_mouse_up(&self, _interface: AppInterface) {}
    fn on_mouse_enter(&self, _interface: AppInterface) {}
    fn on_mouse_leave(&self, _interface: AppInterface) {}
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
