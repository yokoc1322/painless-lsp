#![allow(unused)]
pub trait ScopeFunc {
    fn tap<R, F>(&self, block: F) -> R
    where
        F: FnOnce(&Self) -> R;

    fn tap_mut<R, F>(&mut self, block: F) -> R
    where
        F: FnMut(&mut Self) -> R;

    fn tap_owned<R, F>(self, block: F) -> R
    where
        Self: Sized,
        F: FnOnce(Self) -> R;

    fn also<F>(self, block: F) -> Self
    where
        F: FnOnce(&Self);
}

impl<T> ScopeFunc for T {
    fn tap<R, F>(&self, block: F) -> R
    where
        F: FnOnce(&Self) -> R,
    {
        block(self)
    }

    fn tap_mut<R, F>(&mut self, mut block: F) -> R
    where
        F: FnMut(&mut Self) -> R,
    {
        block(self)
    }

    fn tap_owned<R, F>(self, block: F) -> R
    where
        Self: Sized,
        F: FnOnce(Self) -> R,
    {
        block(self)
    }

    fn also<F>(self, block: F) -> Self
    where
        F: FnOnce(&Self),
    {
        block(&self);
        self
    }
}
