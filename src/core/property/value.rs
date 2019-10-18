pub enum Value<'a, T> {
    Val(T),
    Ref(&'a T),
}

impl<'a, T> AsRef<T> for Value<'a, T> {
    fn as_ref(&self) -> &T {
        match self {
            Self::Val(ref v) => v,
            Self::Ref(r) => r,
        }
    }
}
