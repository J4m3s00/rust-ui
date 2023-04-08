use crate::{color::Color, rect::Rect};

pub enum Space {
    All {
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,
    },
    Axis {
        horizontal: f32,
        vertical: f32,
    },
    None,
}

impl Space {
    pub fn get_rect(&self) -> Rect {
        match self {
            Self::All {
                left,
                top,
                right,
                bottom,
            } => Rect::new(*left, *top, *right, *bottom),
            Self::Axis {
                horizontal,
                vertical,
            } => Rect::new(*horizontal, *vertical, *horizontal, *vertical),
            Self::None => Rect::new(0.0, 0.0, 0.0, 0.0),
        }
    }
}

pub struct Style {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
    pub font_size: Option<f32>,

    pub border_width: Option<f32>,
    pub border_color: Option<Color>,
    pub border_radius: Option<f32>,

    pub padding: Option<Space>,
    pub margin: Option<Space>,
}
