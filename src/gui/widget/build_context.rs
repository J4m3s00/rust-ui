use rust_graphics::{rect::Rect, vec::Vec2};

const PADDING: f32 = 5.;

#[derive(Copy, Clone, Debug)]
pub enum CursorDirection {
    Horizontal,
    Vertical,
    Stacked,
}

#[derive(Clone, Debug)]
struct Cursor {
    pos: Vec2,
    direction: CursorDirection,
}

#[derive(Clone, Debug)]
pub struct BuildContext {
    content_rect: Rect,
    cursor: Cursor,
}

impl BuildContext {
    pub fn new(content_rect: Rect, cursor_direction: CursorDirection) -> Self {
        Self {
            content_rect,
            cursor: Cursor {
                direction: cursor_direction,
                pos: content_rect.top_left(),
            },
        }
    }

    pub fn allocate_space(&mut self, size: Vec2) -> Option<BuildContext> {
        let content_area = Rect::new_from_xy(
            self.cursor.pos.x + PADDING,
            self.cursor.pos.y + PADDING,
            size.x - (PADDING * 2.),
            size.y - (PADDING * 2.),
        );
        let advance = match self.cursor.direction {
            CursorDirection::Horizontal => (size.x, 0.).into(),
            CursorDirection::Vertical => (0., size.y).into(),
            CursorDirection::Stacked => (0., 0.).into(),
        };
        self.cursor.pos += advance;
        Some(Self::new(content_area, self.cursor.direction))
    }

    pub fn get_content_size(&self) -> Vec2 {
        self.content_rect.size()
    }

    pub fn get_content_rect(&self) -> &Rect {
        &self.content_rect
    }

    pub fn set_cursor_direction(&mut self, direction: CursorDirection) {
        self.cursor.direction = direction;
    }
}
