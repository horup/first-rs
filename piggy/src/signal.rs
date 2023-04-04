use std::vec::Drain;

#[derive(Default)]
pub struct Signal<T:Clone> {
    signals:Vec<T>
}

impl<T:Clone> Signal<T> {
    pub fn new() -> Self {
        Self {
            signals:Vec::with_capacity(64)
        }
    }

    pub fn push(&mut self, t:T) {
        self.signals.push(t);
    }

    pub fn drain(&mut self) -> Drain<T> {
        self.signals.drain(..)
    }
}