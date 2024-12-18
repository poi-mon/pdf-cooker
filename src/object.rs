use std::pin::Pin;
use pin_project::*;

use crate::primitive::*;

#[pin_project]
pub struct Object {
    #[pin]
    inner: Vec<Primitive>
}

impl Object {
    pub fn new(inner: Vec<Primitive>) -> Pin<Box<Object>> {
        Box::pin(Object { inner })
    }
}
