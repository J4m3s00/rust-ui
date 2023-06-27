use crate::{
    actions::receiver::Receiver,
    gui::widget::{state::observable::Observer, widget_instance::WidgetInstance},
    prelude::{AlignH, AlignV, Label, Text},
};

use super::{
    clickable::{Clickable, Clicked},
    zstack::ZStack,
};

pub struct Button;

impl Button {
    pub fn new<T>(label: impl Into<String>, on_click: T) -> WidgetInstance
    where
        T: Receiver<Clicked> + 'static,
    {
        ZStack::new(vec![
            Clickable::new(on_click),
            Label::new_observe(Observer::from(
                Text::from(label)
                    .hor_align(AlignH::Center)
                    .vert_align(AlignV::Center),
            )),
        ])
    }
}
