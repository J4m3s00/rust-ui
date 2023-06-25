use std::slice::from_ref;

use rust_ui::{
    gui::widget::{build_context::BuildContext, build_results::BuildResult, impls::zstack::ZStack},
    prelude::*,
};

struct EmptyWidget;
impl Widget for EmptyWidget {
    fn build(&mut self, _ctx: &mut BuildContext) -> BuildResult {
        BuildResult::default()
    }
}

fn main_container() -> WidgetInstance {
    // Center a label in the middle of the screen.
    VStack::new(vec![
        EmptyWidget.instance(),
        HStack::new(vec![
            ZStack::new(vec![
                EmptyWidget.instance(),
                EmptyWidget.instance(),
                EmptyWidget.instance(),
                EmptyWidget.instance(),
                EmptyWidget.instance(),
            ]),
            Button::new("Click me!", |_| {
                println!("Clicked a button on the Screen! YEAHHHH")
            })
            .set_width(SizePolicy::Fixed(128.)),
            TestWidget::new(),
        ])
        .set_height(SizePolicy::Fixed(256.)),
        EmptyWidget.instance(),
    ])
}

struct TestWidget {
    container: WidgetInstance,
}

impl TestWidget {
    fn new() -> WidgetInstance {
        Self {
            container: VStack::new(vec![
                Label::new("Hello, world!"),
                Label::new("Hello, world!"),
            ]),
        }
        .instance()
    }
}

impl Widget for TestWidget {
    fn build(&mut self, size: &mut BuildContext) -> BuildResult {
        //self.container.build(BuildContext::new(content_rect));
        self.container.build(size);
        BuildResult::default()
    }

    fn children(&self) -> &[WidgetInstance] {
        from_ref(&self.container)
    }
}

fn main() {
    UIApp::new().main_container(main_container()).run();
}
