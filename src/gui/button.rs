use rust_graphics::vec::Vec2;

use super::{
    widget::{SizePolicy2D, Widget},
    widget_builder::WidgetBuilder,
};

pub struct Button {
    label: String,
}

impl Widget for Button {
    fn build(&self, builder: &mut WidgetBuilder, size: SizePolicy2D) {
        builder.text(self.label.clone());
    }
}
