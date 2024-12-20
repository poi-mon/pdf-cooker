use crate::primitive::*;
use crate::uid;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Object {
    uid: u32,
    pub inner: Vec<Primitive>,
}

impl Object {
    pub fn new(inner: impl Into<Vec<Primitive>>) -> Object {
        Object {
            uid: uid::issue(),
            inner: inner.into(),
        }
    }

    pub fn encode(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
        writer.write_fmt(format_args!("{} 0 obj\n", self.uid))?;
        for prim in self.inner.iter() {
            prim.encode(0, writer)?;
        }
        writer.write_str("\nendobj\n")?;
        Ok(())
    }

    pub fn resolve_parent_reference(objects: &mut Vec<Object>) {
        let mut xref: HashMap<u32, u32> = HashMap::new();

        objects.iter().for_each(|obj| 
            obj.inner.iter().for_each(|prim| linking(obj.uid, prim, &mut xref))
        );

        objects.iter_mut().for_each(|obj| 
            obj.inner.iter_mut().for_each(|prim| resolve(obj.uid, prim, &mut xref))
        );

        fn linking(uid: u32, prim: &Primitive, xref: &mut HashMap<u32, u32>) {
            match prim {
                Primitive::Array(array) => array.iter().for_each(|elm| linking(uid, elm, xref)),
                Primitive::Map(pairs) => pairs.iter().for_each(|pair| linking(uid, &pair.value, xref)),
                Primitive::Ref(child) => { xref.insert(*child, uid); },
                _ => {}
            }
        }

        fn resolve(uid: u32, prim: &mut Primitive, xref: &mut HashMap<u32, u32>) {
            match prim {
                Primitive::Array(array) => array.iter_mut().for_each(|elm| resolve(uid, elm, xref)),
                Primitive::Map(pairs) => pairs.iter_mut().for_each(|pair| resolve(uid, &mut pair.value, xref)),
                Primitive::Parent => {
                    let parent_uid = xref.get(&uid).expect("unlinked reference");
                    *prim = Primitive::Ref(*parent_uid);
                },
                _ => {}
            }
        }
    }
} 

impl From<&Object> for Primitive {
    fn from(target: &Object) -> Primitive {
        Primitive::Ref(target.uid)
    }
}

impl From<Object> for Vec<Object> {
    fn from(target: Object) -> Vec<Object> {
        vec![target]
    }
}

impl<T> From<T> for Object where T: Into<Primitive> {
    fn from(target: T) -> Object {
        Object::new(target.into())
    }
}