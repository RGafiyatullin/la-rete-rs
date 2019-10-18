use super::ArcDynHEq;
use super::Value;

pub trait Property<In, V> {
    fn id(&self) -> String;

    fn heq(&self) -> ArcDynHEq<V>;

    fn value<'a>(&self, input: &'a In) -> Value<'a, V>;
}
