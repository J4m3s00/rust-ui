use rust_ui::{
    error::Result,
    gui::{button::Button, widget::SizePolicy},
    UIApp,
};

fn main() -> Result<()> {
    UIApp::new()
        .main_container(|container| {
            container
                .add_child(Button::new("My Button"))
                .set_size((SizePolicy::Fill, SizePolicy::Fixed(40.)).into());
            Ok(())
        })
        .start()?;
    Ok(())
}
