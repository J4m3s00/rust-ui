use rust_ui::{
    error::Result,
    gui::{
        button::Button,
        container::{ContainerItem, ContainerWidgetItem},
        label::Label,
        vstack::VStack,
        widget::{SizePolicy, SizePolicy2D, Widget},
        widget_builder::WidgetBuilder,
    },
    UIApp,
};

struct EmptyWidget;

impl Widget for EmptyWidget {
    fn build(&self, build: &mut WidgetBuilder, size: SizePolicy2D) {
        build.child(size);
    }
}

fn main_container() -> VStack {
    VStack::new(vec![
        ContainerWidgetItem::new(EmptyWidget)
            .set_size((SizePolicy::Percentage(1.0), SizePolicy::Fixed(40.)).into()),
        ContainerWidgetItem::new(EmptyWidget)
            .set_size((SizePolicy::Percentage(1.0), SizePolicy::Fixed(40.)).into()),
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
