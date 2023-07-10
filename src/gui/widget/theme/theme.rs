use rust_graphics::color::Color;

use super::color_theme::ColorTheme;

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub colors: ColorTheme,
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark_theme()
    }
}

impl Theme {
    pub fn light_theme() -> Self {
        Theme {
            colors: ColorTheme {
                primary: Color::from("edeec9"),
                primary_variant_light: Color::from("c5c7a7"),
                primary_variant_dark: Color::from("c5c7a7"),
                secondary: Color::from("c5c7a7"),
                secondary_variant_light: Color::from("a3a58a"),
                secondary_variant_dark: Color::from("a3a58a"),
                background: Color::from("ffffff"),
                surface: Color::from("f2f2f2"),
                error: Color::from("b00020"),
                on_primary: Color::from("000000"),
                on_secondary: Color::from("000000"),
                on_background: Color::from("000000"),
                on_surface: Color::from("000000"),
                on_error: Color::from("ffffff"),
            },
        }
    }

    pub fn dark_theme() -> Self {
        Theme {
            colors: ColorTheme {
                primary: Color::from("2C3639"),
                primary_variant_light: Color::from("3F4E4F"),
                primary_variant_dark: Color::from("243131"),
                secondary: Color::from("3F4E6F"),
                secondary_variant_light: Color::from("434253"),
                secondary_variant_dark: Color::from("3d4c4d"),
                background: Color::from("1a1a1a"),
                surface: Color::from("16213e"),
                error: Color::from("ff2e63"),
                on_primary: Color::from("ffffff"),
                on_secondary: Color::from("ffffff"),
                on_background: Color::from("ffffff"),
                on_surface: Color::from("ffffff"),
                on_error: Color::from("ffffff"),
            },
        }
    }
}
