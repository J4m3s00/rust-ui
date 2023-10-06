use rust_graphics::vec::Vec2;

use crate::gui::widget::style::space::ElementSpace;

/// A rectangle.
/// Position is the top left corner
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rect {
    position: Vec2,
    size: Vec2,
}

impl Rect {
    pub fn new(position: impl Into<Vec2>, size: impl Into<Vec2>) -> Self {
        Rect {
            position: position.into(),
            size: size.into(),
        }
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn left(&self) -> f32 {
        self.position.x
    }

    pub fn right(&self) -> f32 {
        self.position.x + self.size.x
    }

    pub fn top(&self) -> f32 {
        self.position.y
    }

    pub fn bottom(&self) -> f32 {
        self.position.y + self.size.y
    }

    pub fn center(&self) -> Vec2 {
        self.position + self.size / 2.
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }

    pub fn width(&self) -> f32 {
        self.size.x
    }

    pub fn height(&self) -> f32 {
        self.size.y
    }

    pub fn contains(&self, position: impl Into<Vec2>) -> bool {
        let position = position.into();
        let end = self.position + self.size;
        position.x >= self.position.x
            && position.x <= end.x
            && position.y >= self.position.y
            && position.y <= end.y
    }

    pub fn set_position(&mut self, position: impl Into<Vec2>) {
        self.position = position.into();
    }

    /// Clones self and returns a new rect with the given offset.
    pub fn with_offset(&self, offset: impl Into<Vec2>) -> Self {
        Self {
            size: self.size,
            position: self.position + offset.into(),
        }
    }

    pub fn set_size(&mut self, size: impl Into<Vec2>) {
        self.size = size.into();
    }

    pub fn set_width(&mut self, width: f32) {
        self.size.x = width;
    }

    pub fn set_height(&mut self, height: f32) {
        self.size.y = height;
    }

    pub fn apply_space(&mut self, space: &ElementSpace) {
        self.position.x += space.left;
        self.position.y += space.top;
        self.size.x -= space.horizontal();
        self.size.y -= space.vertical();
    }
}
