use crate::components::PdfComponent;
use crate::alignments::Alignment;
use crate::colors::Cmyk;
use crate::components::borders::Border;

#[derive(Clone)]
struct Table {
    x: f32,
    y: f32,
    header_alignment: Alignment,
    body_alignment: Alignment,
    header_fill_color: Cmyk,
    body_fill_color: Cmyk,
    header_text_color: Cmyk,
    body_text_color: Cmyk,
    // border_color: Cmyk,
    header_text: Vec<Vec<String>>,
    body_text: Vec<Vec<String>>,
    alt_fill_color: Option<Cmyk>,
    ruled_line: Border,
    out_frame: Border,
}

impl PdfComponent for Table {
    fn draw(&self, pdf: &mut crate::haru_pdf::HaruPDF) {
        println!("{:?}", pdf);
        println!("Now Rendering...");
    }
}
