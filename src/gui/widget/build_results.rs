use rust_graphics::rect::Rect;

#[derive(Debug, Clone, Default)]
pub struct BuildResult {
    content_area: Rect,
    text: Option<String>,
    // Add render commands here as well in the future. need to work with scissor test
}
