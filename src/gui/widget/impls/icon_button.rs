use crate::prelude::*;

use super::clickable::{Clickable, Clicked};

pub struct IconButton {}

impl IconButton {
    pub fn new<T>(icon: IconType, callback: T) -> WidgetInstance
    where
        T: Receiver<Clicked> + 'static,
    {
        ZStack::new(vec![Clickable::new(callback), Icon::new(icon)])
    }
}
