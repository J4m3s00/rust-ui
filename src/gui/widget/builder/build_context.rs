use rust_graphics::vec::Vec2;

use crate::prelude::{Rect, Style};

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

/// The BuildContext is passed to each widget's build function. It contains information about the
/// current container's size and position, and provides a way to allocate space for child widgets.
/// It also contains a cursor, which is used to track the current position of the childs in the container.
#[derive(Clone, Debug)]
pub struct BuildContext {
    content_rect: Rect,
    cursor: Cursor,
    current_style: Style,
}

impl BuildContext {
    pub fn new(content_rect: Rect, cursor_direction: CursorDirection) -> Self {
        Self {
            content_rect,
            cursor: Cursor {
                direction: cursor_direction,
                pos: content_rect.position(),
            },
            current_style: Style::default(),
        }
    }

    /// This allocates space in the current container. If the container is full, it will return None.
    /// Otherwise, it will return a new BuildContext with the allocated space. The new BuildContext
    /// will have the same cursor direction as the parent. The cursor will be advanced by the size
    /// of the allocated space.
    pub fn allocate_space(&mut self, size: impl Into<Vec2>) -> Option<BuildContext> {
        let size = size.into();
        let content_area = Rect::new(
            (
                self.cursor.pos.x + self.current_style.margin.left,
                self.cursor.pos.y + self.current_style.margin.top,
            ),
            (
                size.x - (self.current_style.margin.left + self.current_style.margin.right),
                size.y - (self.current_style.margin.top + self.current_style.margin.bottom),
            ),
        );
        let advance = match self.cursor.direction {
            CursorDirection::Horizontal => (size.x, 0.).into(),
            CursorDirection::Vertical => (0., size.y).into(),
            CursorDirection::Stacked => (0., 0.).into(),
        };
        self.cursor.pos += advance;
        Some(Self::new(content_area, self.cursor.direction))
    }

    pub fn set_style(&mut self, style: Style) {
        self.current_style = style;
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
