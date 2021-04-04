use crate::prelude::Indexable;

#[derive(Debug)]
pub struct Le<T>(pub T);

impl<T: Clone> Clone for Le<T> {
    fn clone(&self) -> Self {
        Le(self.0.clone())
    }
}

impl<T: Indexable> Indexable for Le<T> {
    fn as_usize(&self) -> usize {
        self.0.as_usize()
    }

    fn from_usize(u: usize) -> Self {
        Self(T::from_usize(u))
    }
}
