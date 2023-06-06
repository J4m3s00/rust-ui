use gui::widget::{SizePolicy, Widget};
use rust_graphics::{
    app::App,
    color::COLOR_CYAN,
    draw_command::{DrawCommand, Fill},
    run_app, run_draw_command,
};

pub mod error;
pub mod gui;

struct UIApp {}

impl Default for UIApp {
    fn default() -> Self {
        Self {}
    }
}

impl App for UIApp {
    fn on_start(&mut self) {}

    fn on_draw(&mut self) {}
}

pub fn start_app() {
    run_app(UIApp::default());
}
