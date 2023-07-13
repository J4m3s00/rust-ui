use rust_graphics::vec::Vec2;

#[derive(Debug, Default)]
pub struct InputState {
    pub mouse_pos: Vec2,
    pub focused_input: Option<usize>,
}
