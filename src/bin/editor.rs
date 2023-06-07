use rust_ui::{
    error::Result,
    gui::{button::Button, label::Label, widget::SizePolicy},
    UIApp,
};

fn main() -> Result<()> {
    UIApp::new()
        .main_container(|container| {
            container
                .add_child(Button::new("My Button"))
                .set_size((SizePolicy::Percentage(1.0), SizePolicy::Fixed(20.)).into());
            container.add_child(Label::new("My Label"));
            Ok(())
        })
        .start()?;
    Ok(())
}
