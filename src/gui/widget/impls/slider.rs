use crate::prelude::{ToInstance, Widget, WidgetInstance};

pub struct Slider {}

impl Slider {
    pub fn new() -> WidgetInstance {
        Self {}.instance()
    }
}

impl Widget for Slider {}
