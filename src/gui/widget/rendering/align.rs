#[derive(Clone, Debug, Copy)]
pub enum AlignH {
    Left,
    Center,
    Right,
}

#[derive(Clone, Debug, Copy)]
pub enum AlignV {
    Top,
    Center,
    Bottom,
}

impl Default for AlignH {
    fn default() -> Self {
        Self::Center
    }
}

impl Default for AlignV {
    fn default() -> Self {
        Self::Center
    }
}

#[derive(Clone, Debug, Copy)]
pub struct Align2D {
    pub horizontal: AlignH,
    pub vertical: AlignV,
}

impl Default for Align2D {
    fn default() -> Self {
        Self {
            horizontal: AlignH::default(),
            vertical: AlignV::default(),
        }
    }
}

impl From<(AlignH, AlignV)> for Align2D {
    fn from((horizontal, vertical): (AlignH, AlignV)) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }
}
