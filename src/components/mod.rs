pub mod borders;
pub mod rects;
pub mod tables;

use crate::haru_pdf::HaruPDF;

pub trait PdfComponent {
    fn draw(&self, pdf: &mut HaruPDF);
}
