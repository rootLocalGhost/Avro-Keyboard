use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use avro_core::parser::EnglishToBangla;

static mut PARSER: *mut EnglishToBangla = std::ptr::null_mut();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn avro_convert(input_str: *const c_char) -> *mut c_char {
    if input_str.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        if PARSER.is_null() {
            let mut parser = EnglishToBangla::new();
            parser.auto_correct = false;
            PARSER = Box::into_raw(Box::new(parser));
        }
    }

    let c_str = unsafe { CStr::from_ptr(input_str) };
    let input_rs = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let parser = unsafe { &*PARSER };
    let output_rs = match parser.convert(input_rs) {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let c_string = match CString::new(output_rs) {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    c_string.into_raw()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn avro_free_string(str: *mut c_char) {
    if str.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(str);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn avro_init() {
    unsafe {
        if PARSER.is_null() {
            let mut parser = EnglishToBangla::new();
            parser.auto_correct = false;
            PARSER = Box::into_raw(Box::new(parser));
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn avro_cleanup() {
    unsafe {
        if !PARSER.is_null() {
            let _ = Box::from_raw(PARSER);
            PARSER = std::ptr::null_mut();
        }
    }
}
