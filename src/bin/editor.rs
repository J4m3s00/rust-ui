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

struct StepperWidget;

impl StepperWidget {
    fn new() -> WidgetInstance {
        let val: State<i128> = State::new(1); // I dont like this. It should default clone when it is pushed into a closure
        HStack::new(vec![
            Label::new_observe(val.map(|v| Text::from(format!("Value: {}", v)))),
            VStack::new(vec![
                Button::new("+", {
                    let val = val.clone();
                    move |_| {
                        let cur: i128 = val.get();
                        val.set(cur.checked_mul(10).unwrap_or(cur));
                    }
                }),
                Button::new("-", {
                    let val = val.clone();
                    move |_| {
                        let cur = val.get();
                        val.set(cur.checked_div(-3).unwrap_or(cur));
                    }
                }),
            ])
            .set_width(SizePolicy::Relative(RelativeSize::PercentageV(0.5))),
        ])
    }
}

fn main() {
    UIApp::new().main_container(main_container()).run();
}
