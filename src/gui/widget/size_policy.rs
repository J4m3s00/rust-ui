use super::builder::relative_size::RelativeSize;

#[derive(Debug, Clone, Copy)]
pub enum SizePolicy {
    Fixed(f32),
    Relative(RelativeSize),
    Fraction(f32),
}

impl Default for SizePolicy {
    fn default() -> Self {
        Self::Fraction(1.0)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SizePolicy2D {
    pub horizontal: SizePolicy,
    pub vertical: SizePolicy,
}

impl From<SizePolicy> for SizePolicy2D {
    fn from(policy: SizePolicy) -> Self {
        Self {
            horizontal: policy,
            vertical: policy,
        }
    }
}

impl From<(SizePolicy, SizePolicy)> for SizePolicy2D {
    fn from((horizontal, vertical): (SizePolicy, SizePolicy)) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }
}
