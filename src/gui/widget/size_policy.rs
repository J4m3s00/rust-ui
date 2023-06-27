use rust_graphics::vec::Vec2;

#[derive(Debug, Clone, Copy)]
pub enum SizePolicy {
    Fixed(f32),
    Percent(f32),
    PercentageH(f32),
    PercentageV(f32),
    Fraction(f32),
}

impl Default for SizePolicy {
    fn default() -> Self {
        Self::Fraction(1.0)
    }
}

impl SizePolicy {
    pub fn zero() -> Self {
        Self::Fixed(0.0)
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

impl SizePolicy2D {
    pub fn zero() -> Self {
        Self {
            horizontal: SizePolicy::zero(),
            vertical: SizePolicy::zero(),
        }
    }

    pub fn calculate_size(&self, size: Vec2) -> Vec2 {
        (
            match self.horizontal {
                SizePolicy::Fixed(w) => w,
                SizePolicy::Percent(p) => size.x * p,
                SizePolicy::PercentageH(p) => size.x * p,
                SizePolicy::PercentageV(p) => size.y * p,
                SizePolicy::Fraction(f) => size.x / f,
            },
            match self.vertical {
                SizePolicy::Fixed(h) => h,
                SizePolicy::Percent(p) => size.y * p,
                SizePolicy::PercentageH(p) => size.x * p,
                SizePolicy::PercentageV(p) => size.y * p,
                SizePolicy::Fraction(f) => size.y / f,
            },
        )
            .into()
    }
}
