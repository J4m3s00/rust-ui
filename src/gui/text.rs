use rust_graphics::{
    color::{Color, COLOR_BLACK},
    font::Font,
};

#[derive(Clone)]
pub enum TextAlignV {
    Top,
    Center,
    Bottom,
}

#[derive(Clone)]
pub enum TextAlignH {
    Left,
    Center,
    Right,
}

#[derive(Clone)]
pub struct Text {
    pub text: String,
    pub font: Option<Font>,
    pub color: Color,
    pub alignment_h: TextAlignH,
    pub alignment_v: TextAlignV,
}

impl Default for Text {
    fn default() -> Self {
        Self {
            text: String::new(),
            font: None,
            color: COLOR_BLACK,
            alignment_h: TextAlignH::Center,
            alignment_v: TextAlignV::Center,
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
    pub fn vert_align(mut self, alignment: TextAlignV) -> Self {
        self.alignment_v = alignment;
        self
    }

    pub fn hor_align(mut self, alignment: TextAlignH) -> Self {
        self.alignment_h = alignment;
        self
    }
}
