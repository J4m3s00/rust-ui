use rust_ui::prelude::*;

// Functional component
fn menubar() -> WidgetInstance {
    HStack::new(vec![Button::new("x", |_, inter: AppInterface| {
        inter.quit()
    })
    .set_width(SizePolicy::Relative(RelativeSize::PercentageV(1.)))])
    .set_height(SizePolicy::Fixed(30.0))
}

fn sidebar() -> WidgetInstance {
    VStack::new(vec![Slider::new(10.).set_height(SizePolicy::Fixed(32.))])
        .set_width(SizePolicy::Fixed(250.0))
}

fn main_container() -> WidgetInstance {
    VStack::new(vec![menubar(), HStack::new(vec![sidebar()])])
}

fn main() {
    UIApp::new().main_container(main_container()).run();
}
