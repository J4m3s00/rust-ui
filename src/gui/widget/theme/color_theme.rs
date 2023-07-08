use rust_graphics::color::Color;

#[derive(Clone)]
pub enum ColorId {
    Primary,
    PrimaryVariantLight,
    PrimaryVariantDark,
    Secondary,
    SecondaryVariantLight,
    SecondaryVariantDark,
    Background,
    Surface,
    Error,
    OnPrimary,
    OnSecondary,
    OnBackground,
    OnSurface,
    OnError,
    Custom(Color),
}

#[derive(Debug, Clone, Copy)]
pub struct ColorTheme {
    pub(super) primary: Color,
    pub(super) primary_variant_light: Color,
    pub(super) primary_variant_dark: Color,
    pub(super) secondary: Color,
    pub(super) secondary_variant_light: Color,
    pub(super) secondary_variant_dark: Color,
    pub(super) background: Color,
    pub(super) surface: Color,
    pub(super) error: Color,
    pub(super) on_primary: Color,
    pub(super) on_secondary: Color,
    pub(super) on_background: Color,
    pub(super) on_surface: Color,
    pub(super) on_error: Color,
}

impl ColorTheme {
    pub fn from_id(&self, id: ColorId) -> Color {
        match id {
            ColorId::Primary => self.primary,
            ColorId::PrimaryVariantLight => self.primary_variant_light,
            ColorId::PrimaryVariantDark => self.primary_variant_dark,
            ColorId::Secondary => self.secondary,
            ColorId::SecondaryVariantLight => self.secondary_variant_light,
            ColorId::SecondaryVariantDark => self.secondary_variant_dark,
            ColorId::Background => self.background,
            ColorId::Surface => self.surface,
            ColorId::Error => self.error,
            ColorId::OnPrimary => self.on_primary,
            ColorId::OnSecondary => self.on_secondary,
            ColorId::OnBackground => self.on_background,
            ColorId::OnSurface => self.on_surface,
            ColorId::OnError => self.on_error,
            ColorId::Custom(color) => color,
        }
    }
}
