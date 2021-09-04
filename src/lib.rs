use libharu_sys::*;
use libc::c_void;
use std::ffi::CString;

pub mod page;
pub mod error;

pub use error::PdfError;
use page::{PageSize, PageDirection};

pub struct Hpdf {
    pdf: HPDF_Doc,
    current_page: Option<HPDF_Page>,
    page_num: i32,
    font: *mut c_void,
}

impl Hpdf {
    pub fn new_doc(file_name: &str) -> Self {
        unsafe {
            let null_pointer: *mut c_void = std::ptr::null_mut();
            let pdf_error = PdfError::error_handler;
            let pdf: HPDF_Doc = HPDF_New(pdf_error, null_pointer.clone());
            return Self {
                pdf: pdf,
                current_page: None,
                page_num: 0,
                font: null_pointer
            }    
        }
        
    }

    pub fn save_doc() -> return 

    pub fn new_page(size: PageSize, direction: PageDirection) {
        // set current_page and increment page_num
    }

    pub fn set_font(font_path: String) {
        // set font
    }
}