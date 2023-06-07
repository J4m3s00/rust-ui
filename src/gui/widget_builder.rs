use rust_graphics::{rect::Rect, vec::Vec2};

pub struct WidgetBuilder {
    texts: Vec<(String, Vec2)>,
    click_areas: Vec<Rect>,
    cursor_pos: Vec2,
}

impl Default for WidgetBuilder {
    fn default() -> Self {
        Self {
            texts: Vec::new(),
            click_areas: Vec::new(),
            cursor_pos: (100., 100.).into(),
        }
    }
}

impl WidgetBuilder {
    pub fn push(&mut self, other: WidgetBuilder) {
        self.texts.extend(other.texts);
        self.click_areas.extend(other.click_areas);
    }

    pub fn text(&mut self, text: impl Into<String>) {
        self.texts.push((text.into(), self.cursor_pos));
    }

    pub fn advance(&mut self) {
        self.cursor_pos.y += 20.;
    }

    pub fn texts(&self) -> &Vec<(String, Vec2)> {
        &self.texts
    }
}
