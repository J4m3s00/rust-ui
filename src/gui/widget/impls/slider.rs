use rust_graphics::{color::COLOR_CYAN, draw_command::Fill};

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
        pixels / slider_pixel_scale
    }
}

impl Widget for Slider {
    fn build(&mut self, ctx: &mut BuildContext) -> BuildResult {
        let content_area = ctx.get_content_rect().clone();
        let width = content_area.width();
        self.slider_pixel_scale = Observer::map(self.max.reference(), move |v| width / v);

        let knob_width = 20.;
        let knob_height = content_area.height();
        let knob_left = self.value.map({
            let min = self.min.reference();
            let max = self.max.reference();
            move |v| {
                SizePolicy::Fixed(
                    v.map(min.get().unwrap_or(0.), max.get().unwrap_or(100.), 0., 1.)
                        * content_area.width(),
                )
            }
        });

        let mut res = BuildResult::default();
        res.draw_rect(DrawRect::fill(Observer::value(Some(Fill {
            color: COLOR_CYAN,
        }))))
        .set_width(SizePolicy::Fixed(knob_width))
        .set_height(SizePolicy::Percent(1.))
        .set_alignment_h(AlignH::Left)
        .set_offset_x(knob_left);
        res
    }

    fn on_mouse_down(&self, event: MouseEvent, _interface: AppInterface) {
        self.knob.set(KnobState::Dragging);
        self.value
            .set(self.get_value_from_piexels(event.relative_pos.x));
    }

    fn on_mouse_up(&self, event: MouseEvent, _interface: AppInterface) {
        self.knob.set(KnobState::Idle);
    }

    fn on_mouse_move(&self, event: MouseEvent, _interface: AppInterface) {
        if let KnobState::Dragging = self.knob.get() {
            self.value
                .set(self.get_value_from_piexels(event.relative_pos.x));
        }
    }
}

// Own Widgets
#[derive(Copy, Clone, Debug)]
enum KnobState {
    Idle,
    Dragging,
}
