use std::ffi::{c_char, CString};

use rand::Rng;

#[no_mangle]
pub extern "C" fn wcg_core_crypto_random_uuid() -> *const c_char {
    let uuid = uuid::Uuid::new_v4();
    CString::new(uuid.to_string()).unwrap().into_raw()
}


#[no_mangle]
pub unsafe extern "C" fn wcg_core_crypto_get_random_values(bytes: *mut u8, length: usize) {
    if bytes.is_null() || length == 0 {
        return;
    }

    let bytes = std::slice::from_raw_parts_mut(bytes, length);

    let mut rng = rand::thread_rng();
    rng.fill(bytes);
}
