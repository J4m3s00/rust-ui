use rust_graphics::color::Color;

use super::color_theme::ColorTheme;

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub colors: ColorTheme,
}

impl Default for Theme {
    fn default() -> Self {
        Self::light_theme()
    }
}

impl Theme {
    pub fn light_theme() -> Self {
        Theme {
            colors: ColorTheme {
                primary_color: Color::from("edeec9"),
                primary_color_variant: Color::from("c5c7a7"),
                secondary_color: Color::from("c5c7a7"),
                secondary_color_variant: Color::from("a3a58a"),
                background_color: Color::from("ffffff"),
                surface_color: Color::from("f2f2f2"),
                error_color: Color::from("b00020"),
                on_primary_color: Color::from("000000"),
                on_secondary_color: Color::from("000000"),
                on_background_color: Color::from("000000"),
                on_surface_color: Color::from("000000"),
                on_error_color: Color::from("ffffff"),
            },
        }
    }

    pub fn dark_theme() -> Self {
        Theme {
            colors: ColorTheme {
                primary_color: Color::from("22223b"),
                primary_color_variant: Color::from("4a4e69"),
                secondary_color: Color::from("4a4e69"),
                secondary_color_variant: Color::from("9a8c98"),
                background_color: Color::from("1a1a2e"),
                surface_color: Color::from("16213e"),
                error_color: Color::from("ff2e63"),
                on_primary_color: Color::from("ffffff"),
                on_secondary_color: Color::from("ffffff"),
                on_background_color: Color::from("ffffff"),
                on_surface_color: Color::from("ffffff"),
                on_error_color: Color::from("ffffff"),
            },
        }
    }
}
