use crate::{
    gui::widget::{state::observable::Observer, style::space::Padding},
    prelude::{ColorId, HStack, Label, Receiver, SizePolicy, State, Text, WidgetInstance},
};

use super::{
    clickable::Clickable,
    icon::{Icon, IconType},
    rectangle::Rectangle,
    zstack::ZStack,
};

pub struct CheckboxChange {
    pub checked: bool,
}

pub struct Checkbox;

impl Checkbox {
    pub fn new(label: impl Into<Observer<Text>>, checked: State<bool>) -> WidgetInstance {
        HStack::new(vec![
            Label::new_observe(label),
            Self::click_area(checked, ()),
        ])
    }

    pub fn new_c<T>(
        label: impl Into<Observer<Text>>,
        checked: State<bool>,
        on_change: T,
    ) -> WidgetInstance
    where
        T: Receiver<CheckboxChange> + 'static,
    {
        HStack::new(vec![
            Label::new_observe(label),
            Self::click_area(checked, on_change),
        ])
    }

    fn click_area(
        checked: State<bool>,
        on_change: impl Receiver<CheckboxChange> + 'static,
    ) -> WidgetInstance {
        let observer = checked.observe();
        ZStack::new(vec![
            Rectangle::fill(ColorId::PrimaryVariantLight).set_padding(Padding::all(10.)),
            Clickable::new(move |_, i| {
                checked.set(!checked.get());
                on_change.action(
                    CheckboxChange {
                        checked: checked.get(),
                    },
                    i,
                );
            }),
            Icon::new(IconType::Checkmark).set_visible(observer),
        ])
        .set_width(SizePolicy::PercentageV(1.))
    }
}
