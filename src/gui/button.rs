use super::{
    widget::{SizePolicy2D, Widget},
    widget_builder::{WidgetBuilder, WidgetInteractionType},
};

pub struct Button {
    label: String,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }
}

impl Widget for Button {
    fn build(&self, builder: &mut WidgetBuilder, size: SizePolicy2D) {
        builder
            .child(size)
            .text(self.label.clone())
            .interaction(WidgetInteractionType::Click);
    }
}
