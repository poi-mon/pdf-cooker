use std::pin::Pin;
use std::marker::PhantomPinned;

use std::collections::HashMap;

use pin_project::*;

use crate::primitive::*;

#[derive(Debug, Clone)]
#[pin_project]
pub struct Object {
    inner: Vec<Primitive>,
    _pinned: PhantomPinned,
}

impl Object {
    pub fn new(inner: impl Into<Vec<Primitive>>) -> Pin<Box<Object>> {
        Box::pin(Object { inner: inner.into(), _pinned: PhantomPinned })
    }

    pub fn resolve(objects: &mut Vec<Pin<Box<Object>>>) {
        let mut query: HashMap<*const Object, u64> = HashMap::new();
        let mut xref: HashMap<*const Object, *const Object> = HashMap::new();

        for (number, object) in objects.iter_mut().enumerate() {
            let selfref = object.as_ref().get_ref() as *const Object;
            query.insert(selfref, number as u64);
            for prim in object.as_mut().project().inner.iter_mut() {
                for pp in prim.iter_mut() {
                    if let Primitive::Defer(ref ptr) = pp {
                        xref.insert(ptr.clone(), selfref);
                    }
                }
            }
        }

        for object in objects.iter_mut() {
            let selfref = object.as_ref().get_ref() as *const Object;
            for prim in object.as_mut().project().inner.iter_mut() {
                for pp in prim.iter_mut() {
                    if let Primitive::Defer(ref ptr) = pp {
                        // let number = query.get(ptr).expect("unresolved reference");
                        // *pp = Primitive::Ref(*number);
                        if let Some(number) = query.get(ptr) {
                            *pp = Primitive::Ref(*number);
                        }
                    } else if let Primitive::Parent = pp {
                        let from = xref.get(&selfref).expect("unresolved parent reference");
                        let number = query.get(from).expect("unresolved indirectrly reference");
                        *pp = Primitive::Ref(*number);
                    }
                }
            }
        }
    }
}

impl From<&Pin<Box<Object>>> for Primitive {
    fn from(pin: &Pin<Box<Object>>) -> Primitive {
        Primitive::Defer(pin.as_ref().get_ref() as *const Object)
    }
}

impl From<&Object> for Primitive {
    fn from(obj: &Object) -> Primitive {
        Primitive::Defer(obj as *const Object)
    }
}