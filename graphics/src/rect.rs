pub struct Rect {
    pub top: f32,
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Rect {
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            top,
            left,
            right,
            bottom,
        }
    }

    pub fn new_from_xy(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            top: y + height,
            left: x,
            right: x + width,
            bottom: y,
        }
    }
}
