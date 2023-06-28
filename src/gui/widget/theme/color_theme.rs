use rust_graphics::color::Color;

#[derive(Clone)]
pub enum ColorId {
    Primary,
    PrimaryVariant,
    Secondary,
    SecondaryVariant,
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
    pub(super) primary_color: Color,
    pub(super) primary_color_variant: Color,
    pub(super) secondary_color: Color,
    pub(super) secondary_color_variant: Color,
    pub(super) background_color: Color,
    pub(super) surface_color: Color,
    pub(super) error_color: Color,
    pub(super) on_primary_color: Color,
    pub(super) on_secondary_color: Color,
    pub(super) on_background_color: Color,
    pub(super) on_surface_color: Color,
    pub(super) on_error_color: Color,
}

impl ColorTheme {
    pub fn from_id(&self, id: ColorId) -> Color {
        match id {
            ColorId::Primary => self.primary_color,
            ColorId::PrimaryVariant => self.primary_color_variant,
            ColorId::Secondary => self.secondary_color,
            ColorId::SecondaryVariant => self.secondary_color_variant,
            ColorId::Background => self.background_color,
            ColorId::Surface => self.surface_color,
            ColorId::Error => self.error_color,
            ColorId::OnPrimary => self.on_primary_color,
            ColorId::OnSecondary => self.on_secondary_color,
            ColorId::OnBackground => self.on_background_color,
            ColorId::OnSurface => self.on_surface_color,
            ColorId::OnError => self.on_error_color,
            ColorId::Custom(color) => color,
        }
    }
}
