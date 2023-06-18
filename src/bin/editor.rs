use rust_graphics::app::App;
use rust_ui::prelude::*;

fn main_screen() -> impl Widget {
    VStack::new(vec![
        HStack::new(vec![
            Button::new("Click 1")
                .into_item()
                .set_width(SizePolicy::Fixed(72.)),
            Button::new("Click 2")
                .into_item()
                .set_width(SizePolicy::Fixed(72.)),
            Button::new("Click 3")
                .into_item()
                .set_width(SizePolicy::Fixed(72.)),
            Button::new("Click 4")
                .into_item()
                .set_width(SizePolicy::Fixed(72.)),
            Button::new("Click 5")
                .into_item()
                .set_width(SizePolicy::Fixed(72.)),
            Button::new("Click 6")
                .into_item()
                .set_width(SizePolicy::Fixed(72.)),
            Label::new("Header").into_item(),
        ])
        .into_item()
        .set_height(SizePolicy::Fixed(32.)),
        HStack::new(vec![
            Label::new("Sidebar")
                .hor_align(TextAlignH::Left)
                .vert_align(TextAlignV::Top)
                .into_item()
                .set_width(SizePolicy::Fixed(256.)),
            VStack::new(vec![]).into_item(),
        ])
        .into_item(),
    ])
}

fn main() -> Result<()> {
    UIApp::new().main_container(main_screen).run();
    Ok(())
}
