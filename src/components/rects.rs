use crate::alignments::Alignment;
use crate::colors::Cmyk;
use crate::constants::CSTRING_ERROR_MSG;
use crate::padding::Padding;
use crate::components::borders::Outline;
use crate::haru_pdf::HaruPDF;
use crate::alignments::{ VerticalAlignment, HorizontalAlignment };
use libharu_sys::*;
use std::ffi::CString;

use super::PdfComponent;

#[derive(Clone, Debug)]
pub struct Position {
    x: f32, y:f32
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x, y
        }
    }
}

#[derive(Clone, Debug)]
pub struct RRect {
    center_position: Position,
    width: f32,
    height: f32,
    alignment: Alignment,
    cornor_radius: f32,
    text: String,
    font_size: f32,
    bg_color: Cmyk,
    text_color: Cmyk,
    padding: Padding,
    line_space: f32,
    wrap: bool,
    outline: Outline,
}

impl RRect {
    pub fn new() -> Self {
        let center_position = Position {
            x: 100.0, y: 100.0
        };
        let width = 200.0;
        let height = 200.0;
        let alignment = Alignment::center();
        let cornor_radius = 5.0;
        let text = "Hello, World!".to_string();
        let text_color = Cmyk::dark_grey();
        let font_size = 10.0;
        let bg_color = Cmyk::black();
        let padding = Padding::new_all(10.0);
        let line_space = 0.0;
        let wrap = true;
        let outline = Outline::new(true, 1.0, Cmyk::cian());

        Self {
            center_position,
            width, height,
            alignment,
            cornor_radius,
            text,
            font_size,
            bg_color,
            text_color,
            padding,
            line_space,
            wrap,
            outline,
        }
    }

    pub fn set_position(mut self, position: Position) -> Self {
        self.center_position = position;
        self
    }

    pub fn set_text(mut self, text: String) -> Self {
        self.text = text;
        self
    }
    pub fn set_rrect_fill(&self, pdf: &mut HaruPDF) {
        
    }

    pub fn set_rrect_stroke(&self, pdf: &mut HaruPDF) {
        
    }
}

impl PdfComponent for RRect {
    fn draw(&self, pdf: &mut HaruPDF) {
        unsafe {
            if let Some(page) = pdf.current_page {

                // Set Graphic States

                // Start new path
                // Switch GMODE to PATH_OBJECT
                // Draw RRect from RRect info
                self.set_rrect_fill(pdf);
                self.set_rrect_stroke(pdf);

                let mut rect_width = self.width - self.cornor_radius * 2.0;
                let mut rect_height = self.height - self.cornor_radius * 2.0;

                // Rendering Text
                // Switch GMODE to TEXT_OBJECT
                HPDF_Page_BeginText(page);

                let start_x = self.center_position.x - self.width / 2.0 + self.padding.left;
                let start_y = self.center_position.y - self.height / 2.0 + self.padding.bottom;
                rect_width += self.cornor_radius * 2.0 - self.padding.left - self.padding.right;
                rect_height += self.cornor_radius * 2.0 - self.padding.top - self.padding.bottom;

                // Set Text State
                let font_size = self.font_size;
                HPDF_Page_SetFontAndSize(page, pdf.font, font_size.clone());
                let txt_color = self.text_color;
                HPDF_Page_SetCMYKFill(page, txt_color.c, txt_color.m, txt_color.y, txt_color.k);

                // Show Text

                let text_cstring: CString = CString::new(self.text.clone()).expect(CSTRING_ERROR_MSG);
                let len: *mut u32 = &mut 0;

                // Textを高さ方向センターに位置づけるための計算

                // Calculating One line height
                let bbox = HPDF_Font_GetBBox(pdf.font);
                let line_height = ((bbox.top - bbox.bottom) * font_size / 1000.0) + self.line_space;
                HPDF_Page_SetTextLeading(page, line_height);

                // Calculating Lines
                // 1文字でrect_widthをはみ出す場合に正しく計算できない。
                // その場合は文字数を返したい
                let trimmed_text = self.text.replace(" ", "").replace("　", "");
                let trimmed_ctext = CString::new(trimmed_text.clone()).expect(CSTRING_ERROR_MSG);
                let text_em_width =
                    HPDF_Font_TextWidth(pdf.font, trimmed_ctext.as_ptr(), self.text.len() as u32).width;
                let text_width = (text_em_width as f32) * font_size / 1000.0;

                let mut lines = (text_width / rect_width).ceil();

                // println!("Trimmed Text Length: {:?}", trimmed_text.clone().len());
                let char_count = trimmed_text.len() as f32 / 3.0;
                if lines > char_count {
                    lines = char_count;
                }

                let text_height = if self.wrap {
                    line_height * lines - self.line_space
                } else {
                    line_height
                };

                let box_center = start_y + rect_height / 2.0;

                let top_y = match self.alignment.vertical_align {
                    VerticalAlignment::Top => start_y + rect_height,
                    VerticalAlignment::Center => box_center + text_height / 2.0,
                    VerticalAlignment::Bottom => start_y + text_height,
                };

                let hpdf_horiz_alignment = match self.alignment.horizontal_align {
                    HorizontalAlignment::Left => HPDF_TextAlignment::HPDF_TALIGN_LEFT,
                    HorizontalAlignment::Center => HPDF_TextAlignment::HPDF_TALIGN_CENTER,
                    HorizontalAlignment::Right => HPDF_TextAlignment::HPDF_TALIGN_RIGHT,
                    HorizontalAlignment::Justify => HPDF_TextAlignment::HPDF_TALIGN_JUSTIFY,
                };

                HPDF_Page_TextRect(
                    page,
                    start_x,
                    top_y,
                    start_x + rect_width,
                    start_y,
                    text_cstring.as_ptr(),
                    hpdf_horiz_alignment,
                    len,
                );
                // println!("{:?} characters rendered.", *len);
                // println!("");
                HPDF_Page_EndText(page);
            }
        }
    }
}

