pub enum MaybeOwned<'a, T> {
    Owned(T),
    Borrowed(&'a T),
}

impl<T> AsRef<T> for MaybeOwned<'_, T> {
    fn as_ref(&self) -> &T {
        match self {
            MaybeOwned::Owned(e) => &e,
            MaybeOwned::Borrowed(e) => e,
        }
    }
}
