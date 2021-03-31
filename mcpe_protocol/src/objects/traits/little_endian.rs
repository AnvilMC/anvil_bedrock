use crate::prelude::Indexable;

#[derive(Debug)]
pub struct Le<T>(pub T);

impl<T: Indexable> Indexable for Le<T> {
    fn as_usize(&self) -> usize {
        self.0.as_usize()
    }

    fn from_usize(u: usize) -> Self {
        Self(T::from_usize(u))
    }
}
