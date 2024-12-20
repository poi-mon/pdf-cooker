use crate::object::*;
use crate::primitive::*;

use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Page {
    pub resource: Resource,
    mediabox: MediaBox,
    content: Content,
}

impl Page {
    pub fn new(mediabox: MediaBox) -> Page {
        Page {
            resource: Resource::new(),
            mediabox,
            content: Content {},
        }
    }
}

impl From<Page> for Vec<Object> {
    fn from(target: Page) -> Vec<Object> {
        let resource: Object = target.resource.into();

        let page = Object::new(map![
            "Type" => "Page",
            "Resource" => &resource,
            "MediaBox" => target.mediabox
        ]);

        vec![resource, page]
    }    
}

#[derive(Eq, Hash, Debug)]
struct Font {
    base: String,
    identifier: String,
}

impl PartialEq for Font {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base
    }
}

impl From<Font> for Pair {
    fn from(target: Font) -> Pair {
        pair!(target.identifier => map![
            "Type" => "Font",
            "BaseFont" => target.base,
            "SubType" => "Type1"
        ])
    }
}

#[derive(Debug)]
pub struct Resource {
    fonts: HashSet<Font>,
}

impl Resource {
    pub fn new() -> Resource {
        Resource {
            fonts: HashSet::new(),
        }
    }

    pub fn add_font(&mut self, base: impl Into<String>) {
        let base = base.into();        
        if !self.fonts.contains(&Font { base: base.clone(), identifier: String::from("") }) {
            self.fonts.insert(Font { base, identifier: format!("F{}", self.fonts.len()) });
        }
    }
}

impl From<Resource> for Object {
    fn from(target: Resource) -> Object {
        Object::new(map![
            "Font" => map![target.fonts],
            "Parent" => Primitive::Parent
        ])
    }
}

#[derive(Debug)] 
pub enum MediaBox {
    A4
}

impl From<MediaBox> for Primitive {
    fn from(target: MediaBox) -> Primitive {
        match target {
            MediaBox::A4 => array![0, 0, 595, 842],
        }
    }
}

#[derive(Debug)]
pub struct Content {

}