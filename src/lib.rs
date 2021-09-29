use libharu_sys::*;
use libc::c_void;
use std::ffi::CString;
use dispose::{Dispose, Disposable};

pub mod pages;
pub mod error;
pub mod constants;
pub mod alignments;

pub use error::PdfError;
use pages::{PageSize, PageDirection};
use constants::*;

pub struct Hpdf {
    doc: HPDF_Doc,
    current_page: Option<HPDF_Page>,
    font: *mut c_void,
}

impl Dispose for Hpdf {
    fn dispose(self) {
        unsafe {
            HPDF_Free(self.doc);
        }
    }
}

// 
impl Hpdf {
    pub fn new_doc() -> Disposable<Self> {
        unsafe {
            let null_pointer: *mut c_void = std::ptr::null_mut();
            let pdf_error = PdfError::error_handler;
            let doc: HPDF_Doc = HPDF_New(pdf_error, null_pointer.clone());
            return Self {
                doc: doc,
                current_page: None,
                font: null_pointer
            }.into()
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
                Some(val)=> {
                    HPDF_Page_SetSize(
                        val,
                        page_size,
                        page_direction,
                    );
                },
                None => println!("Internal Error. Adding page failed.")
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
                Some(val)=> {
                    HPDF_Page_SetSize(
                        val,
                        page_size,
                        page_direction,
                    );
                },
                None => println!("Internal Error. Adding page failed.")
            }
        }
    }

    pub fn set_font(&mut self, font_path: String) {
        // set font
        unsafe {
            HPDF_UseUTFEncodings(self.doc);
            let font_path = CString::new(font_path)
                .expect(CSTRING_ERROR_MSG);

            let font_name = HPDF_LoadTTFontFromFile(self.doc, font_path.as_ptr(), HPDF_TRUE);
            let encoding = CString::new("UTF-8").expect(CSTRING_ERROR_MSG);

            HPDF_SetCurrentEncoder(self.doc, encoding.as_ptr());

            self.font = HPDF_GetFont(self.doc, font_name, encoding.as_ptr());
        }
    }
}

impl Hpdf {
    fn get_page_size(size: PageSize) -> HPDF_PageSizes {
        use pages::PageSize::*;
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
        use pages::PageDirection::*;
        match direction {
            Landscape => HPDF_PageDirection::HPDF_PAGE_LANDSCAPE,
            Portrait => HPDF_PageDirection::HPDF_PAGE_PORTRAIT,
        }
    } 
}