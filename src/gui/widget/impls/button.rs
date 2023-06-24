use rust_graphics::vec::Vec2;

use crate::{
    actions::{receiver::Receiver, signal::Signal},
    prelude::Widget,
};

pub struct ButtonClick;

pub struct Button {
    label: String,
    on_click: Signal<ButtonClick>,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            on_click: Signal::default(),
        }
    }

    pub fn on_click<T>(mut self, receiver: Box<T>) -> Self
    where
        T: Receiver + 'static,
    {
        self.on_click.connect(receiver);
        self
    }
}

impl Widget for Button {
    fn build(&self, size: Vec2) {
        unimplemented!()
    }
}

impl Receiver for Button {
    //type Action = ButtonClick;

    fn action(self) {
        println!("Clicked!");
    }
}
