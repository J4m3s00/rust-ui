use crate::prelude::*;

pub struct Clicked(pub MouseEvent);

pub struct Clickable {
    mouse_state: Observer<WidgetMouseState>,
    on_click: Box<dyn Receiver<Clicked>>,
}

impl Clickable {
    pub fn new<T>(on_click: T) -> WidgetInstance
    where
        T: Receiver<Clicked> + 'static,
    {
        Self {
            mouse_state: WidgetMouseState::Normal.into(),
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
        mouse_state: &State<WidgetMouseState>,
    ) -> BuildResult {
        self.mouse_state = mouse_state.observe(); // To make mouse up work. Not sure how to pass/store the mouse state in the widget itself

        let mut res = BuildResult::default();

        res.draw_rect(DrawRect::fill(mouse_state.map(move |v| match v {
            WidgetMouseState::Normal => Some(ColorId::Primary),
            WidgetMouseState::Hovered => Some(ColorId::PrimaryVariantLight),
            WidgetMouseState::Pressed => Some(ColorId::PrimaryVariantDark),
        })));
        res
    }

    fn on_mouse_up(&self, event: &MouseEvent, interface: AppInterface) {
        if let Some(WidgetMouseState::Pressed) = self.mouse_state.get() {
            self.on_click.action(Clicked(event.clone()), interface);
        }
    }

    fn on_key_down(&self, key: KeyCode, _interface: AppInterface) {
        if KeyCode::Return == key {
            self.on_click
                .action(Clicked(MouseEvent::default()), _interface);
        }
    }

    /*fn on_mouse_enter(&self, _event: MouseEvent, _interface: AppInterface) {
        self.mouse_state.set(MouseState::Hovered);
    }

    fn on_mouse_leave(&self, _event: MouseEvent, _interface: AppInterface) {
        self.mouse_state.set(MouseState::Normal);
    }*/
}
