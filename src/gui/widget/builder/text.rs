use rust_graphics::{
    color::{Color, COLOR_BLACK},
    font::Font,
};

use super::align::{AlignH, AlignV};

#[derive(Clone, Debug)]
pub struct Text {
    pub text: String,
    pub font: Option<Font>,
    pub color: Color,
    pub alignment_h: AlignH,
    pub alignment_v: AlignV,
}

impl Default for Text {
    fn default() -> Self {
        Self {
            text: String::new(),
            font: None,
            color: COLOR_BLACK,
            alignment_h: AlignH::Center,
            alignment_v: AlignV::Center,
        }
    }
}

impl<T> From<T> for Text
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self {
            text: value.into(),
            ..Default::default()
        }
    }
}

impl Text {
    pub fn vert_align(mut self, alignment: AlignV) -> Self {
        self.alignment_v = alignment;
        self
    }

    pub fn hor_align(mut self, alignment: AlignH) -> Self {
        self.alignment_h = alignment;
        self
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
    }
}
