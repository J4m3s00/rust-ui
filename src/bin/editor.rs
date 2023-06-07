use rust_ui::{error::Result, gui::label::Label, UIApp};

fn main() -> Result<()> {
    UIApp::new()
        .main_container(|container| {
            container.add_child(Label::new("Hello, world!"))?;
            container.add_child(Label::new("Hello, world2!"))?;
            Ok(())
        })
        .start()?;
    Ok(())
}
