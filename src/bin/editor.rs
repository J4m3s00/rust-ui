use rust_ui::{error::Result, gui::button::Button, UIApp};

fn main() -> Result<()> {
    UIApp::new()
        .main_container(|container| {
            container.add_child(Button::new("My Button"))?;
            Ok(())
        })
        .start()?;
    Ok(())
}
