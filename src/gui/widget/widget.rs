use super::{
    build_context::BuildContext, build_results::BuildResult, widget_instance::WidgetInstance,
};

pub trait Widget {
    fn build(&mut self, ctx: &mut BuildContext) -> BuildResult;
    fn children(&self) -> &[WidgetInstance] {
        &[]
    }

    fn on_click(&self) {}
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
