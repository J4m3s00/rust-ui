#[derive(Clone, Debug)]
pub struct Style {
    pub padding: Padding,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            padding: Padding::all(2.),
        }
    }
}

/// Padding
/// Space between the content and the border
#[derive(Clone, Debug)]
pub struct Padding {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Padding {
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
