use pdf_cooker::*;

pub fn main() {
    let mut page = Page::new(MediaBox::A4);
    page.resource.add_font("Times-New-Roman");
    page.resource.add_font("Calib");
    let page: Vec<Object> = page.into();

    let mut doc = Document::new();
    doc.appendix(page);
    let _ = doc.encode();
}