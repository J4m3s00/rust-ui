use super::{event::Event, receiver::Rec};

pub trait Sig {
    type Event: Event;
    fn emmit(&mut self, event: Self::Event);
}
