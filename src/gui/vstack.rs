use rust_graphics::vec::Vec2;

use super::{
    container::ContainerItem,
    widget::{SizePolicy, Widget},
    widget_builder::{CursorDirection, WidgetBuilder},
};

pub struct VStack {
    children: Vec<ContainerItem>,
}

impl VStack {
    pub fn new(children: Vec<ContainerItem>) -> Self {
        Self { children }
    }

    fn children(&self) -> &[ContainerItem] {
        &self.children
    }
}

impl Widget for VStack {
    fn build(&self, builder: &mut WidgetBuilder, content_area: Vec2) {
        let mut remaining_height = content_area.y;
        let mut total_frac = 0.;
        for item in self.children().iter() {
            match item.size().vertical {
                SizePolicy::Fixed(pixels) => {
                    remaining_height -= pixels;
                }
                SizePolicy::Percentage(percent) => {
                    remaining_height -= percent * content_area.y;
                }
                SizePolicy::PercentageH(percent) => {
                    remaining_height -= percent * content_area.x;
                }
                SizePolicy::PercentageV(percent) => {
                    remaining_height -= percent * content_area.y;
                }
                SizePolicy::Fraction(frac) => {
                    total_frac += frac;
                }
            }
        }

        let frac_height = remaining_height / total_frac;

        let mut child = builder
            .new_child(content_area)
            .set_cursor_direction(CursorDirection::Vertical);
        for item in self.children().iter() {
            let height = match item.size().vertical {
                SizePolicy::Fixed(pixels) => pixels,
                SizePolicy::Percentage(percent) => percent * content_area.y,
                SizePolicy::PercentageH(percent) => percent * content_area.x,
                SizePolicy::PercentageV(percent) => percent * content_area.y,
                SizePolicy::Fraction(frac) => frac * frac_height,
            };
            child = child
                .widget(item.widget(), (content_area.x, height).into())
                .set_cursor_direction(CursorDirection::Vertical); // Reset if children changed them
        }
    }
}
