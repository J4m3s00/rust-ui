#[derive(Clone, Copy, Debug)]
pub enum Anchor {
    Leading,
    Center,
    Trailing,
}

impl Default for Anchor {
    fn default() -> Self {
        Self::Center
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Anchor2D {
    pub horizontal: Anchor,
    pub vertical: Anchor,
}

impl From<(Anchor, Anchor)> for Anchor2D {
    fn from((horizontal, vertical): (Anchor, Anchor)) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }
}
