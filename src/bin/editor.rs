use rust_graphics::app::App;
use rust_ui::{
    error::Result,
    gui::{
        button::Button,
        hstack::HStack,
        label::Label,
        text::{TextAlignH, TextAlignV},
        vstack::VStack,
        widget::{SizePolicy, ToItem, Widget},
    },
    UIApp,
};

fn main_screen() -> impl Widget {
    VStack::new(vec![
        HStack::new(vec![
            Label::new("Header").into_item(),
            Button::new("Click")
                .into_item()
                .set_width(SizePolicy::Fixed(64.)),
        ])
        .into_item()
        .set_height(SizePolicy::Fixed(32.)),
        HStack::new(vec![
            Label::new("Sidebar")
                .hor_align(TextAlignH::Left)
                .vert_align(TextAlignV::Top)
                .into_item()
                .set_width(SizePolicy::Fixed(256.)),
            VStack::new(vec![]).into_item(),
        ])
        .into_item(),
    ])
}

fn main() -> Result<()> {
    UIApp::new().main_container(main_screen).run();
    Ok(())
}
