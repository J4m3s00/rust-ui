use crate::{
    gui::widget::{
        builder::{build_context::BuildContext, build_results::BuildResult},
        rendering::drawable::rectangle::DrawRect,
        state::observable::{MapObserver, Observer},
        widget::WidgetMouseState,
    },
    prelude::{ColorId, State, ToInstance, Widget, WidgetInstance},
};

pub struct Rectangle {
    fill: Observer<Option<ColorId>>,
    stroke: Observer<Option<(ColorId, f32)>>,
}

impl Rectangle {
    pub fn fill(fill: impl Into<Observer<ColorId>>) -> WidgetInstance {
        Self {
            fill: fill.into().map(|fill| Some(fill.clone())),
            stroke: Observer::value(None),
        }
        .instance()
    }

    pub fn stroke(stroke: impl Into<Observer<(ColorId, f32)>>) -> WidgetInstance {
        Self {
            fill: Observer::value(None),
            stroke: stroke.into().map(|stroke| Some(stroke.clone())),
        }
        .instance()
    }

    pub fn fill_and_stroke(
        fill: impl Into<Observer<ColorId>>,
        stroke: impl Into<Observer<(ColorId, f32)>>,
    ) -> WidgetInstance {
        Self {
            fill: fill.into().map(|fill| Some(fill.clone())),
            stroke: stroke.into().map(|stroke| Some(stroke.clone())),
        }
        .instance()
    }
}

impl Widget for Rectangle {
    fn build(&mut self, _context: &mut BuildContext, _: &State<WidgetMouseState>) -> BuildResult {
        let mut res = BuildResult::default();
        res.draw_rect(DrawRect::fill_and_stroke(
            self.fill.clone(),
            self.stroke.clone(),
        ));
        res
    }
}
