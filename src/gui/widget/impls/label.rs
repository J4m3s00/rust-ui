use rust_graphics::vec::Vec2;

use crate::{
    gui::text::Text,
    prelude::{AlignH, AlignV, Widget},
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

    pub fn vert_align(mut self, alignment: AlignV) -> Self {
        self.text.alignment_v = alignment;
        self
    }

    pub fn hor_align(mut self, alignment: AlignH) -> Self {
        self.text.alignment_h = alignment;
        self
    }
}

impl Widget for Label {
    fn build(&self, size: Vec2) {
        unimplemented!()
    }
}
