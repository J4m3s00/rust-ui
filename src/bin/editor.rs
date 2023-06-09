use rust_ui::{
    error::Result,
    gui::{
        button::Button,
        container::ContainerItem,
        label::Label,
        vstack::{ContainerWidgetItem, VStack},
        widget::SizePolicy,
    },
    UIApp,
};

fn main_container() -> VStack {
    VStack::new(vec![
        ContainerWidgetItem::new(Label::new("My Label"))
            .set_size((SizePolicy::Percentage(1.0), SizePolicy::Fixed(20.)).into()),
        ContainerWidgetItem::new(Button::new("My Button"))
            .set_size((SizePolicy::Percentage(1.0), SizePolicy::Fixed(20.)).into()),
    ])
}

#[allow(dead_code)]
fn main_just_button() -> Button {
    Button::new("My Button")
}

fn main() -> Result<()> {
    UIApp::new().main_container(main_container).start()?;
    Ok(())
}
