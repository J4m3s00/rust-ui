use crate::{circle::Circle, rect::Rect, text::Text};

pub enum DrawCommand {
    Rect(Rect),
    Circle(Circle),
    Line { x1: f32, y1: f32, x2: f32, y2: f32 },
    Text(Text),
}
