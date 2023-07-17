use rust_ui::prelude::*;

fn footer() -> WidgetInstance {
    HStack::new(vec![
        Button::new("Save", |_, _| {
            println!("Save");
        }),
        Button::new("Save As", |_, _| {
            println!("Save As");
        }),
        Button::new("Open", |_, _| {
            println!("Open");
        }),
    ])
    .set_height(SizePolicy::Fixed(24.))
}

fn side_bar() -> WidgetInstance {
    VStack::new(vec![
        IconButton::new(IconType::Home, |_, _| {}).set_height(SizePolicy::PercentageH(1.)),
        IconButton::new(IconType::Home, |_, _| {}).set_height(SizePolicy::PercentageH(1.)),
        IconButton::new(IconType::Home, |_, _| {}).set_height(SizePolicy::PercentageH(1.)),
        IconButton::new(IconType::Home, |_, _| {}).set_height(SizePolicy::PercentageH(1.)),
    ])
    .set_width(SizePolicy::Fixed(56.))
}

fn main_container() -> WidgetInstance {
    VStack::new(vec![HStack::new(vec![side_bar()]), footer()])
}

fn main() {
    UIApp::new().main_container(main_container()).run();
}
