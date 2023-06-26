use rust_graphics::draw_command::{Fill, Stroke};

use crate::gui::widget::state::observable::Observer;

use super::{relative_size::RelativeSize, text::Text};

pub struct WidgetRenderRect {
    pub width: Observer<RelativeSize>,
    pub height: Observer<RelativeSize>,
    pub fill: Observer<Option<Fill>>,
    pub stroke: Observer<Option<Stroke>>,
}

pub enum WidgetRenderItem {
    Text(Observer<Text>),
    Rect(WidgetRenderRect),
}

#[derive(Default)]
pub struct BuildResult {
    render_items: Vec<WidgetRenderItem>,
}

impl BuildResult {
    pub fn with_text(mut self, text: Observer<Text>) -> Self {
        self.render_items.push(WidgetRenderItem::Text(text));
        self
    }

    pub fn with_render_item(mut self, item: WidgetRenderItem) -> Self {
        self.render_items.push(item);
        self
    }

    pub fn render_items(&self) -> &[WidgetRenderItem] {
        &self.render_items
    }
}
