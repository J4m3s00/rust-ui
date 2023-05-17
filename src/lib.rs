use bevy_ecs::{schedule::Schedule, system::Query};
use gui::widget::Widget;
use rust_graphics::{app::App, run_app, draw_command::DrawCommand};
use scene::scene::Scene;

pub mod scene;
pub mod error;
pub mod gui;

struct UIApp {
    scene: Scene,
    scheduler: Schedule,
}

fn test_system(query : Query<&Widget>) {
    for widget in query.iter() {
        println!("Drawing Widget {:?}", widget);
    }
}

impl Default for UIApp {
    fn default() -> Self {
        let mut scheduler = Schedule::new();
        scheduler.add_system(test_system);
        Self {
            scene: Scene::new(),
            scheduler,
        }
    }
}

impl App for UIApp {
    fn on_start(&mut self) {
        let test_widget = self.scene.add_entity(None);
    }

    fn on_draw(&mut self) {
        self.scene.run_schedule(&mut self.scheduler);
    }
}

pub fn start_app() {
    run_app(UIApp::default());
}