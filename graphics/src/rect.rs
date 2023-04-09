#[derive(Default, Clone, Copy, Debug)]
pub struct Rect {
    pub top: f32,
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub origin_x: f32,
    pub origin_y: f32,
}

impl Rect {
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        assert!(left < right, "left must be less than right");
        assert!(bottom < top, "bottom must be less than top");
        Self {
            top,
            left,
            right,
            bottom,
            origin_x: (right - left) / 2.0,
            origin_y: (top - bottom) / 2.0,
        }
    }

    pub fn new_from_xy(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            top: y + height,
            left: x,
            right: x + width,
            bottom: y,
            origin_x: width / 2.0,
            origin_y: height / 2.0,
        }
    }
}
