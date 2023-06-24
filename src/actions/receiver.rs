pub trait Receiver {
    fn action(self);
}

impl<T> Receiver for T
where
    T: FnOnce(),
{
    fn action(self) {
        self();
    }
}
