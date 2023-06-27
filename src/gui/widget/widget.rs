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

    fn on_mouse_down(&self) {}
    fn on_mouse_up(&self) {}
    fn on_mouse_enter(&self) {}
    fn on_mouse_leave(&self) {}
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
