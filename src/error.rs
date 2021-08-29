use libharu_sys::*;
use libc::c_void;
use std::error::Error;

struct PdfError {
    error_handler: |error_no: HPDF_STATUS| {}
}

impl Error for PdfError {}

impl Display for PdfError {
    
}

pub extern "C" fn error_handler(error_no: HPDF_STATUS, detail_no: HPDF_STATUS, user_data: *mut c_void) {
    println!("Error Hexacode: {0:x}  Detail No: {1:?}", error_no, detail_no);
    println!("User Data Address: {:?}", user_data);
}