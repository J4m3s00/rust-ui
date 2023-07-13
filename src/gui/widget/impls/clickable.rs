use crate::prelude::*;

pub struct Clicked(pub MouseEvent);

pub struct Clickable {
    on_click: Box<dyn Receiver<Clicked>>,
}

impl Clickable {
    pub fn new<T>(on_click: T) -> WidgetInstance
    where
        T: Receiver<Clicked> + 'static,
    {
        Self {
            on_click: Box::new(on_click),
        }
        .instance()
        .accept_input()
    }
}

impl Widget for Clickable {
    fn build(
        &mut self,
        _ctx: &mut BuildContext,
        mouse_state: Observer<WidgetMouseState>,
        _: Observer<bool>,
    ) -> BuildResult {
        let mut res = BuildResult::default();

        res.draw_rect(DrawRect::fill(mouse_state.map(move |v| match v {
            WidgetMouseState::Normal => Some(ColorId::Primary),
            WidgetMouseState::Hovered => Some(ColorId::PrimaryVariantLight),
            WidgetMouseState::Pressed => Some(ColorId::PrimaryVariantDark),
        })));
        res
    }

    fn on_mouse_up(&self, event: &MouseEvent, interface: AppInterface) {
        if event.inside {
            self.on_click.action(Clicked(event.clone()), interface);
        }
    }

    fn on_key_down(&self, key: KeyCode, _: KeyMods, _interface: AppInterface) {
        if KeyCode::Return == key {
            self.on_click
                .action(Clicked(MouseEvent::default()), _interface);
        }
    }
}
