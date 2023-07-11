use rust_graphics::draw_command::{Fill, Stroke};

use crate::{
    gui::{
        svg::svg::Svg,
        widget::{
            rendering::drawable::{draw_svg::DrawSvg, item::DrawItem, rectangle::DrawRect},
            state::observable::Observer,
        },
    },
    prelude::{SizePolicy, Text},
};

#[derive(Default)]
pub struct WidgetRenderRect {
    pub width: Observer<SizePolicy>,
    pub height: Observer<SizePolicy>,
    pub fill: Observer<Option<Fill>>,
    pub stroke: Observer<Option<Stroke>>,
}

pub enum WidgetRenderItem {
    Text(Observer<Text>),
    Rect(WidgetRenderRect),
}

#[derive(Default)]
pub struct BuildResult {
    render_items: Vec<DrawItem>,
}

impl BuildResult {
    pub fn draw_text(&mut self, text: impl Into<Observer<Text>>) -> &mut DrawItem {
        self.push_item(DrawItem::new(text.into()))
    }

    pub fn draw_rect(&mut self, rect: DrawRect) -> &mut DrawItem {
        self.push_item(DrawItem::new(rect))
    }

    pub fn draw_svg(&mut self, svg: DrawSvg) -> &mut DrawItem {
        self.push_item(DrawItem::new(svg))
    }

    fn push_item(&mut self, item: DrawItem) -> &mut DrawItem {
        self.render_items.push(item);
        self.render_items.last_mut().unwrap()
    }

    pub fn render_items(&self) -> &[DrawItem] {
        &self.render_items
    }
}
