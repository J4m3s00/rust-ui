use crate::vec::Vec2;

pub struct Text {
    pub position: Vec2,
    pub text: String,
}

impl Text {
    pub fn new(position: impl Into<Vec2>, text: impl Into<String>) -> Self {
        Self {
            position: position.into(),
            text: text.into(),
        }
    }
}
