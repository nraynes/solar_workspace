use yaml_serde::{
    Mapping, Number, Sequence, Value,
    value::{Tag, TaggedValue},
};

pub struct Yaml {}

impl Yaml {
    fn new() -> Self {
        Self {}
    }

    pub fn root<F, T>(f: F) -> Value
    where
        F: Fn(Self) -> T,
        T: IntoIterator<Item = (Value, Value)>,
    {
        Value::Mapping(Mapping::from_iter(f(Self::new())))
    }

    pub fn mapping<F, T>(&self, f: F) -> Value
    where
        F: Fn(Self) -> T,
        T: IntoIterator<Item = (Value, Value)>,
    {
        Value::Mapping(Mapping::from_iter(f(Self::new())))
    }

    pub fn empty_mapping(&self) -> Value {
        Value::Mapping(Mapping::new())
    }

    pub fn sequence<F, T>(&self, f: F) -> Value
    where
        F: Fn(Self) -> T,
        T: IntoIterator<Item = Value>,
    {
        Value::Sequence(Sequence::from_iter(f(Self::new())))
    }

    pub fn empty_sequence(&self) -> Value {
        Value::Sequence(Sequence::new())
    }

    pub fn tagged<F>(&self, tag: &str, f: F) -> Value
    where
        F: Fn(Self) -> Value,
    {
        Value::Tagged(Box::new(TaggedValue {
            tag: Tag::new(tag),
            value: f(Self::new()),
        }))
    }

    pub fn string<T>(&self, s: T) -> Value
    where
        T: AsRef<str>,
    {
        Value::String(s.as_ref().into())
    }

    pub fn bool(&self, b: bool) -> Value {
        Value::Bool(b)
    }

    pub fn number<T>(&self, n: T) -> Value
    where
        T: Into<Number>,
    {
        Value::Number(Number::from(n.into()))
    }
}
