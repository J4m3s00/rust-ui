use rust_graphics::vec::Vec2;

use super::{
    container::ContainerItem,
    widget::{SizePolicy, SizePolicy2D, Widget},
    widget_builder::{CursorDirection, WidgetBuilder},
};

pub struct HStack {
    children: Vec<ContainerItem>,
}

impl HStack {
    pub fn new(children: Vec<ContainerItem>) -> Self {
        Self { children }
    }

    fn children(&self) -> &[ContainerItem] {
        &self.children
    }
}

impl Widget for HStack {
    fn build(&self, builder: &mut WidgetBuilder, content_area: Vec2) {
        let mut remaining_width = content_area.x;
        let mut total_frac = 0.;
        for item in self.children().iter() {
            match item.size().horizontal {
                SizePolicy::Fixed(pixels) => {
                    remaining_width -= pixels;
                }
                SizePolicy::Percentage(percent) => {
                    remaining_width -= percent * content_area.x;
                }
                SizePolicy::Fraction(frac) => {
                    total_frac += frac;
                }
            }
        }

        let frac_width = remaining_width / total_frac;

        let mut child = builder
            .new_child(content_area)
            .set_cursor_direction(CursorDirection::Horizontal);
        for item in self.children().iter() {
            let width = match item.size().horizontal {
                SizePolicy::Fixed(pixels) => pixels,
                SizePolicy::Percentage(percent) => percent * content_area.x,
                SizePolicy::Fraction(frac) => frac * frac_width,
            };
            child = child.widget(item.widget(), (width, content_area.y).into());
        }
    }

    fn calc_min_size(&self, size: SizePolicy2D) -> Vec2 {
        let mut min_size = Vec2::zero();
        for child in self.children() {
            let child_size_px = child.widget().calc_min_size(child.size());
            min_size.x = min_size.x.max(child_size_px.x);
            min_size.y += child_size_px.y;
        }
        min_size
    }
}
