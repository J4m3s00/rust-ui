use crate::prelude::{ToInstance, Widget, WidgetInstance};

pub struct Spacer;
impl Widget for Spacer {}
impl Spacer {
    pub fn new() -> WidgetInstance {
        Self.instance()
    }
}
