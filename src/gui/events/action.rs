pub trait Action<T> {
    fn invoke(&mut self, event: T);
}

impl<Fn, A> Action<A> for Fn
where
    Fn: FnMut(A),
{
    fn invoke(&mut self, event: A) {
        self(event);
    }
}
