use std::slice::from_ref;

use rust_ui::{
    gui::widget::{build_context::BuildContext, build_results::BuildResult},
    prelude::*,
};

struct EmptyWidget;
impl Widget for EmptyWidget {
    fn build(&mut self, size: &mut BuildContext) -> BuildResult {
        BuildResult::default()
    }
}

fn main_container() -> WidgetInstance {
    // Center a label in the middle of the screen.
    VStack::new(vec![
        EmptyWidget.instance(),
        HStack::new(vec![
            EmptyWidget.instance(),
            //Button::new("Click me!").set_width(SizePolicy::Fixed(128.)),
            TestWidget::new(),
        ])
        .set_height(SizePolicy::Fixed(256.)),
        EmptyWidget.instance(),
    ])
}

struct TestWidget {
    state: u32,
    container: WidgetInstance,
}

impl TestWidget {
    fn new() -> WidgetInstance {
        Self {
            state: 0,
            container: VStack::new(vec![
                Label::new("Hello, world!"),
                Button::new("Click me!").set_width(SizePolicy::Fixed(128.)),
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
