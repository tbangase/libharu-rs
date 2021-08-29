use libharu_sys::*;
use libc::c_void;
use std::ffi:: { CString };

mod error;

pub use error::error_handler;

pub struct Pdf {
    pdf: HPDF_Doc,
    current_page: Option<HPDF_Page>,
    page_num: i32,
    font: *mut c_void,
}

impl Pdf {
    pub fn new_doc(file_name: &str) -> Self {
        let null_pointer: *mut c_void = std::ptr::null_mut();
        let pdf: HPDF_Doc = HPDF_New(error_handler, null_pointer.clone());
        return Self {
            pdf: pdf,
            current_page: None,
            page_num: 0,
            font: null_pointer
        }
    }

    pub fn new_page(size: PageSize, direction: PageDirection) {
        // set current_page and increment page_num
    }

    pub fn set_font(font_path: String) {
        // set font
    }
}