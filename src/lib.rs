use prelude::WidgetInstance;

pub mod actions;
pub mod error;
pub mod gui;
pub mod prelude;

pub fn print_widget_tree(widget: &WidgetInstance, indent: usize) {
    let indent_str = "\t".repeat(indent);
    println!("{}{}", indent_str, widget.type_name());
    for child in widget.widget().children() {
        print_widget_tree(child, indent + 1);
    }
}
