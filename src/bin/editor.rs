use rust_graphics::vec::Vec2;
use rust_ui::{
    error::Result,
    gui::{
        hstack::HStack,
        vstack::VStack,
        widget::{SizePolicy, ToItem, Widget},
        widget_builder::WidgetBuilder, label::Label,
    },
    UIApp,
};

struct TestWidget {
    stack: HStack,
}

impl TestWidget {
    fn new() -> Self {
        Self {
            stack: HStack::new(vec![
                Label::new("Text").into_item(),
                VStack::new(vec![EmptyWidget.into_item(), EmptyWidget.into_item()])
                    .into_item()
                    .set_width(SizePolicy::PercentageV(0.5)),
            ]),
        }
    }
}

impl Widget for TestWidget {
    fn build(&self, builder: &mut WidgetBuilder, size: Vec2) {
        builder.new_child(size).widget(&self.stack, size);
    }
}
struct EmptyWidget;

impl Widget for EmptyWidget {
    fn build(&self, build: &mut WidgetBuilder, size: Vec2) {
        build.new_child(size);
    }
}

fn main_container() -> impl Widget {
    VStack::new(vec![
        EmptyWidget.into_item(),
        HStack::new(vec![
            EmptyWidget.into_item(),
            TestWidget::new()
                .into_item()
                .set_width(SizePolicy::Fixed(128.)),
            EmptyWidget.into_item(),
        ])
        .into_item()
        .set_height(SizePolicy::Fixed(32.)),
        EmptyWidget.into_item(),
    ])
}

#[allow(dead_code)]
fn main_just_button() -> TestWidget {
    TestWidget::new()
}

fn main() -> Result<()> {
    UIApp::new().main_container(main_container).start()?;
    Ok(())
}
