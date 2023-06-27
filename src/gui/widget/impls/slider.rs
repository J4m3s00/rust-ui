use std::slice::from_ref;

use rust_graphics::{color::Color, draw_command::Fill};

use crate::{
    gui::widget::{
        builder::{
            build_context::{BuildContext, CursorDirection},
            build_results::{BuildResult, WidgetRenderItem, WidgetRenderRect},
        },
        state::{observable::Observer, state::State, state_ref::StateRef},
    },
    prelude::{AppInterface, ToInstance, Widget, WidgetInstance},
    MapScalar,
};

pub struct Slider {
    knob: WidgetInstance,
    knob_state: State<KnobState>,
    value: State<f32>,
    min: Observer<f32>,
    max: Observer<f32>,
}

impl Slider {
    pub fn new(init: f32) -> WidgetInstance {
        Self::new_min_max(init, 0., 100.)
    }

    pub fn new_min_max(
        init: f32,
        min: impl Into<Observer<f32>>,
        max: impl Into<Observer<f32>>,
    ) -> WidgetInstance {
        let knob_state = State::new(KnobState::Idle);
        Self {
            knob: SliderKnob::new(knob_state.get_ref()),
            value: State::new(init),
            min: min.into(),
            max: max.into(),
            knob_state,
        }
        .instance()
    }
}

impl Widget for Slider {
    fn build(&mut self, ctx: &mut BuildContext) -> BuildResult {
        ctx.set_cursor_direction(CursorDirection::Horizontal);
        let content_area = ctx.get_content_rect().clone();
        let curr_value = self.value.get();
        let knob_width = 20.;
        let knob_height = content_area.height();
        let knob_left = curr_value.map(
            self.min.get().unwrap_or_default(),
            self.max.get().unwrap_or_default(),
            0.,
            1.,
        ) * content_area.width()
            - knob_width / 2.;

        // Spacing the knob
        ctx.allocate_space((knob_left, content_area.height()));

        // Knob
        if let Some(mut knob_ctx) = ctx.allocate_space((knob_width, knob_height)) {
            self.knob.build(&mut knob_ctx);
        }

        BuildResult::default()
    }

    fn children(&self) -> &[WidgetInstance] {
        from_ref(&self.knob)
    }
}

// Own Widgets
#[derive(Copy, Clone, Debug)]
enum KnobState {
    Idle,
    Dragging,
}

struct SliderKnob {
    state: StateRef<KnobState>,
}

impl SliderKnob {
    fn new(state: StateRef<KnobState>) -> WidgetInstance {
        Self { state }.instance()
    }
}

impl Widget for SliderKnob {
    fn build(&mut self, _ctx: &mut BuildContext) -> BuildResult {
        BuildResult::default().with_render_item(WidgetRenderItem::Rect(WidgetRenderRect {
            fill: self
                .state
                .map(|v| match v {
                    KnobState::Idle => Some(Fill {
                        color: Color::new(54, 54, 54, 255),
                    }),
                    KnobState::Dragging => Some(Fill {
                        color: Color::new(64, 64, 64, 255),
                    }),
                })
                .into(),
            ..Default::default()
        }))
    }

    fn on_mouse_down(&self, _interface: AppInterface) {
        self.state.set(KnobState::Dragging);
    }

    fn on_mouse_up(&self, _interface: AppInterface) {
        self.state.set(KnobState::Idle);
    }
}
