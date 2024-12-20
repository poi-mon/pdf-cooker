use crate::object::*;
use crate::primitive::*;

use bytes::{BytesMut, BufMut};
use std::fmt::Write;

#[derive(Debug)]
pub struct Document {
    objects: Vec<Object>,
}

impl Document {
    pub fn new() -> Document {
        Document {
            objects: vec![],
        }
    }

    pub fn appendix(&mut self, objects: impl Into<Vec<Object>>) {
        self.objects.extend(objects.into());
    }

    pub fn encode(&mut self) -> std::fmt::Result {
        let mut bytes: BytesMut = BytesMut::new();
        bytes.write_str("%PDF-1.7\n")?;
        bytes.extend_from_slice(&[0xe2, 0xe3, 0xcf, 0xd3, b'\n']);

        let pages_ref: Vec<&Object> = 
            self.objects
                .iter()
                .filter(|obj| obj.inner.iter().any(|prim| prim.is_type("Page")))
                .collect();

        let pages = Object::new(
            map![
                "Type" => "Pages",
                "Count" => pages_ref.len(),
                "Kids" => pages_ref
            ]
        );

        let catalog = Object::new(
            map![
                "Pages" => &pages,
                "Type" => "Catalog"
             ]
        );

        self.appendix(catalog);
        self.appendix(pages);

        Object::resolve_parent_reference(&mut self.objects);

        self.objects.iter().for_each(|obj| {
            obj.encode(&mut bytes);
        });

        // trailer
        

        bytes.iter().for_each(|byte| if byte.is_ascii() {
            print!("{}", *byte as char);
        } else {
            print!("[{}]", *byte);
        });

        Ok(())
    }
}