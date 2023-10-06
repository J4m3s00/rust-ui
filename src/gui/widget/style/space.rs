use rust_graphics::vec::Vec2;

use crate::prelude::Rect;

/// Used for Padding and Margin
/// Space between the content and the border
pub type Margin = ElementSpace;
pub type Padding = ElementSpace;
#[derive(Clone, Debug)]
pub struct ElementSpace {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl ElementSpace {
    pub fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    pub fn new_axis(horizontal: f32, vertical: f32) -> Self {
        Self {
            left: horizontal,
            right: horizontal,
            top: vertical,
            bottom: vertical,
        }
    }

    pub fn new_horizontal(horizontal: f32) -> Self {
        Self {
            left: horizontal,
            right: horizontal,
            top: 0.0,
            bottom: 0.0,
        }
    }

    pub fn new_vertical(vertical: f32) -> Self {
        Self {
            left: 0.0,
            right: 0.0,
            top: vertical,
            bottom: vertical,
        }
    }

    pub fn new_all(value: f32) -> Self {
        Self {
            left: value,
            right: value,
            top: value,
            bottom: value,
        }
    }

    pub fn zero() -> Self {
        Self::new_all(0.0)
    }

    pub fn horizontal(&self) -> f32 {
        self.left + self.right
    }

    pub fn vertical(&self) -> f32 {
        self.top + self.bottom
    }

    pub fn axis(&self) -> Vec2 {
        Vec2::new(self.horizontal(), self.vertical())
    }
}
