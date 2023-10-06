use rust_graphics::draw_command::{DrawCommand, Fill};

use crate::{gui::widget::rendering::drawable::item::DrawItem, prelude::*};

const WORD_BREAK_CHARS: &[char] = &[' ', '\n', '\t', '-', '_', '.', '/'];

pub struct TextInput {
    value: State<Text>,
    cursor_pos: State<usize>,
    editing: State<bool>,
}

const CURSOR_WIDTH: f32 = 2.0;
const CURSOR_PADDING_V: f32 = 2.0;

struct CursorDrawer {
    cursor_pos: Observer<usize>,
    text: Observer<Text>,
}

impl Drawable for CursorDrawer {
    fn draw(
        &self,
        area: Rect,
        font_manager: &crate::gui::app::app::FontManager,
        theme: &crate::gui::widget::theme::theme::Theme,
    ) -> Vec<rust_graphics::draw_command::DrawCommand> {
        let Some(text) = self.text.get() else {
            return vec![];
        };
        let Some(cursor_pos) = self.cursor_pos.get() else {
            return vec![];
        };
        let font = text.font.unwrap_or(font_manager.default_font().clone());

        let offset_x = font.get_text_width(&text.text[..cursor_pos]);
        println!("Offset: {offset_x}; {cursor_pos}");
        vec![DrawCommand::rect_fill(
            area.left() + offset_x as f32,
            area.top() + CURSOR_PADDING_V,
            CURSOR_WIDTH,
            area.height() - (CURSOR_PADDING_V * 2.0),
            Fill::new(theme.colors.from_id(ColorId::OnPrimary)),
        )]
        /*vec![DrawCommand::rect_fill(
            area.left,
            area.top,
            area.width(),
            area.height(),
            Fill::new(Color::new(255, 0, 255, 255)),
        )]*/
    }
}

impl TextInput {
    pub fn new(init_value: impl Into<String>) -> WidgetInstance {
        Self {
            value: State::new(Text::from(init_value).hor_align(AlignH::Left)),
            cursor_pos: State::new(0),
            editing: State::new(false),
        }
        .instance()
        .accept_input()
        .custom_cursor(SystemCursor::IBeam)
    }

    /// Editing functions
    /// Those change the value and the cursor
    fn delete_previous(&self) {
        if self.cursor_pos.get() > 0 {
            let mut text = self.value.get().clone();
            text.text.remove(self.cursor_pos.get() - 1);
            self.value.set(text);
            self.cursor_pos.set(self.cursor_pos.get() - 1);
        }
    }

    fn delete_previous_word(&self) {
        let new_cursor = self.find_previous_word_pos();
        let mut text = self.value.get();

        text.text
            .replace_range(new_cursor..self.cursor_pos.get(), "");
        self.value.set(text);
        self.cursor_pos.set(new_cursor);
    }

    fn delete_next(&self) {
        if self.cursor_pos.get() < self.value.get().text.len() {
            let mut text = self.value.get().clone();
            text.text.remove(self.cursor_pos.get());
            self.value.set(text);
        }
    }

    fn delete_next_word(&self) {
        let new_cursor = self.find_next_word_pos();
        let mut text = self.value.get();

        text.text
            .replace_range(self.cursor_pos.get()..new_cursor, "");
        self.value.set(text);
        self.cursor_pos.set(new_cursor);
    }

    fn move_cursor_left(&self) {
        if self.cursor_pos.get() > 0 {
            self.cursor_pos.set(self.cursor_pos.get() - 1);
        }
    }

    fn move_cursor_previous_word(&self) {
        self.cursor_pos.set(self.find_previous_word_pos());
    }

    fn move_cursor_right(&self) {
        if self.cursor_pos.get() < self.value.get().text.len() {
            self.cursor_pos.set(self.cursor_pos.get() + 1);
        }
    }

    fn move_cursor_next_word(&self) {
        self.cursor_pos.set(self.find_next_word_pos());
    }

    //fn move_cursor_up(&self) {}

    //fn move_cursor_down(&self) {}

    fn find_previous_word_pos(&self) -> usize {
        let mut current_pos = self.cursor_pos.get();
        let text = self.value.get();

        while current_pos > 0 {
            let c = text.text.chars().nth(current_pos - 1).unwrap();
            current_pos -= 1;
            if c.is_whitespace() || WORD_BREAK_CHARS.contains(&c) {
                break;
            }
        }

        current_pos
    }

    fn find_next_word_pos(&self) -> usize {
        let mut current_pos = self.cursor_pos.get();
        let text = self.value.get();

        while current_pos < text.text.len() {
            let c = text.text.chars().nth(current_pos).unwrap();
            current_pos += 1;
            if c.is_whitespace() || WORD_BREAK_CHARS.contains(&c) {
                break;
            }
        }

        current_pos
    }
}

impl Widget for TextInput {
    fn build(
        &mut self,
        _ctx: &mut BuildContext,
        _: Observer<WidgetMouseState>,
        _: Observer<bool>,
    ) -> BuildResult {
        let mut res = BuildResult::default();
        res.draw_rect(DrawRect::fill_and_stroke(
            self.editing.map(|s| {
                if *s {
                    Some(ColorId::PrimaryVariantDark)
                } else {
                    Some(ColorId::Primary)
                }
            }),
            self.editing.map(|s| {
                if *s {
                    Some((ColorId::OnPrimary, 2.5))
                } else {
                    None
                }
            }),
        ));
        res.draw_text(self.value.observe());

        res.push_item(DrawItem::new(CursorDrawer {
            cursor_pos: self.cursor_pos.observe(),
            text: self.value.observe(),
        }));

        res
    }

    fn on_text_input(&self, input: String, _app_interface: AppInterface) {
        let mut text = self.value.get().clone();
        text.text.insert_str(self.cursor_pos.get(), &input);
        self.value.set(text);
        self.cursor_pos.set(self.cursor_pos.get() + input.len());
    }

    fn on_key_down(&self, key: KeyCode, mods: KeyMods, _interface: AppInterface) {
        match (key, mods) {
            (KeyCode::Backspace, KeyMods { l_alt: true, .. }) => {
                self.delete_previous_word();
            }
            (KeyCode::Backspace, _) => {
                self.delete_previous();
            }
            (KeyCode::Delete, KeyMods { l_alt: true, .. }) => {
                self.delete_next_word();
            }
            (KeyCode::Delete, _) => {
                self.delete_next();
            }
            (KeyCode::Left, KeyMods { l_alt: true, .. }) => {
                self.move_cursor_previous_word();
            }
            (KeyCode::Left, _) => {
                self.move_cursor_left();
            }
            (KeyCode::Right, KeyMods { l_alt: true, .. }) => {
                self.move_cursor_next_word();
            }
            (KeyCode::Right, _) => {
                self.move_cursor_right();
            }
            _ => {}
        }
    }

    fn on_mouse_down(&self, event: &MouseEvent, _interface: AppInterface) {
        if event.inside {
            self.on_gain_focus();
        }
    }

    fn on_gain_focus(&self) {
        self.cursor_pos.set(self.value.get().text.len());
        self.editing.set(true);
    }

    fn on_lose_focus(&self) {
        self.editing.set(false);
    }
}
