use rust_graphics::draw_command::Fill;

use crate::{
    gui::widget::{
        builder::{build_context::BuildContext, build_results::BuildResult},
        rendering::drawable::rectangle::DrawRect,
        state::{observable::Observer, state::State},
        widget::MouseEvent,
    },
    prelude::{AlignH, AppInterface, SizePolicy, ToInstance, Widget, WidgetInstance},
    MapScalar,
};

const KNOB_WIDTH: f32 = 16.;

pub struct Slider {
    knob: State<KnobState>,
    value: State<f32>,
    min: Observer<f32>,
    max: Observer<f32>,

    slider_pixel_scale: Observer<f32>,
}

impl Slider {
    pub fn new(init: State<f32>) -> WidgetInstance {
        Self::new_min_max(init, 0., 100.)
    }

    pub fn new_min_max(
        init: State<f32>,
        min: impl Into<Observer<f32>>,
        max: impl Into<Observer<f32>>,
    ) -> WidgetInstance {
        let knob = State::new(KnobState::Idle);
        Self {
            value: init,
            min: min.into(),
            max: max.into(),
            knob,

            slider_pixel_scale: Observer::value(1.),
        }
        .instance()
    }

    fn get_value_from_piexels(&self, pixels: f32) -> f32 {
        let slider_pixel_scale = self.slider_pixel_scale.get().unwrap();
        (pixels - (KNOB_WIDTH / 2.)) / slider_pixel_scale
    }

    fn min(&self) -> f32 {
        self.min.get().unwrap_or(0.)
    }

    fn max(&self) -> f32 {
        self.max.get().unwrap_or(100.)
    }

    fn set_value(&self, value: f32) {
        self.value.set(value.clamp(self.min(), self.max()));
    }
}

impl Widget for Slider {
    fn build(&mut self, ctx: &mut BuildContext) -> BuildResult {
        let content_area = ctx.get_content_rect().clone();

        let width = content_area.width() - KNOB_WIDTH;
        self.slider_pixel_scale = Observer::map(self.max.reference(), move |v| width / v);

        let knob_left = self.value.map({
            let min = self.min.reference();
            let max = self.max.reference();
            move |v| {
                SizePolicy::Fixed(
                    (KNOB_WIDTH / 2.)
                        + v.map(min.get().unwrap_or(0.), max.get().unwrap_or(100.), 0., 1.) * width,
                )
            }
        });

        let mut res = BuildResult::default();
        res.draw_rect(DrawRect::fill(Observer::value(Some(Fill {
            color: ctx.theme().primary_color,
        }))))
        .set_width(SizePolicy::Fixed(KNOB_WIDTH))
        .set_alignment_h(AlignH::Left)
        .set_offset_x(knob_left);
        res
    }

    fn on_mouse_down(&self, event: MouseEvent, _interface: AppInterface) {
        if event.inside {
            self.knob.set(KnobState::Dragging);
            self.set_value(self.get_value_from_piexels(event.relative_pos.x));
        }
    }

    fn on_mouse_up(&self, _event: MouseEvent, _interface: AppInterface) {
        self.knob.set(KnobState::Idle);
    }

    fn on_mouse_move(&self, event: MouseEvent, _interface: AppInterface) {
        if let KnobState::Dragging = self.knob.get() {
            self.set_value(self.get_value_from_piexels(event.relative_pos.x));
        }
    }
}

// Own Widgets
#[derive(Copy, Clone, Debug)]
enum KnobState {
    Idle,
    Dragging,
}
