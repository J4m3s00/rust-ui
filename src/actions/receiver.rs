pub trait Receiver<T> {
    fn action(&self, data: T);
}

impl<T, F> Receiver<T> for F
where
    F: Fn(T),
{
    fn action(&self, data: T) {
        self(data)
    }
}
