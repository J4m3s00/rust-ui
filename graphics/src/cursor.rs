use crate::{get_current_font_size, vec::Vec2};

#[derive(Clone)]
pub enum CursorDirection {
    Down,
    Right,
}

impl CursorDirection {
    pub fn get_default_step(&self) -> Vec2 {
        match self {
            Self::Down => Vec2::new(0.0, -get_current_font_size()),
            Self::Right => Vec2::new(get_current_font_size(), 0.0),
        }
    }
}

#[derive(Clone)]
pub struct Cursor {
    pub position: Vec2,
    pub direction: CursorDirection,
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            position: Vec2::default(),
            direction: CursorDirection::Down,
        }
    }
}
