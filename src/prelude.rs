pub use crate::gui::app::{app::UIApp, interface::AppInterface};
pub use crate::{
    actions::receiver::Receiver,
    error::Result,
    gui::widget::{
        builder::{
            align::{AlignH, AlignV},
            relative_size::RelativeSize,
            text::Text,
        },
        impls::button::Button,
        impls::hstack::HStack,
        impls::label::Label,
        impls::slider::Slider,
        impls::spacer::Spacer,
        impls::vstack::VStack,
        size_policy::SizePolicy,
        widget::{ToInstance, Widget},
        widget_instance::WidgetInstance,
    },
};
pub use rust_graphics::app::App;
pub use rust_graphics::vec::*;
