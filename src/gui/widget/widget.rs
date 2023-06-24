use rust_graphics::vec::Vec2;

use super::widget_instance::WidgetInstance;

pub trait Widget {
    fn build(&self, size: Vec2);
    fn children(&self) -> &[WidgetInstance] {
        &[]
    }
}

pub trait ToInstance {
    fn into_item(self) -> WidgetInstance;
}

impl<T> ToInstance for T
where
    T: Widget + 'static,
{
    fn into_item(self) -> WidgetInstance {
        WidgetInstance::new(self)
    }
}
