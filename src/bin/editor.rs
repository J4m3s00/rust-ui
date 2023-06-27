use rust_ui::prelude::*;

// Functional component
fn menubar() -> WidgetInstance {
    HStack::new(vec![Button::new("x", |_, inter: AppInterface| {
        inter.quit()
    })
    .set_width(SizePolicy::PercentageV(1.))])
    .set_height(SizePolicy::Fixed(30.0))
}

fn slider() -> WidgetInstance {
    let slider_value = State::new(0.0);
    HStack::new(vec![
        Label::new_observe(slider_value.map(|v| Text::from(format!("{:.0}", v))))
            .set_width(SizePolicy::Fixed(50.)),
        Slider::new(slider_value.clone()),
    ])
}

fn sidebar() -> WidgetInstance {
    VStack::new(vec![slider().set_height(SizePolicy::Fixed(32.))])
        .set_width(SizePolicy::Fixed(250.0))
}

fn main_container() -> WidgetInstance {
    VStack::new(vec![menubar(), HStack::new(vec![sidebar()])])
}

fn main() {
    UIApp::new().main_container(main_container()).run();
}
