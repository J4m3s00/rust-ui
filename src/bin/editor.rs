use rust_ui::{
    gui::widget::impls::{
        checkbox::Checkbox,
        clickable::Clicked,
        icon::{Icon, IconType},
    },
    prelude::*,
};

// Functional component
fn menubar() -> WidgetInstance {
    HStack::new(vec![Button::new("x", |_, inter: AppInterface| {
        inter.quit()
    })
    .set_width(SizePolicy::PercentageV(1.))])
    .set_height(SizePolicy::Fixed(30.0))
}

fn sidebar() -> WidgetInstance {
    VStack::new(vec![
        Slider::new(State::new(50.)).set_height(SizePolicy::Fixed(32.)),
        Checkbox::new(Text::from("My Checkbox"), State::new(false))
            .set_height(SizePolicy::Fixed(32.)),
        Button::new("Button", |click: Clicked, inter: AppInterface| {
            inter.open_panel(Button::new("Hello World!", |_, _| {}), click.0.absolute_pos);
        })
        .set_height(SizePolicy::Fixed(32.)),
        Button::new("Test Button", |_, _| {}).set_height(SizePolicy::Fixed(32.)),
    ])
    .set_width(SizePolicy::Fixed(250.0))
}

fn main_container() -> WidgetInstance {
    VStack::new(vec![
        menubar(),
        HStack::new(vec![sidebar(), Icon::new(IconType::Refresh)]),
    ])
}

fn main() {
    UIApp::new().main_container(main_container()).run();
}
