use rust_graphics::vec::Vec2;
use rust_ui::{
    error::Result,
    gui::{
        button::Button,
        hstack::HStack,
        vstack::VStack,
        widget::{SizePolicy, SizePolicy2D, ToItem, Widget},
        widget_builder::WidgetBuilder,
    },
    UIApp,
};

struct EmptyWidget;

impl Widget for EmptyWidget {
    fn build(&self, build: &mut WidgetBuilder, size: Vec2) {
        build.new_child(size);
    }

    fn calc_min_size(&self, _size: SizePolicy2D) -> Vec2 {
        Vec2::zero()
    }
}

fn main_container() -> VStack {
    VStack::new(vec![
        EmptyWidget
            .into_item()
            .set_size((Default::default(), Default::default()).into()),
        HStack::new(vec![
            EmptyWidget
                .into_item()
                .set_size((SizePolicy::Fraction(1.), SizePolicy::default()).into()),
            VStack::new(vec![
                EmptyWidget
                    .into_item()
                    .set_size((SizePolicy::default(), SizePolicy::Fraction(1.)).into()),
                EmptyWidget
                    .into_item()
                    .set_size((SizePolicy::default(), SizePolicy::Fixed(20.)).into()),
            ])
            .into_item()
            .set_size((SizePolicy::Fraction(2.), SizePolicy::default()).into()),
        ])
        .into_item()
        .set_size((Default::default(), SizePolicy::Fixed(250.)).into()),
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
