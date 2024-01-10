use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use base64::Engine;

use crate::ccow::CCow;

pub mod url;
pub mod ccow;
pub mod text_decoder;
pub mod text_encoder;
pub mod buffers;
mod crypto;


#[no_mangle]
pub extern "C" fn wcg_core_atob(value: *const c_char) -> *const c_char {
    if value.is_null() {
        return std::ptr::null_mut();
    }
    let value = unsafe { CStr::from_ptr(value) };
    let value = value.to_string_lossy();
    match base64::engine::general_purpose::STANDARD.decode(value.as_bytes()) {
        Ok(value) => {
            String::from_utf8(value).map(|value| CString::new(value).unwrap().into_raw() as *const c_char)
                .unwrap_or_else(|_| std::ptr::null())
        }
        Err(_) => std::ptr::null(),
    }
}


#[no_mangle]
pub extern "C" fn wcg_core_btoa(value: *const c_char) -> *const c_char {
    if value.is_null() {
        return std::ptr::null_mut();
    }
    let value = unsafe { CStr::from_ptr(value) };

    let value = value.to_string_lossy();

    let ret = base64::engine::general_purpose::STANDARD.encode(value.as_bytes());

    CString::new(ret).unwrap().into_raw()
}


#[no_mangle]
pub extern "C" fn wcg_core_string_destroy(value: *mut c_char) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { CString::from_raw(value) };
}