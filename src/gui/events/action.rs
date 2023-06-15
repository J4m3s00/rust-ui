use super::event::Event;

pub trait Action {
    fn invoke(&mut self, event: Box<dyn Event>);
}

impl<Fn> Action for Fn
where
    Fn: FnMut(Box<dyn Event>),
{
    fn invoke(&mut self, event: Box<dyn Event>) {
        self(event);
    }
}
