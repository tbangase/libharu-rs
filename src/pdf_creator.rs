use libc::c_void;
#[allow(dead_code)]
// use printpdf::*;
use libharu_sys::*;
// use std::io::BufWriter;
use std::ffi::CString;
use std::fs::File;

//use crate::libraries::haru_rs::haru;

#[allow(dead_code)]
pub struct CustomPdf {
    fontfile: File,
}

#[allow(dead_code)]
#[derive(Clone)]
enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}

#[allow(dead_code)]
#[derive(Clone)]
enum HorizontalAlignment {
    Left,
    Center,
    Right,
    Justify,
}

#[derive(Clone)]
struct Alignment {
    horizontal_align: HorizontalAlignment,
    vertical_align: VerticalAlignment,
}

impl Alignment {
    pub fn center() -> Self {
        return Self {
            horizontal_align: HorizontalAlignment::Center,
            vertical_align: VerticalAlignment::Center,
        };
    }

    pub fn top_left() -> Self {
        return Self {
            horizontal_align: HorizontalAlignment::Left,
            vertical_align: VerticalAlignment::Top,
        };
    }

    pub fn bottom_right() -> Self {
        return Self {
            horizontal_align: HorizontalAlignment::Right,
            vertical_align: VerticalAlignment::Bottom,
        };
    } 
}

#[derive(Clone, Debug)]
struct Rgb {
    r: f32,
    g: f32,
    b: f32,
}

impl Rgb {
    pub fn green() -> Self {
        Self {
            r: 0.0,
            g: 1.0,
            b: 0.0,
        }
    }

    pub fn blue() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 1.0,
        }
    }

    pub fn sky_blue() -> Self {
        Self {
            r: 0.7,
            g: 0.8,
            b: 1.0
        }
    }

    pub fn black() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn white() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }

    pub fn dark_grey() -> Self {
        Self {
            r: 0.2,
            g: 0.2,
            b: 0.2,
        }
    }

    pub fn grey() -> Self {
        Self {
            r: 0.3,
            g: 0.3,
            b: 0.3,
        }
    }

    pub fn yellow() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 0.5,
        }
    }

    pub fn red() -> Self {
        Self {
            r: 1.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn pink() -> Self {
        Self {
            r: 1.0,
            g: 0.7,
            b: 0.7,
        }
    }

    pub fn orange() -> Self {
        Self {
            r: 1.0,
            g: 0.6,
            b: 0.3,
        }
    } 

    pub fn light_grey() -> Self {
        Self {
            r: 0.7,
            g: 0.7,
            b: 0.7,
        }
    }

    pub fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }
}

#[derive(Clone)]
struct Padding {
    top: f32,
    bottom: f32,
    right: f32,
    left: f32,
}

#[allow(dead_code)]
impl Padding {
    pub fn new_all(all: f32) -> Self {
        Self {
            top: all,
            bottom: all,
            right: all,
            left: all,
        }
    }

    pub fn new_vh(vertical: f32, horizontal: f32) -> Self {
        Self {
            top: vertical,
            bottom: vertical,
            right: horizontal,
            left: horizontal,
        }
    }

    pub fn new(top: f32, bottom: f32, right: f32, left: f32) -> Self {
        Self {
            top,
            bottom,
            right,
            left,
        }
    }

    pub fn none() -> Self {
        Self {
            top: 0.0,
            bottom: 0.0,
            right: 0.0,
            left: 0.0,
        }
    }
}

#[derive(Clone)]
struct RRect {
    page: HPDF_Page,
    font: HPDF_Font,
    center_x: f32,
    center_y: f32,
    width: f32,
    height: f32,
    horizontal_align: HorizontalAlignment,
    vertical_align: VerticalAlignment,
    cornor_radius: f32,
    text: String,
    font_size: f32,
    bg_color: Rgb,
    text_color: Rgb,
    padding: Padding,
    line_space: f32,
    wrap: bool,
    outline: Outline,
}

#[derive(Clone, Debug)]
struct Outline {
    top: Border,
    bottom: Border,
    left: Border,
    right: Border,
}

#[allow(dead_code)]
impl Outline {
    pub fn new(enable: bool, width: f32, color: Rgb) -> Self {
        let border = Border {
            enable,
            width,
            color,
        };
        return Self {
            top: border.clone(),
            bottom: border.clone(),
            left: border.clone(),
            right: border.clone(),
        };
    }

    pub fn from_border(border: Border) -> Self {
        return Self {
            top: border.clone(),
            bottom: border.clone(),
            left: border.clone(),
            right: border.clone(),
        };
    }

    pub fn set_top(&mut self, border: Border) {
        self.top = border;
    }

    pub fn set_bottom(&mut self, border: Border) {
        self.bottom = border;
    }

    pub fn set_left(&mut self, border: Border) {
        self.left = border;
    }

    pub fn set_right(&mut self, border: Border) {
        self.right = border;
    }
}

#[derive(Clone, Debug)]
struct Border {
    enable: bool,
    width: f32,
    color: Rgb,
}

impl Border {
    pub fn no_border() -> Self {
        Self {
            enable: false,
            width: 0.0,
            color: Rgb::white(),
        }
    }
}

#[derive(Clone)]
struct Table {
    page: HPDF_Page,
    font: HPDF_Font,
    x: f32,
    y: f32,
    header_alignment: Alignment,
    body_alignment: Alignment,
    header_fill_color: Rgb,
    body_fill_color: Rgb,
    header_text_color: Rgb,
    body_text_color: Rgb,
    // border_color: Rgb,
    header_text: Vec<Vec<String>>,
    body_text: Vec<Vec<String>>,
    alt_fill_color: Option<Rgb>,
    ruled_line: Border,
    out_frame: Border,
}

impl CustomPdf {
    pub fn new() -> Self {
        let maru_gothic = File::open("fonts/honoka-marugo2/Honoka-Shin-Maru-Gothic_R.ttf").unwrap();
        Self {
            fontfile: maru_gothic,
        }
    }

    pub fn create_reciept_pdf(&self, file_name: String) {
        // Cライブラリを使用する場合は安全でない可能性があるので
        // Unsafe Rustで書く
        unsafe {
            const CSTRING_ERROR_MSG: &str = "CString::new failed.";
            const A4_WIDTH_LANDSCAPE: f32 = 842.0;
            const A4_HEIGHT_LANDSCAPE: f32 = 595.0;

            extern "C" fn error_handler(
                error_no: HPDF_STATUS,
                detail_no: HPDF_STATUS,
                user_data: *mut c_void,
            ) {
                println!(
                    "Error Hexacode: {0:x}  Detail No: {1:?}",
                    error_no, detail_no
                );
                println!("User Data Address: {:?}", user_data);
            }

            println!("Start creating pdf.");

            let null_pointer: *mut c_void = std::ptr::null_mut();
            let pdf: HPDF_Doc = HPDF_New(error_handler, null_pointer);

            let page1: HPDF_Page = HPDF_AddPage(pdf);

            HPDF_Page_SetSize(
                page1,
                HPDF_PageSizes::HPDF_PAGE_SIZE_A4,
                HPDF_PageDirection::HPDF_PAGE_LANDSCAPE,
            );
            // まずはフォントを指定する
            HPDF_UseUTFEncodings(pdf);

            let font_path = CString::new("fonts/honoka-marugo2/Honoka-Shin-Maru-Gothic_R.ttf")
                .expect(CSTRING_ERROR_MSG);

            let font_name = HPDF_LoadTTFontFromFile(pdf, font_path.as_ptr(), HPDF_TRUE);
            let encoding = CString::new("UTF-8").expect(CSTRING_ERROR_MSG);

            HPDF_SetCurrentEncoder(pdf, encoding.as_ptr());

            let font = HPDF_GetFont(pdf, font_name, encoding.as_ptr());

            let tsukiji_font_path = CString::new("fonts/GL-Tsukiji/GL-Tsukiji-2go.ttf")
                .expect(CSTRING_ERROR_MSG);
            let tsukiji_font_name = HPDF_LoadTTFontFromFile(pdf, tsukiji_font_path.as_ptr(), HPDF_TRUE);
            let tsukiji_font = HPDF_GetFont(pdf, tsukiji_font_name, encoding.as_ptr());

            // 丸角矩形テキストボックスの追加
            //HPDF_Page_SetCharSpace(page1, 10.0);
            self.set_text_in_rrect(RRect {
                page: page1,
                font,
                center_x: A4_WIDTH_LANDSCAPE / 2.0,
                center_y: A4_HEIGHT_LANDSCAPE / 2.0 + 125.0,
                width: 160.0,
                height: 35.0,
                horizontal_align: HorizontalAlignment::Center,
                vertical_align: VerticalAlignment::Center,
                cornor_radius: 5.0,
                text: "受 領 書".to_string(),
                font_size: 28.0,
                bg_color: Rgb::dark_grey(),
                text_color: Rgb::light_grey(),
                padding: Padding::none(),
                line_space: 0.0,
                wrap: false,
                outline: Outline::new(true, 2.0, Rgb::dark_grey()),
            });

            // 受領書 白枠
            self.set_text_in_rrect(RRect {
                page: page1,
                font,
                center_x: A4_WIDTH_LANDSCAPE / 2.0,
                center_y: A4_HEIGHT_LANDSCAPE / 2.0 + 80.0,
                width: 160.0,
                height: 35.0,
                horizontal_align: HorizontalAlignment::Center,
                vertical_align: VerticalAlignment::Center,
                cornor_radius: 5.0,
                text: "受　領　書".to_string(),
                font_size: 24.0,
                bg_color: Rgb::white(),
                text_color: Rgb::light_grey(),
                padding: Padding::new_vh(0.0, 0.0),
                line_space: 0.0,
                wrap: false,
                outline: Outline::new(true, 2.0, Rgb::dark_grey()),
            });

            // self.set_text_in_rrect(RRect {
            //     page: page1,
            //     font: font,
            //     center_x: A4_WIDTH_LANDSCAPE / 2.0 + 250.0,
            //     center_y: A4_HEIGHT_LANDSCAPE / 2.0 + 80.0,
            //     width: 160.0,
            //     height: 45.0,
            //     horizontal_align: HorizontalAlignment::Center,
            //     vertical_align: VerticalAlignment::Center,
            //     cornor_radius: 5.0,
            //     text: "Title\nSub Title Sub Title".to_string(),
            //     font_size: 11.0,
            //     bg_color: Rgb::white(),
            //     text_color: Rgb::orange(),
            //     padding: Padding::new_vh(0.0, 0.0),
            //     line_space: -10.0,
            //     wrap: true,
            //     outline: Outline::new(true, 2.0, Rgb::pink()),
            // });

            //HPDF_Page_SetTextLeading(page1, 20.0);
            self.set_text_in_rrect(RRect {
                page: page1,
                font,
                center_x: 100.0,
                center_y: A4_HEIGHT_LANDSCAPE / 2.0,
                width: 30.0,
                height: 150.0,
                horizontal_align: HorizontalAlignment::Center,
                vertical_align: VerticalAlignment::Center,
                cornor_radius: 5.0,
                text: "お 客 様 名".to_string(),
                font_size: 14.0,
                bg_color: Rgb::black(),
                text_color: Rgb::white(),
                padding: Padding::new_all(0.0),
                line_space: 20.0,
                wrap: true,
                outline: Outline::new(true, 2.0, Rgb::black()),
            });

            //HPDF_Page_SetTextLeading(page1, 30.0);
            self.set_text_in_rrect(RRect {
                page: page1,
                font,
                center_x: 150.0,
                center_y: A4_HEIGHT_LANDSCAPE / 2.0,
                width: 30.0,
                height: 150.0,
                horizontal_align: HorizontalAlignment::Center,
                vertical_align: VerticalAlignment::Center,
                cornor_radius: 5.0,
                text: "お 客 様 名".to_string(),
                font_size: 20.0,
                bg_color: Rgb::black(),
                text_color: Rgb::white(),
                padding: Padding::new_all(0.0),
                line_space: 6.0,
                wrap: true,
                outline: Outline::new(true, 2.0, Rgb::black()),
            });

            self.set_text_in_rrect(RRect {
                page: page1,
                font,
                center_x: 200.0,
                center_y: A4_HEIGHT_LANDSCAPE / 2.0,
                width: 30.0,
                height: 150.0,
                horizontal_align: HorizontalAlignment::Center,
                vertical_align: VerticalAlignment::Center,
                cornor_radius: 15.0,
                text: "お 客 様 名".to_string(),
                font_size: 18.0,
                bg_color: Rgb::yellow(),
                text_color: Rgb::from_rgb(0.9, 0.1, 0.1),
                padding: Padding::new_all(0.0),
                line_space: 10.0,
                wrap: true,
                outline: Outline::new(true, 2.0, Rgb::dark_grey()),
            });

            HPDF_Page_SetTextLeading(page1, 0.0);
            // HPDF_Page_SetCharSpace(page1, 0.0);
            self.set_text_in_rrect(RRect {
                page: page1,
                font,
                center_x: A4_WIDTH_LANDSCAPE * 0.2,
                center_y: A4_HEIGHT_LANDSCAPE * 0.8,
                width: 150.0,
                height: 150.0,
                horizontal_align: HorizontalAlignment::Right,
                vertical_align: VerticalAlignment::Bottom,
                cornor_radius: 0.0,
                text: "右下に 寄せる".to_string(),
                font_size: 24.0,
                bg_color: Rgb::grey(),
                text_color: Rgb::green(),
                padding: Padding::new_all(10.0),
                line_space: 0.0,
                wrap: true,
                outline: Outline::new(true, 2.0, Rgb::grey()),
            });

            // 画像を挿入する
            let image_path = CString::new("images/mitsuibau_logo.jpeg").expect(CSTRING_ERROR_MSG);
            let image: HPDF_Image = HPDF_LoadJpegImageFromFile(pdf, image_path.as_ptr());
            let image_size: HPDF_Point = HPDF_Image_GetSize(image);
            HPDF_Page_DrawImage(
                page1,
                image,
                A4_WIDTH_LANDSCAPE - image_size.x - 30.0,
                A4_HEIGHT_LANDSCAPE - image_size.y - 50.0,
                image_size.x,
                image_size.y,
            );

            // 表を作成する
            let header_text: Vec<Vec<String>> = vec![vec![
                "header 1".to_string(),
                "header 2".to_string(),
                "header 3".to_string(),
            ]];

            let body_text: Vec<Vec<String>> = vec![
                vec![
                    "body 11".to_string(),
                    "body 12".to_string(),
                    "body 13".to_string(),
                ],
                vec![
                    "body 21".to_string(),
                    "body 22\nbody 22".to_string(),
                    "body 23".to_string(),
                ],
                vec![
                    "body 31".to_string(),
                    "body 32".to_string(),
                    "body 33".to_string(),
                ],
            ];

            self.set_table(Table {
                page: page1,
                font,
                x: A4_WIDTH_LANDSCAPE / 2.0 - 150.0,
                y: A4_HEIGHT_LANDSCAPE / 2.0 + 30.0,
                header_alignment: Alignment::center(),
                body_alignment: Alignment::top_left(),
                header_fill_color: Rgb::black(),
                body_fill_color: Rgb::white(),
                header_text_color: Rgb::white(),
                body_text_color: Rgb::dark_grey(),
                header_text: header_text.clone(),
                body_text: body_text.clone(),
                alt_fill_color: Some(Rgb::from_rgb(1.0, 1.0, 0.8)),
                ruled_line: Border {
                    enable: true,
                    width: 2.0,
                    color: Rgb::dark_grey(),
                },
                out_frame: Border {
                    enable: true,
                    width: 2.0,
                    color: Rgb::dark_grey(),
                },
            });

            let header_text: Vec<Vec<String>> = vec![vec![
                "header 1".to_string(),
                "header 2".to_string(),
                "header 3".to_string(),
                "header 4".to_string(),
                "header 5".to_string(),
                "header 6".to_string(),
            ]];

            let body_text: Vec<Vec<String>> = vec![
                vec![
                    "body 11".to_string(),
                    "body 12".to_string(),
                    "body 13".to_string(),
                    "".to_string(),
                    "body 15".to_string(),
                    "body 16".to_string(),
                ],
                vec![
                    "body 21".to_string(),
                    "body 22\nbody 22".to_string(),
                    "body 23".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "body 26".to_string(),
                ],
                vec![
                    "body 31".to_string(),
                    "body 32".to_string(),
                    "body 33".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                ],
            ];

            self.set_table(Table {
                page: page1,
                font,
                x: A4_WIDTH_LANDSCAPE / 2.0 - 150.0,
                y: A4_HEIGHT_LANDSCAPE / 2.0 - 150.0,
                header_alignment: Alignment::center(),
                body_alignment: Alignment::bottom_right(),
                header_fill_color: Rgb::orange(),
                body_fill_color: Rgb::white(),
                header_text_color: Rgb::white(),
                body_text_color: Rgb::dark_grey(),
                header_text: header_text.clone(),
                body_text: body_text.clone(),
                alt_fill_color: Some(Rgb::from_rgb(1.0, 1.0, 0.8)),
                ruled_line: Border {
                    enable: true,
                    width: 0.7,
                    color: Rgb::pink(),
                },
                out_frame: Border {
                    enable: true,
                    width: 2.0,
                    color: Rgb::red(),
                },
            });


            let header_text: Vec<Vec<String>> = vec![vec![
                "へっだー 1".to_string(),
                "へっだー 2".to_string(),
                "へっだー 3".to_string(),
            ]];

            let body_text: Vec<Vec<String>> = vec![
                vec![
                    "ぼでい 11".to_string(),
                    "ぼでい 12".to_string(),
                    "ぼでい 13".to_string(),
                ],
                vec![
                    "ぼでい 21".to_string(),
                    "ぼでい 22\nかいぎょうぼでい".to_string(),
                    "ぼでい 23".to_string(),
                ],
                vec![
                    "ボディー 31".to_string(),
                    "中身 32".to_string(),
                    "こんてんつ こんてんつ こんてんつ こんてんつ 33".to_string(),
                ],
            ];

            self.set_table(Table {
                page: page1,
                font: tsukiji_font,
                x: A4_WIDTH_LANDSCAPE / 2.0 + 150.0,
                y: A4_HEIGHT_LANDSCAPE / 2.0 + 30.0,
                header_alignment: Alignment::center(),
                body_alignment: Alignment::top_left(),
                header_fill_color: Rgb::blue(),
                body_fill_color: Rgb::white(),
                header_text_color: Rgb::white(),
                body_text_color: Rgb::dark_grey(),
                header_text,
                body_text,
                alt_fill_color: Some(Rgb::from_rgb(0.9, 0.95, 1.0)),
                ruled_line: Border {
                    enable: true,
                    width: 1.0,
                    color: Rgb::sky_blue(),
                },
                out_frame: Border::no_border(),
            });



            let file_name: CString = CString::new(file_name).expect(CSTRING_ERROR_MSG);
            HPDF_SaveToFile(pdf, file_name.as_ptr());

            println!("File Saved!");

            HPDF_Free(pdf);
        }
    }

    unsafe fn set_text_in_rrect(&self, rrect: RRect) {
        const CSTRING_ERROR_MSG: &str = "CString::new failed.";
        // Set Graphic States

        // Start new path
        // Switch GMODE to PATH_OBJECT
        // Draw RRect from RRect info
        self.set_rrect(rrect.clone());
        let mut rect_width = rrect.width - rrect.cornor_radius * 2.0;
        let mut rect_height = rrect.height - rrect.cornor_radius * 2.0;

        // Rendering Text
        // Switch GMODE to TEXT_OBJECT
        HPDF_Page_BeginText(rrect.page);

        let start_x = rrect.center_x - rrect.width / 2.0 + rrect.padding.left;
        let start_y = rrect.center_y - rrect.height / 2.0 + rrect.padding.bottom;
        rect_width += rrect.cornor_radius * 2.0 - rrect.padding.left - rrect.padding.right;
        rect_height += rrect.cornor_radius * 2.0 - rrect.padding.top - rrect.padding.bottom;

        // Set Text State
        let font_size = rrect.font_size;
        HPDF_Page_SetFontAndSize(rrect.page, rrect.font, font_size.clone());
        let txt_color = rrect.text_color;
        HPDF_Page_SetRGBFill(rrect.page, txt_color.r, txt_color.g, txt_color.b);

        // Show Text

        let text_cstring: CString = CString::new(rrect.text.clone()).expect(CSTRING_ERROR_MSG);
        let len: *mut u32 = &mut 0;

        // Textを高さ方向センターに位置づけるための計算

        // Calculating One line height
        let bbox = HPDF_Font_GetBBox(rrect.font);
        let line_height = ((bbox.top - bbox.bottom) * font_size / 1000.0) + rrect.line_space;
        HPDF_Page_SetTextLeading(rrect.page, line_height);

        // Calculating Lines
        // 1文字でrect_widthをはみ出す場合に正しく計算できない。
        // その場合は文字数を返したい
        let trimmed_text = rrect.text.replace(" ", "").replace("　", "");
        let trimmed_ctext = CString::new(trimmed_text.clone()).expect(CSTRING_ERROR_MSG);
        let text_em_width =
            HPDF_Font_TextWidth(rrect.font, trimmed_ctext.as_ptr(), rrect.text.len() as u32).width;
        let text_width = (text_em_width as f32) * font_size / 1000.0;

        let mut lines = (text_width / rect_width).ceil();

        // println!("Trimmed Text Length: {:?}", trimmed_text.clone().len());
        let char_count = trimmed_text.len() as f32 / 3.0;
        if lines > char_count {
            lines = char_count;
        }

        let text_height = if rrect.wrap {
            line_height * lines - rrect.line_space
        } else {
            line_height
        };

        let box_center = start_y + rect_height / 2.0;

        let top_y = match rrect.vertical_align {
            VerticalAlignment::Top => start_y + rect_height,
            VerticalAlignment::Center => box_center + text_height / 2.0,
            VerticalAlignment::Bottom => start_y + text_height,
        };

        let hpdf_horiz_alignment = match rrect.horizontal_align {
            HorizontalAlignment::Left => HPDF_TextAlignment::HPDF_TALIGN_LEFT,
            HorizontalAlignment::Center => HPDF_TextAlignment::HPDF_TALIGN_CENTER,
            HorizontalAlignment::Right => HPDF_TextAlignment::HPDF_TALIGN_RIGHT,
            HorizontalAlignment::Justify => HPDF_TextAlignment::HPDF_TALIGN_JUSTIFY,
        };

        HPDF_Page_TextRect(
            rrect.page,
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
        HPDF_Page_EndText(rrect.page);
    }

    fn set_rrect(&self, rrect: RRect) {
        // set fill
        self.set_rrect_fill(rrect.clone());
        // set border
        self.set_rrect_stroke(rrect.clone());
    }

    fn set_rrect_fill(&self, rrect: RRect) {
        unsafe {
            let bg_color = rrect.bg_color;
            HPDF_Page_SetRGBFill(rrect.page, bg_color.r, bg_color.g, bg_color.b);
            HPDF_Page_SetLineWidth(rrect.page, 0.0);

            // 左下の弧
            if rrect.cornor_radius > 0.0 {
                HPDF_Page_Arc(
                    rrect.page,
                    rrect.center_x - rrect.width / 2.0 + rrect.cornor_radius,
                    rrect.center_y - rrect.height / 2.0 + rrect.cornor_radius,
                    rrect.cornor_radius,
                    180.0,
                    270.0,
                );
            } else {
                HPDF_Page_MoveTo(
                    rrect.page,
                    rrect.center_x - rrect.width / 2.0,
                    rrect.center_y - rrect.height / 2.0,
                );
            }

            // 左の辺
            HPDF_Page_LineTo(
                rrect.page,
                rrect.center_x - rrect.width / 2.0,
                rrect.center_y + rrect.height / 2.0 - rrect.cornor_radius,
            );

            // 左上の弧
            if rrect.cornor_radius > 0.0 {
                HPDF_Page_Arc(
                    rrect.page,
                    rrect.center_x - rrect.width / 2.0 + rrect.cornor_radius,
                    rrect.center_y + rrect.height / 2.0 - rrect.cornor_radius,
                    rrect.cornor_radius,
                    270.0,
                    360.0,
                );
            }

            // 上の辺
            HPDF_Page_LineTo(
                rrect.page,
                rrect.center_x + rrect.width / 2.0 - rrect.cornor_radius,
                rrect.center_y + rrect.height / 2.0,
            );

            // 右上の弧
            if rrect.cornor_radius > 0.0 {
                HPDF_Page_Arc(
                    rrect.page,
                    rrect.center_x + rrect.width / 2.0 - rrect.cornor_radius,
                    rrect.center_y + rrect.height / 2.0 - rrect.cornor_radius,
                    rrect.cornor_radius,
                    0.0,
                    90.0,
                );
            }

            // 右の辺
            HPDF_Page_LineTo(
                rrect.page,
                rrect.center_x + rrect.width / 2.0,
                rrect.center_y - rrect.height / 2.0 + rrect.cornor_radius,
            );

            // 右下の弧
            if rrect.cornor_radius > 0.0 {
                HPDF_Page_Arc(
                    rrect.page,
                    rrect.center_x + rrect.width / 2.0 - rrect.cornor_radius,
                    rrect.center_y - rrect.height / 2.0 + rrect.cornor_radius,
                    rrect.cornor_radius,
                    90.0,
                    180.0,
                );
            }

            // パスを閉じる
            HPDF_Page_LineTo(
                rrect.page,
                rrect.center_x - rrect.width / 2.0 + rrect.cornor_radius,
                rrect.center_y - rrect.height / 2.0,
            );

            HPDF_Page_Fill(rrect.page);
        }
    }

    fn set_rrect_stroke(&self, rrect: RRect) {
        unsafe {
            HPDF_Page_SetLineJoin(rrect.page, HPDF_LineJoin::HPDF_ROUND_JOIN);
            HPDF_Page_SetLineCap(rrect.page, HPDF_LineCap::HPDF_ROUND_END);

            let border = rrect.outline.left.clone();
            let line_color = border.color;
            HPDF_Page_SetRGBStroke(rrect.page, line_color.r, line_color.g, line_color.b);
            HPDF_Page_SetLineWidth(rrect.page, border.width.clone());

            let width = rrect.width;
            let height = rrect.height;

            // 左下の弧
            let border = rrect.outline.left.clone();
            if rrect.outline.bottom.enable.clone() && rrect.outline.left.enable.clone() {
                if rrect.cornor_radius > 0.0 {
                    HPDF_Page_Arc(
                        rrect.page,
                        rrect.center_x - width / 2.0 + rrect.cornor_radius,
                        rrect.center_y - height / 2.0 + rrect.cornor_radius,
                        rrect.cornor_radius,
                        180.0,
                        270.0,
                    );
                } else {
                    HPDF_Page_MoveTo(
                        rrect.page,
                        rrect.center_x - width / 2.0,
                        rrect.center_y - height / 2.0,
                    );
                }
            } else {
                HPDF_Page_MoveTo(
                    rrect.page,
                    rrect.center_x - width / 2.0,
                    rrect.center_y - height / 2.0,
                );
            }

            // 左の辺
            if rrect.outline.left.enable.clone() {
                HPDF_Page_LineTo(
                    rrect.page,
                    rrect.center_x - width / 2.0,
                    rrect.center_y + height / 2.0 - rrect.cornor_radius - border.width / 2.0,
                );

                // 左上の弧
                if rrect.cornor_radius > 0.0 {
                    HPDF_Page_Arc(
                        rrect.page,
                        rrect.center_x - width / 2.0 + rrect.cornor_radius,
                        rrect.center_y + height / 2.0 - rrect.cornor_radius,
                        rrect.cornor_radius,
                        270.0,
                        360.0,
                    );
                }
            } else {
                HPDF_Page_MoveTo(
                    rrect.page,
                    rrect.center_x - width / 2.0 + rrect.cornor_radius,
                    rrect.center_y + height / 2.0,
                );
            }

            // 一度描画して設定し直せるようにする
            HPDF_Page_Stroke(rrect.page);

            let border = rrect.outline.top.clone();
            let line_color = border.color;
            HPDF_Page_SetRGBStroke(rrect.page, line_color.r, line_color.g, line_color.b);
            HPDF_Page_SetLineWidth(rrect.page, border.width.clone());

            HPDF_Page_MoveTo(
                rrect.page,
                rrect.center_x - width / 2.0 + rrect.cornor_radius,
                rrect.center_y + height / 2.0,
            );

            // 上の辺
            if rrect.outline.top.enable.clone() {
                HPDF_Page_LineTo(
                    rrect.page,
                    rrect.center_x + width / 2.0 - rrect.cornor_radius,
                    rrect.center_y + height / 2.0,
                );

                // 右上の弧
                if rrect.cornor_radius > 0.0 {
                    HPDF_Page_Arc(
                        rrect.page,
                        rrect.center_x + width / 2.0 - rrect.cornor_radius,
                        rrect.center_y + height / 2.0 - rrect.cornor_radius,
                        rrect.cornor_radius,
                        0.0,
                        90.0,
                    );
                }
            } else {
                HPDF_Page_MoveTo(
                    rrect.page,
                    rrect.center_x + width / 2.0,
                    rrect.center_y + height / 2.0 - rrect.cornor_radius,
                );
            }

            // 一度描画して設定し直せるようにする
            HPDF_Page_Stroke(rrect.page);

            let border = rrect.outline.right.clone();
            let line_color = border.color;
            HPDF_Page_SetRGBStroke(rrect.page, line_color.r, line_color.g, line_color.b);
            HPDF_Page_SetLineWidth(rrect.page, border.width.clone());

            HPDF_Page_MoveTo(
                rrect.page,
                rrect.center_x + width / 2.0,
                rrect.center_y + height / 2.0 - rrect.cornor_radius,
            );

            // 右の辺
            if rrect.outline.right.enable.clone() {
                HPDF_Page_LineTo(
                    rrect.page,
                    rrect.center_x + width / 2.0,
                    rrect.center_y - height / 2.0 + rrect.cornor_radius,
                );

                // 右下の弧
                if rrect.cornor_radius > 0.0 {
                    HPDF_Page_Arc(
                        rrect.page,
                        rrect.center_x + width / 2.0 - rrect.cornor_radius,
                        rrect.center_y - height / 2.0 + rrect.cornor_radius,
                        rrect.cornor_radius,
                        90.0,
                        180.0,
                    );
                }
            } else {
                HPDF_Page_MoveTo(
                    rrect.page,
                    rrect.center_x + width / 2.0 - rrect.cornor_radius,
                    rrect.center_y - height / 2.0,
                );
            }

            // 一度描画して設定し直せるようにする
            HPDF_Page_Stroke(rrect.page);

            let border = rrect.outline.bottom.clone();

            // パスを閉じる
            if border.enable.clone() {
                let line_color = border.color;
                HPDF_Page_SetRGBStroke(rrect.page, line_color.r, line_color.g, line_color.b);
                HPDF_Page_SetLineWidth(rrect.page, border.width.clone());
                HPDF_Page_MoveTo(
                    rrect.page,
                    rrect.center_x + width / 2.0 - rrect.cornor_radius,
                    rrect.center_y - height / 2.0,
                );

                HPDF_Page_LineTo(
                    rrect.page,
                    rrect.center_x - width / 2.0 + rrect.cornor_radius,
                    rrect.center_y - height / 2.0,
                );

                HPDF_Page_Stroke(rrect.page);
            }
        }
    }

    fn set_table(&self, table: Table) {
        unsafe {
            let header_row = table.header_text.clone().len();
            let header_col = table.header_text[0].clone().len();

            println!("");
            println!("Header Row   : {:?}", header_row);
            println!("Header Column: {:?}", header_col);

            let col_width = 80.0 as f32;
            let row_height = 20.0 as f32;

            let mut center_x = table.x.clone() + col_width / 2.0;
            let mut center_y = table.y.clone() + row_height / 2.0;

            for (row, row_texts) in table.header_text.iter().enumerate() {
                for (column, text) in row_texts.iter().enumerate() {
                    let mut outline = Outline::from_border(table.ruled_line.clone());

                    if row == 0 {
                        outline.set_top(table.out_frame.clone());
                    } 

                    if column == 0 {
                        outline.set_left(table.out_frame.clone());
                    } else if column == row_texts.len() - 1 {
                        outline.set_right(table.out_frame.clone());
                    }

                    self.set_text_in_rrect(RRect {
                        page: table.page,
                        font: table.font,
                        center_x,
                        center_y,
                        width: col_width,
                        height: row_height,
                        horizontal_align: table.header_alignment.horizontal_align.clone(),
                        vertical_align: table.header_alignment.vertical_align.clone(),
                        cornor_radius: 0.0,
                        text: text.clone(),
                        font_size: 10.0,
                        bg_color: table.header_fill_color.clone(),
                        text_color: table.header_text_color.clone(),
                        padding: Padding::new_vh(2.0, 5.0),
                        line_space: 0.0,
                        wrap: true,
                        outline,
                    });
                    center_x += col_width;
                }

                center_x = table.x.clone() + col_width / 2.0;
                center_y -= row_height;
            }

            center_y += row_height / 2.0;
            let row_height = 40.0 as f32;
            center_y -= row_height / 2.0;

            // TODO: 高さ、幅の自動計算
            // 高さは全要素で一番高いものに合わせる
            // それぞれ縦列で一番広いものに合わせる
            for (row, row_texts) in table.body_text.iter().enumerate() {
                for (column, text) in row_texts.iter().enumerate() {
                    let mut outline = Outline::from_border(table.ruled_line.clone());

                    // if row == 0 {
                    //     outline.set_top(table.out_frame.clone());
                    //     println!("set top blank.");
                    // } else
                    if row == table.body_text.len() - 1 {
                        outline.set_bottom(table.out_frame.clone());
                    }

                    if column == 0 {
                        outline.set_left(table.out_frame.clone());
                    } else if column == row_texts.len() - 1 {
                        outline.set_right(table.out_frame.clone());
                    }

                    self.set_text_in_rrect(RRect {
                        page: table.page,
                        font: table.font,
                        center_x,
                        center_y,
                        width: col_width,
                        height: row_height,
                        horizontal_align: table.body_alignment.horizontal_align.clone(),
                        vertical_align: table.body_alignment.vertical_align.clone(),
                        cornor_radius: 0.0,
                        text: text.clone(),
                        font_size: 10.0,
                        bg_color: if row % 2 == 0 {
                            table.body_fill_color.clone()
                        } else {
                            match table.alt_fill_color.clone() {
                                Some(val) => val,
                                None => table.body_fill_color.clone(),
                            }
                        },
                        text_color: table.body_text_color.clone(),
                        padding: Padding::new_vh(2.0, 5.0),
                        line_space: 0.0,
                        wrap: true,
                        outline: outline.clone(),
                    });
                    center_x += col_width;
                }

                center_x = table.x.clone() + col_width / 2.0;
                center_y -= row_height;
            }
        }
    }
}
