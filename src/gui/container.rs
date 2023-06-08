use super::{
    widget::{SizePolicy, SizePolicy2D, Widget},
    widget_builder::WidgetBuilder,
};

pub struct ContainerItem {
    widget: Box<dyn Widget>,
    size: SizePolicy2D,
}

impl ContainerItem {
    pub fn new<T>(widget: T) -> Self
    where
        T: Widget + 'static,
    {
        Self {
            widget: Box::new(widget),
            size: SizePolicy::Percentage(1.).into(),
        }
    }

    pub fn set_size(mut self, size: SizePolicy2D) -> Self {
        self.size = size;
        self
    }

    pub fn widget(&self) -> &dyn Widget {
        &*self.widget
    }
}

pub trait Container: Widget {}
