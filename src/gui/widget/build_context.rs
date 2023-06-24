use rust_graphics::{rect::Rect, vec::Vec2};

enum CursorDirection {
    Horizontal,
    Vertical,
}

struct Cursor {
    pos: Vec2,
    direction: CursorDirection,
}

pub struct BuildContext {
    content_rect: Rect,
    cursor: Cursor,
}

impl BuildContext {
    pub fn new(content_rect: Rect) -> Self {
        Self {
            content_rect,
            cursor: Cursor {
                direction: CursorDirection::Vertical,
                pos: content_rect.top_left(),
            },
        }
    }

    pub fn allocate_space(&mut self, size: Vec2) -> Option<Rect> {
        unimplemented!()
    }
}
