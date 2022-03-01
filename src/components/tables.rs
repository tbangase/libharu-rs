use crate::components::PdfComponent;
use crate::alignments::Alignment;
use crate::colors::Cmyk;
use crate::components::borders::{Border, Outline};
// use crate::padding::Padding;
use crate::components::rects::RRect;
use crate::entities::Position;


#[allow(dead_code)]
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
        let header_row = self.header_text.clone().len();
        let header_col = self.header_text[0].clone().len();

        println!("");
        println!("Header Row   : {:?}", header_row);
        println!("Header Column: {:?}", header_col);

        let col_width = 80.0 as f32;
        let row_height = 20.0 as f32;

        let mut center_x = self.x.clone() + col_width / 2.0;
        let mut center_y = self.y.clone() + row_height / 2.0;

        for (row, row_texts) in self.header_text.iter().enumerate() {
            for (column, _text) in row_texts.iter().enumerate() {
                let mut outline = Outline::from_border(self.ruled_line.clone());

                if row == 0 {
                    outline.set_top(self.out_frame.clone());
                }

                if column == 0 {
                    outline.set_left(self.out_frame.clone());
                } else if column == row_texts.len() - 1 {
                    outline.set_right(self.out_frame.clone());
                }

                // TODO
                RRect::new()
                    .set_position(Position::new(col_width, row_height))
                    .draw(pdf);

                // self.set_text_in_rrect(RRect {
                //     page: self.page,
                //     font: self.font,
                //     center_x,
                //     center_y,
                //     width: col_width,
                //     height: row_height,
                //     horizontal_align: self.header_alignment.horizontal_align.clone(),
                //     vertical_align: self.header_alignment.vertical_align.clone(),
                //     cornor_radius: 0.0,
                //     text: text.clone(),
                //     font_size: 10.0,
                //     bg_color: self.header_fill_color.clone(),
                //     text_color: self.header_text_color.clone(),
                //     padding: Padding::new_vh(2.0, 5.0),
                //     line_space: 0.0,
                //     wrap: true,
                //     outline,
                // });
                center_x += col_width;
            }

            center_x = self.x.clone() + col_width / 2.0;
            center_y -= row_height;
        }

        center_y += row_height / 2.0;
        let row_height = 40.0 as f32;
        center_y -= row_height / 2.0;

        // TODO: 高さ、幅の自動計算
        // 高さは全要素で一番高いものに合わせる
        // それぞれ縦列で一番広いものに合わせる
        for (row, row_texts) in self.body_text.iter().enumerate() {
            for (column, _text) in row_texts.iter().enumerate() {
                let mut outline = Outline::from_border(self.ruled_line.clone());

                // if row == 0 {
                //     outline.set_top(self.out_frame.clone());
                //     println!("set top blank.");
                // } else
                if row == self.body_text.len() - 1 {
                    outline.set_bottom(self.out_frame.clone());
                }

                if column == 0 {
                    outline.set_left(self.out_frame.clone());
                } else if column == row_texts.len() - 1 {
                    outline.set_right(self.out_frame.clone());
                }

                // self.set_text_in_rrect(RRect {
                //     page: self.page,
                //     font: self.font,
                //     center_x,
                //     center_y,
                //     width: col_width,
                //     height: row_height,
                //     horizontal_align: self.body_alignment.horizontal_align.clone(),
                //     vertical_align: self.body_alignment.vertical_align.clone(),
                //     cornor_radius: 0.0,
                //     text: text.clone(),
                //     font_size: 10.0,
                //     bg_color: if row % 2 == 0 {
                //         self.body_fill_color.clone()
                //     } else {
                //         match self.alt_fill_color.clone() {
                //             Some(val) => val,
                //             None => self.body_fill_color.clone(),
                //         }
                //     },
                //     text_color: self.body_text_color.clone(),
                //     padding: Padding::new_vh(2.0, 5.0),
                //     line_space: 0.0,
                //     wrap: true,
                //     outline: outline.clone(),
                // });
                center_x += col_width;
            }

            center_x = self.x.clone() + col_width / 2.0;
            center_y -= row_height;
        }
        

        println!("{:?}", pdf);
        println!("Now Rendering...");
    }
}
