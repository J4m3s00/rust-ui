use rust_ui::{
    gui::widget::{
        impls::{checkbox::Checkbox, rectangle::Rectangle},
        style::space::Padding,
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
    let shared_state = State::new(50.);
    VStack::new(vec![
        Slider::new(shared_state.clone()).set_height(SizePolicy::Fixed(32.)),
        Slider::new(shared_state.clone()).set_height(SizePolicy::Fixed(32.)),
        Slider::new(shared_state.clone()).set_height(SizePolicy::Fixed(32.)),
        Slider::new(State::new(50.)).set_height(SizePolicy::Fixed(32.)),
        Slider::new(State::new(50.)).set_height(SizePolicy::Fixed(32.)),
        Checkbox::new(Text::from("My Checkbox"), State::new(false))
            .set_height(SizePolicy::Fixed(32.)),
    ])
    .set_width(SizePolicy::Fixed(250.0))
}

fn main_container() -> WidgetInstance {
    VStack::new(vec![
        menubar(),
        HStack::new(vec![
            sidebar(),
            Rectangle::fill(ColorId::SecondaryVariant).set_padding(Padding::all(10.)),
        ]),
    ])
}

fn main() {
    UIApp::new().main_container(main_container()).run();
}
