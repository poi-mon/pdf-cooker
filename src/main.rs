use pdf_cooker::*;

use std::pin::Pin;

#[derive(Debug)]
pub enum MediaBox {
    A1, A2, A3, A4
}

#[derive(Debug)]
pub struct Page {
    mediabox: MediaBox,
}

impl Page {
    pub fn new() -> Page {
        Page {
            mediabox: MediaBox::A4,
        }
    }

    pub fn paging(pages: Vec<Pin<Box<Object>>>) -> Vec<Pin<Box<Object>>> {
        let mut prev = pages.len(); 
        let mut whole = 1;
        while prev > 2 {
            prev = prev / 2 + prev % 2;
            whole += prev;
        }
        let mut prev: Vec<Pin<Box<Object>>> = vec![];
        let mut iter = pages.iter();
        while let Some(a) = iter.next() {
            if let Some(b) = iter.next() {
                prev.push(Object::new(array![a, b]));
            } else {
                prev.push(Object::new(array![a]));
            }
        }
        let mut all = prev.clone();
        while prev.len() > 2 {
            let mut nextion = vec![];
            let mut iter = prev.iter();
            while let Some(a) = iter.next() {
                if let Some(b) = iter.next() {
                    nextion.push(Object::new(array![a, b]));
                } else {
                    nextion.push(Object::new(array![a]));
                }
            }
            all.extend(nextion.clone());
            prev = nextion;
        }

        all.extend(pages);
        all
    }
}

impl From<MediaBox> for Primitive {
    fn from(mediabox: MediaBox) -> Primitive {
        match mediabox {
            MediaBox::A4 => Primitive::Array(vec![Primitive::Number(1), Primitive::Number(2)]),
            _ => todo!()
        }
    }
}

impl From<Page> for Vec<Primitive> {
    fn from(page: Page) -> Vec<Primitive> {
        vec![page.mediabox.into()]
    }
}

pub fn main() {
    let mut obj = vec![
        Object::new(vec![]), 
        Object::new(vec![]), 
        Object::new(vec![]),
        Object::new(vec![])
        // Object::new(vec![]) 
        // Object::new(vec![]),
        // Object::new(vec![]), 
        // Object::new(vec![]), 
        // Object::new(vec![]),
        // Object::new(vec![]), 
        // Object::new(vec![])
    ];
    let mut paged = Page::paging(obj);
    Object::resolve(&mut paged);

    println!("{:#?}", paged);
}