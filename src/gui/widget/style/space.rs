use rust_graphics::rect::Rect;

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

    pub fn axis(horizontal: f32, vertical: f32) -> Self {
        Self {
            left: horizontal,
            right: horizontal,
            top: vertical,
            bottom: vertical,
        }
    }

    pub fn horizontal(horizontal: f32) -> Self {
        Self {
            left: horizontal,
            right: horizontal,
            top: 0.0,
            bottom: 0.0,
        }
    }

    pub fn vertical(vertical: f32) -> Self {
        Self {
            left: 0.0,
            right: 0.0,
            top: vertical,
            bottom: vertical,
        }
    }

    pub fn all(value: f32) -> Self {
        Self {
            left: value,
            right: value,
            top: value,
            bottom: value,
        }
    }

    pub fn zero() -> Self {
        Self::all(0.0)
    }
}

pub trait ApplySpace {
    fn apply_space(&mut self, space: &ElementSpace);
}

impl ApplySpace for Rect {
    fn apply_space(&mut self, space: &ElementSpace) {
        self.left += space.left;
        self.top += space.top;
        self.right -= space.right;
        self.bottom -= space.bottom;
    }
}
