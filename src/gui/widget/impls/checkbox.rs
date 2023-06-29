use crate::{
    gui::widget::{state::observable::Observer, style::space::Padding},
    prelude::{ColorId, HStack, Label, SizePolicy, State, Text, WidgetInstance},
};

use super::{clickable::Clickable, rectangle::Rectangle, zstack::ZStack};

pub struct Checkbox;

impl Checkbox {
    pub fn new(label: impl Into<Observer<Text>>, checked: State<bool>) -> WidgetInstance {
        HStack::new(vec![Label::new_observe(label), Self::click_area(checked)])
    }

    fn click_area(checked: State<bool>) -> WidgetInstance {
        ZStack::new(vec![
            Rectangle::fill(ColorId::PrimaryVariant).set_padding(Padding::all(10.)),
            Clickable::new(move |_, _| {
                checked.set(!checked.get());
            }),
        ])
        .set_width(SizePolicy::PercentageV(1.))
    }
}
