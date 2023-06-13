use rust_graphics::vec::Vec2;

use super::{widget::Widget, widget_builder::WidgetBuilder};

pub struct Label {
    text: String,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl Widget for Label {
    fn build(&self, builder: &mut WidgetBuilder, size: Vec2) {
        builder.new_child(size).text(self.text.clone());
    }
}
