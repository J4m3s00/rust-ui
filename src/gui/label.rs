use rust_graphics::vec::Vec2;

use super::{
    text::{Text, TextAlignH, TextAlignV},
    widget::Widget,
    widget_builder::WidgetBuilder,
};

pub struct Label {
    text: Text,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: Text::from(text.into()),
        }
    }

    pub fn vert_align(mut self, alignment: TextAlignV) -> Self {
        self.text.alignment_v = alignment;
        self
    }

    pub fn hor_align(mut self, alignment: TextAlignH) -> Self {
        self.text.alignment_h = alignment;
        self
    }
}

impl Widget for Label {
    fn build(&self, builder: &mut WidgetBuilder, size: Vec2) {
        builder.new_child(size).text(self.text.clone());
    }
}
