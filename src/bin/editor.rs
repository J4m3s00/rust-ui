use std::slice::from_ref;

use rust_ui::{
    gui::widget::{
        builder::{
            build_context::BuildContext, build_results::BuildResult, relative_size::RelativeSize,
        },
        impls::zstack::ZStack,
        state::state::State,
    },
    prelude::*,
};

struct EmptyWidget;
impl Widget for EmptyWidget {
    fn build(&mut self, _ctx: &mut BuildContext) -> BuildResult {
        BuildResult::default()
    }
}

fn main_container() -> WidgetInstance {
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
            StepperWidget::new(),
        ])
        .set_height(SizePolicy::Fixed(72.)),
        EmptyWidget.instance(),
    ])
}

struct StepperWidget {
    container: WidgetInstance,
}

impl StepperWidget {
    fn new() -> WidgetInstance {
        let val = State::new(0);
        let changer_a = val.clone();
        let changer_b = val.clone();

        Self {
            container: HStack::new(vec![
                Label::new_observe(val.map(|v| Text::from(format!("Value: {}", v)))),
                VStack::new(vec![
                    Button::new("+", move |_| {
                        let cur = changer_b.get();
                        changer_b.set(cur + 1);
                        println!("Value: {}", changer_b.get());
                    }),
                    Button::new("-", move |_| {
                        let cur = changer_a.get();
                        changer_a.set(cur - 1);
                        println!("Value: {}", changer_a.get());
                    }),
                ])
                .set_width(SizePolicy::Relative(RelativeSize::PercentageV(0.5))),
            ]),
        }
        .instance()
    }
}

impl Widget for StepperWidget {
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
