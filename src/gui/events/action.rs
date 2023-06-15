pub trait ActionRunner {
    type Event;
    fn invoke(&mut self, event: Self::Event);
}

impl<Fn, A> ActionRunner for Fn
where
    Fn: FnMut(A),
{
    type Event = A;
    fn invoke(&mut self, event: Self::Event) {
        self(event);
    }
}
