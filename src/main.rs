use application::run;

mod application;
mod contrast;
mod state;

fn main() {
    pollster::block_on(run());
}
