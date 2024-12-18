use crate::object::*;

pub enum Primitive {
    Array(Vec<Primitive>),
    Map(Vec<Pair>),
    Number(u64),
    Name(String),
    Defer(*const Object),
}

pub struct Pair(String, Box<Primitive>);

pub enum XPrimitive {
    Array(Vec<XPrimitive>),
    Map(Vec<XPair>),
    Number(u64),
    Name(String),
    Ref(u64),
}

pub struct XPair(String, Box<XPrimitive>);