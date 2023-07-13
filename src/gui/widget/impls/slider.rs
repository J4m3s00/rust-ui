use crate::{prelude::*, MapScalar};

use super::{rectangle::Rectangle, zstack::ZStack};

const KNOB_WIDTH: f32 = 16.;

pub struct Slider;

impl Slider {
    pub fn new(init: State<f32>) -> WidgetInstance {
        Self::new_min_max(init, 0., 100.)
    }

    pub fn new_min_max(
        init: State<f32>,
        min: impl Into<Observer<f32>>,
        max: impl Into<Observer<f32>>,
    ) -> WidgetInstance {
        ZStack::new(vec![
            Rectangle::fill(ColorId::Primary),
            HStack::new(vec![
                Label::new_observe(init.map(|v| Text::from(format!("{v:.0}"))))
                    .set_width(SizePolicy::Fixed(50.)),
                SliderBase::new_min_max(init, min, max),
            ]),
        ])
        .set_margin(Margin::zero())
    }
}

struct SliderBase {
    knob: State<KnobState>,
    value: State<f32>,
    min: Observer<f32>,
    max: Observer<f32>,

    slider_pixel_scale: Observer<f32>,
}

impl SliderBase {
    fn new_min_max(
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
        .accept_input()
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

impl Widget for SliderBase {
    fn build(
        &mut self,
        ctx: &mut BuildContext,
        mouse_state: Observer<WidgetMouseState>,
        _: Observer<bool>,
    ) -> BuildResult {
        let content_area = *ctx.get_content_rect();

        let width = content_area.width() - KNOB_WIDTH;
        self.slider_pixel_scale = self.max.map(move |v| width / v);

        let knob_left = self.value.map({
            let min = self.min.clone();
            let max = self.max.clone();
            move |v| {
                SizePolicy::Fixed(
                    (KNOB_WIDTH / 2.)
                        + v.map(min.get().unwrap_or(0.), max.get().unwrap_or(100.), 0., 1.) * width,
                )
            }
        });

        let color = (self.knob.observe(), mouse_state).map(|state| match state {
            (KnobState::Idle, WidgetMouseState::Hovered) => Some(ColorId::SecondaryVariantLight),
            (KnobState::Dragging, _) => Some(ColorId::SecondaryVariantDark),
            _ => Some(ColorId::Secondary),
        });

        let mut res = BuildResult::default();
        // Draw Knob
        res.draw_rect(DrawRect::fill(color))
            .set_width(SizePolicy::Fixed(KNOB_WIDTH))
            .set_alignment_h(AlignH::Left)
            .set_offset_x(knob_left);
        res
    }

    fn on_mouse_down(&self, event: &MouseEvent, _interface: AppInterface) {
        if event.inside {
            self.knob.set(KnobState::Dragging);
            self.set_value(self.get_value_from_piexels(event.relative_pos.x));
        }
    }

    fn on_mouse_up(&self, _event: &MouseEvent, _interface: AppInterface) {
        self.knob.set(KnobState::Idle);
    }

    fn on_mouse_move(&self, event: &MouseEvent, _interface: AppInterface) {
        if let KnobState::Dragging = self.knob.get() {
            self.set_value(self.get_value_from_piexels(event.relative_pos.x));
        }
    }

    fn on_key_down(&self, key: KeyCode, _: KeyMods, _interface: AppInterface) {
        match key {
            KeyCode::Left => self.set_value(self.value.get() - 1.),
            KeyCode::Right => self.set_value(self.value.get() + 1.),
            _ => {}
        }
    }
}

// Own Widgets
#[derive(Copy, Clone, Debug)]
enum KnobState {
    Idle,
    Dragging,
}
