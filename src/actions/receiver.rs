use crate::gui::app::interface::AppInterface;

pub trait Receiver<T> {
    fn action(&self, data: T, interface: AppInterface);
}

impl<T, F> Receiver<T> for F
where
    F: Fn(T, AppInterface),
{
    fn action(&self, data: T, interface: AppInterface) {
        self(data, interface)
    }
}
