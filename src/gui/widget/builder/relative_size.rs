#[derive(Debug, Clone, Copy)]
pub enum RelativeSize {
    Percent(f32),
    PercentageH(f32),
    PercentageV(f32),
}

impl Default for RelativeSize {
    fn default() -> Self {
        Self::Percent(1.)
    }
}
