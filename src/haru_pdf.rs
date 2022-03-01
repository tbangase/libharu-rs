use crate::components::PdfComponent;
use dispose::{Disposable, Dispose};
use libc::c_void;
use libharu_sys::*;
use std::ffi::CString;
use crate::constants::*;
pub use crate::error::PdfError;
use crate::pages::{PageDirection, PageSize};

#[derive(Debug)]
pub struct HaruPDF {
    pub doc: HPDF_Doc,
    pub current_page: Option<HPDF_Page>,
    pub font: HPDF_Font,
}

impl Dispose for HaruPDF {
    fn dispose(self) {
        unsafe {
            HPDF_Free(self.doc);
        }
    }
}

/// Functions about PDF Configs
impl HaruPDF {
    pub fn new_doc() -> Disposable<Self> {
        unsafe {
            let null_pointer: *mut c_void = std::ptr::null_mut();
            let pdf_error = PdfError::error_handler;
            let doc: HPDF_Doc = HPDF_New(pdf_error, null_pointer.clone());

            HPDF_UseJPFonts(doc);

            let font_name = CString::new("MS-Gothic").expect(CSTRING_ERROR_MSG);
            let encoding_name = CString::new("UTF-8").expect(CSTRING_ERROR_MSG);

            let font = HPDF_GetFont(doc, font_name.as_ptr(), encoding_name.as_ptr());

            println!("Set MS-Gothic Font");

            return Self {
                doc,
                current_page: None,
                font,
            }
            .into();
        }
    }

    pub fn save_doc(&self, file_name: &str) {
        unsafe {
            let file_name = CString::new(file_name).expect(CSTRING_ERROR_MSG);
            HPDF_SaveToFile(self.doc, file_name.as_ptr());
        }
    }

    pub fn add_page(&mut self, size: PageSize, direction: PageDirection) {
        unsafe {
            self.current_page = Some(HPDF_AddPage(self.doc));
            let page_size = Self::get_page_size(size);
            let page_direction = Self::get_page_direction(direction);

            match self.current_page {
                Some(val) => {
                    HPDF_Page_SetSize(val, page_size, page_direction);
                }
                None => println!("Internal Error. Adding page failed."),
            }
        }
    }

    pub fn insert_page(&mut self, size: PageSize, direction: PageDirection) {
        // TODO: Modify to inserting with index
        unsafe {
            self.current_page = Some(HPDF_InsertPage(self.doc, self.current_page.unwrap()));
            let page_size = Self::get_page_size(size);
            let page_direction = Self::get_page_direction(direction);

            match self.current_page {
                Some(val) => {
                    HPDF_Page_SetSize(val, page_size, page_direction);
                }
                None => println!("Internal Error. Adding page failed."),
            }
        }
    }

    pub fn set_font(&mut self, font_path: String) {
        // set font
        unsafe {
            HPDF_UseUTFEncodings(self.doc);
            let font_path = CString::new(font_path).expect(CSTRING_ERROR_MSG);

            let font_name = HPDF_LoadTTFontFromFile(self.doc, font_path.as_ptr(), HPDF_TRUE);
            let encoding = CString::new("UTF-8").expect(CSTRING_ERROR_MSG);

            HPDF_SetCurrentEncoder(self.doc, encoding.as_ptr());

            self.font = HPDF_GetFont(self.doc, font_name, encoding.as_ptr());
        }
    }
}

/// Functions for getting PDF Information
impl HaruPDF {
    fn get_page_size(size: PageSize) -> HPDF_PageSizes {
        use crate::pages::PageSize::*;
        match size {
            A3 => HPDF_PageSizes::HPDF_PAGE_SIZE_A3,
            A4 => HPDF_PageSizes::HPDF_PAGE_SIZE_A4,
            A5 => HPDF_PageSizes::HPDF_PAGE_SIZE_A5,
            B4 => HPDF_PageSizes::HPDF_PAGE_SIZE_B4,
            B5 => HPDF_PageSizes::HPDF_PAGE_SIZE_B5,
            Comm10 => HPDF_PageSizes::HPDF_PAGE_SIZE_COMM10,
            Eof => HPDF_PageSizes::HPDF_PAGE_SIZE_EOF,
            Executive => HPDF_PageSizes::HPDF_PAGE_SIZE_EXECUTIVE,
            Legal => HPDF_PageSizes::HPDF_PAGE_SIZE_LEGAL,
            Letter => HPDF_PageSizes::HPDF_PAGE_SIZE_LETTER,
            Us46 => HPDF_PageSizes::HPDF_PAGE_SIZE_US4x6,
            Us48 => HPDF_PageSizes::HPDF_PAGE_SIZE_US4x8,
            Us57 => HPDF_PageSizes::HPDF_PAGE_SIZE_US5x7,
        }
    }

    fn get_page_direction(direction: PageDirection) -> HPDF_PageDirection {
        use crate::pages::PageDirection::*;
        match direction {
            Landscape => HPDF_PageDirection::HPDF_PAGE_LANDSCAPE,
            Portrait => HPDF_PageDirection::HPDF_PAGE_PORTRAIT,
        }
    }
}

/// Functions of drawing PDF
/// PdfComponent is now implemented for 
/// - RRect
/// - Table
impl HaruPDF {
    pub fn add_component(&mut self, component: impl PdfComponent) {
        component.draw(self);
    }
}
