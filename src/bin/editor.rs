use rust_ui::{
    error::Result,
    gui::{
        button::Button,
        container::{Container, ContainerItem},
        label::Label,
        widget::SizePolicy,
    },
    UIApp,
};

fn main_container() -> Container {
    Container {
        children: vec![
            ContainerItem::new(Label::new("My Label"))
                .set_size((SizePolicy::Percentage(1.0), SizePolicy::Fixed(20.)).into()),
            ContainerItem::new(Button::new("My Button"))
                .set_size((SizePolicy::Percentage(1.0), SizePolicy::Fixed(20.)).into()),
        ],
    }
}

fn main_just_button() -> Button {
    Button::new("My Button")
}

fn main() -> Result<()> {
    UIApp::new().main_container(main_container).start()?;
    Ok(())
}
