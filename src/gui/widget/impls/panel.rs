use rust_graphics::{
    draw_command::{DrawCommand, Fill, Stroke},
    run_draw_command,
};

use crate::{
    gui::{
        app::{app::FontManager, input::InputState},
        widget::{
            builder::build_context::CursorDirection, style::space::ApplySpace, theme::theme::Theme,
        },
    },
    prelude::*,
};
pub struct Panel {
    widget: WidgetInstance,
    position: State<Vec2>,
    size: State<Vec2>,
}

impl Widget for Panel {
    fn build(
        &mut self,
        _ctx: &mut BuildContext,
        _mouse_state: Observer<WidgetMouseState>,
        _focused: Observer<bool>,
    ) -> BuildResult {
        let position = self.position.get();
        let size = self.size.get();

        let mut build_context = BuildContext::new(
            Rect::new_from_xy(position.x, position.y, size.x, size.y),
            CursorDirection::Vertical,
        );

        self.widget.build(&mut build_context);
        BuildResult::default()
    }
}

impl Panel {
    pub fn new(widget: WidgetInstance, position: Vec2, size: Vec2) -> Self {
        Self {
            widget,
            position: State::new(position),
            size: State::new(size),
        }
    }

    pub fn draw(&self, font_manager: &FontManager, theme: &Theme, input_state: &InputState) {
        let position = self.position.get();
        let size = self.size.get();

        DrawCommand::rect_fill(
            position.x,
            position.y,
            size.x,
            size.y,
            Fill::new(theme.colors.from_id(ColorId::Background)),
        )
        .run();
        for item in self.widget.iter() {
            if !item.visible() {
                continue;
            }
            let (result, area, _) = item.build_result();
            let mut padded_area = *area;
            padded_area.apply_space(&item.style().padding);

            for item in result.render_items().iter() {
                item.get_draw_command(&padded_area, font_manager, theme)
                    .iter()
                    .for_each(DrawCommand::run);
            }
            if let Some(id) = input_state.focused_input {
                if id == item.id() {
                    run_draw_command(&DrawCommand::Rect {
                        left: area.left,
                        top: area.top,
                        width: area.width(),
                        height: area.height(),
                        fill: None,
                        stroke: Some(Stroke {
                            width: 1.,
                            color: theme.colors.from_id(ColorId::OnPrimary),
                        }),
                    });
                }
            }
        }
    }

    pub fn set_position(&mut self, position: impl Into<Vec2>) {
        self.position.set(position.into());
    }

    pub fn set_size(&mut self, size: impl Into<Vec2>) {
        self.size.set(size.into());
    }

    pub fn position(&self) -> Vec2 {
        self.position.get()
    }

    pub fn size(&self) -> Vec2 {
        self.size.get()
    }
}
