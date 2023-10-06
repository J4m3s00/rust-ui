use prelude::WidgetInstance;

pub mod actions;
pub mod error;
pub mod gui;
pub mod prelude;
pub mod rect;

pub fn print_widget_tree(widget: &WidgetInstance, indent: usize) {
    let indent_str = "\t".repeat(indent);
    println!("({}) -> {}{}", widget.id(), indent_str, widget.type_name());
    for child in widget.widget().children() {
        print_widget_tree(child, indent + 1);
    }
}

pub trait MapScalar {
    fn map(self, min: Self, max: Self, new_min: Self, new_max: Self) -> Self;
}

impl<T> MapScalar for T
where
    T: std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Add<Output = T>
        + Copy
        + From<f32>,
{
    fn map(self, min: Self, max: Self, new_min: Self, new_max: Self) -> Self {
        (self - min) / (max - min) * (new_max - new_min) + new_min
    }
}
