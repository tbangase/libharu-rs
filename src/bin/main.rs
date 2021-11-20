use libharu_rs::{components::rects::Position, haru_pdf::HaruPDF, pages::{ PageSize, PageDirection}};
use libharu_rs::components::*;

pub fn main() {
    println!("Hello, world!");

    let mut pdf = HaruPDF::new_doc();

    pdf.add_page(PageSize::A4, PageDirection::Landscape);

    let rrec = rects::RRect::new()
        .set_position(Position::new(100.0, 100.0))
        .set_text("Hello, World!".to_string()); 

    pdf.add_component(rrec);

    pdf.save_doc("test.pdf");
}
