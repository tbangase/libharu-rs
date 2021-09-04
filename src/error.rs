use libharu_sys::*;
use libc::c_void;
//use std::error::Error;

pub struct PdfError {
    
}

// impl Display for PdfError {
    
// }

impl PdfError {
    pub extern "C" fn error_handler(error_no: HPDF_STATUS, detail_no: HPDF_STATUS, user_data: *mut c_void) {
        println!("Error Hexacode: {0:x}  Detail No: {1:?}", error_no, detail_no);
        println!("User Data Address: {:?}", user_data);
    }
}