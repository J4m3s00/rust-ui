use rust_graphics::vec::Vec2;

use super::{
    widget::Widget,
    widget_builder::{WidgetBuilder, WidgetInteractionType},
};

pub struct Button {
    label: String,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }
}

impl Widget for Button {
    fn build(&self, builder: &mut WidgetBuilder, size: Vec2) {
        builder
            .new_child(size)
            .text(self.label.clone().into())
            .interaction(WidgetInteractionType::Click);
    }
}
