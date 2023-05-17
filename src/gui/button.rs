use bevy_ecs::prelude::{Bundle, Component};

use super::widget::Widget;

#[derive(Component)]
pub struct ButtonComponent {
    pub text: String,
}


#[derive(Bundle)]
pub struct Button {
    widget: Widget,
    button: ButtonComponent,
}
