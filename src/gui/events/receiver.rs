pub trait Rec {
    type Event;
    fn receive(&mut self, event: Self::Event);
}
