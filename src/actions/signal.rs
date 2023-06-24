use super::receiver::Receiver;

// Store receivers in a list and call them when the signal is emitted.
pub struct Signal<T> {
    receivers: Vec<Box<dyn Receiver>>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Default for Signal<T> {
    fn default() -> Self {
        Self {
            receivers: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> Signal<T> {
    pub fn connect<R>(&mut self, receiver: Box<R>)
    where
        R: Receiver + 'static,
    {
        self.receivers.push(receiver);
    }

    pub fn emmit(&self) {
        for receiver in &self.receivers {
            //receiver.action();
        }
    }
}
