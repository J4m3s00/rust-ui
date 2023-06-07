use super::{
    widget::{SizePolicy2D, Widget},
    widget_builder::{PushChild, WidgetInteractionType},
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
    fn build(&self, builder: &PushChild, size: SizePolicy2D) {
        builder
            .push_child(size)
            .text(self.label.clone())
            .interaction(WidgetInteractionType::Click)
            .pop_child();
    }
}
