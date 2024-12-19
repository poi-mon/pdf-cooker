use std::cell::RefCell;
use std::rc::Rc;

use crate::object::*;

#[derive(Debug, Clone)]
pub enum Primitive {
    Array(Vec<Primitive>),
    Map(Vec<Pair>),
    Number(u64),
    Name(String),
    Defer(*const Object),
    Parent,
    Ref(u64),
    Dummy,
}

impl Primitive {
    pub fn iter_mut(&mut self) -> PrimitiveIterMut {
        PrimitiveIterMut(vec![self])
    }
}

pub struct PrimitiveIterMut<'a>(Vec<&'a mut Primitive>);

impl<'a> Iterator for PrimitiveIterMut<'a> {
    type Item = &'a mut Primitive;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(current) = self.0.pop() {
            match current {
                Primitive::Array(array) => {
                    self.0.extend(array.iter_mut());
                    continue;
                },
                Primitive::Map(pairs) => {
                    for pair in pairs {
                        self.0.push(&mut *pair.1);
                    }
                    continue;
                },
                _ => {}
            }
            return Some(current);
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct Pair(pub String, pub Box<Primitive>);

impl Into<Vec<Primitive>> for Primitive {
    fn into(self) -> Vec<Primitive> {
        vec![self]
    }
 }

impl Into<Primitive> for &str {
    fn into(self) -> Primitive {
        Primitive::Name(self.to_string())
    }
}

#[macro_export]
macro_rules! map {
    () => {
        Primitive::Map(Vec::new())
    };
    ($($key:expr => $value:expr),*) => {
        Primitive::Map(vec![
            $(
                Pair($key.into(), Box::new($value.into()))
            ),*
        ])
    }
}

#[macro_export]
macro_rules! array {
    () => {
        Primitive::Array(Vec::new())
    };
    ($($elm:expr),*) => {
        Primitive::Array(vec![
            $(
                $elm.into()
            ),*
        ])
    };
}

#[macro_export]
macro_rules! parent {
    () => {
        Primitive::Parent
    };
}