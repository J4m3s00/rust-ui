use rust_ui::prelude::*;

struct EmptyWidget;
impl Widget for EmptyWidget {
    fn build(&self, size: Vec2) {}
}

fn main_container() -> impl Widget {
    // Center a label in the middle of the screen.
    VStack::new(vec![
        EmptyWidget.into_item(),
        HStack::new(vec![
            EmptyWidget.into_item(),
            Button::new("Click me!")
                .into_item()
                .set_width(SizePolicy::Fixed(128.)),
            TestWidget::new().into_item(),
        ])
        .into_item()
        .set_height(SizePolicy::Fixed(64.)),
        EmptyWidget.into_item(),
    ])
}

struct TestWidget {
    state: u32,
    container: VStack,
}

impl TestWidget {
    fn new() -> Self {
        Self {
            state: 0,
            container: VStack::new(vec![
                Label::new("Hello, world!").into_item(),
                Button::new("Click me!")
                    .into_item()
                    .set_width(SizePolicy::Fixed(128.)),
                Label::new("Hello, world!").into_item(),
            ]),
        }
    }
}

impl Widget for TestWidget {
    fn build(&self, size: Vec2) {
        self.container.build(size);
    }
}

fn main() {
    UIApp::new().run();
}
