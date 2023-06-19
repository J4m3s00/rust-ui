use rust_graphics::vec::Vec2;

use super::{
    events::action::Action,
    text::Text,
    widget::Widget,
    widget_builder::{interactions::Click, WidgetBuilder},
};

pub struct ButtonClick;

pub struct Button {
    label: String,
    on_click: Option<Box<dyn Action<ButtonClick>>>,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            on_click: None,
        }
    }

    pub fn on_click<T>(mut self, action: T) -> Self
    where
        T: Action<ButtonClick> + 'static,
    {
        self.on_click = Some(Box::new(action));
        self
    }
}

impl Widget for Button {
    fn build(&self, builder: &mut WidgetBuilder, size: Vec2) {
        builder
            .new_child(size)
            .text(Text::from(self.label.clone()))
            .interaction::<Click, _>(|c| println!("Got click event on some Button! {:?}", c));
    }
}
