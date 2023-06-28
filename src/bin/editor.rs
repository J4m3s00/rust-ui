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
    Slider::new(State::new(50.)).set_height(SizePolicy::Fixed(32.))
}

fn sidebar() -> WidgetInstance {
    VStack::new(vec![slider(), slider(), slider(), slider(), slider()])
        .set_width(SizePolicy::Fixed(250.0))
}

fn main_container() -> WidgetInstance {
    VStack::new(vec![menubar(), HStack::new(vec![sidebar()])])
}

fn main() {
    UIApp::new().main_container(main_container()).run();
}
