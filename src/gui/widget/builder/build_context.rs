use rust_graphics::{rect::Rect, vec::Vec2};

use crate::gui::widget::{style::Style, theme::Theme};

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
    current_theme: Theme,
}

impl BuildContext {
    pub fn new(content_rect: Rect, cursor_direction: CursorDirection) -> Self {
        Self {
            content_rect,
            cursor: Cursor {
                direction: cursor_direction,
                pos: content_rect.top_left(),
            },
            current_style: Style::default(),
            current_theme: Theme::default(),
        }
    }

    /// This allocates space in the current container. If the container is full, it will return None.
    /// Otherwise, it will return a new BuildContext with the allocated space. The new BuildContext
    /// will have the same cursor direction as the parent. The cursor will be advanced by the size
    /// of the allocated space.
    pub fn allocate_space(&mut self, size: impl Into<Vec2>) -> Option<BuildContext> {
        let size = size.into();
        let content_area = Rect::new_from_xy(
            self.cursor.pos.x + self.current_style.padding.left,
            self.cursor.pos.y + self.current_style.padding.top,
            size.x - (self.current_style.padding.left + self.current_style.padding.right),
            size.y - (self.current_style.padding.top + self.current_style.padding.bottom),
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

    pub fn theme(&self) -> &Theme {
        &self.current_theme
    }
}
