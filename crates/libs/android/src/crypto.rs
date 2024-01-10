use std::ffi::c_void;

use jni::{JNIEnv, NativeMethod};
use jni::objects::{JByteArray, JByteBuffer, JClass, JIntArray, JLongArray, JShortArray, ReleaseMode};
use jni::sys::{jobject, jstring};
use rand::Rng;

pub(crate) const CRYPTO_CLASS: &str = "org/nativescript/wcg/core/Crypto";

pub fn register_native_methods(env: &mut JNIEnv, api_level: i32) {
    let crypto_class = env.find_class(CRYPTO_CLASS).unwrap();

    let crypto_method_names = [
        "nativeRandomUUID",
        "nativeGetRandomValues",
        "nativeGetRandomValuesBuffer",
        "nativeGetRandomValuesShort",
        "nativeGetRandomValuesInt",
        "nativeGetRandomValuesLong",
        "nativeGetRandomValuesBufferShort",
        "nativeGetRandomValuesBufferInt",
        "nativeGetRandomValuesBufferLong",
    ];

    let crypto_signatures = if api_level >= crate::ANDROID_O {
        [
            "()Ljava/lang/String;",
            "([B)V",
            "(Ljava/nio/ByteBuffer;)V",
            "([S)V",
            "([I)V",
            "([J)V",
            "(Ljava/lang/Object;)V",
            "(Ljava/lang/Object;)V",
            "(Ljava/lang/Object;)V",
        ]
    } else {
        [
            "!()Ljava/lang/String;",
            "(![B)V",
            "!(Ljava/nio/ByteBuffer;)V",
            "!([S)V",
            "!([I)V",
            "!([J)V",
            "!(Ljava/lang/Object;)V",
            "!(Ljava/lang/Object;)V",
            "!(Ljava/lang/Object;)V",
        ]
    };


    let crypto_methods = [
        random_uuid as *mut c_void,
        get_random_values as *mut c_void,
        get_random_values_buffer as *mut c_void,
        get_random_values_short as *mut c_void,
        get_random_values_int as *mut c_void,
        get_random_values_long as *mut c_void,
        get_random_values_buffer_short as *mut c_void,
        get_random_values_buffer_int as *mut c_void,
        get_random_values_buffer_long as *mut c_void,
    ];

    let crypto_native_methods: Vec<NativeMethod> =
        itertools::izip!(crypto_method_names, crypto_signatures, crypto_methods)
            .map(|(name, signature, method)| NativeMethod {
                name: name.into(),
                sig: signature.into(),
                fn_ptr: method,
            })
            .collect();

    let _ = env.register_native_methods(&crypto_class, crypto_native_methods.as_slice());
}


#[no_mangle]
pub extern "system" fn random_uuid(mut env: JNIEnv,
                                   _: JClass) -> jstring {
    let uuid = uuid::Uuid::new_v4();
    env.new_string(uuid.to_string()).unwrap().into_raw()
}


#[no_mangle]
pub unsafe extern "system" fn get_random_values(mut env: JNIEnv,
                                                _: JClass, bytes: JByteArray) {
    if let Ok(mut bytes) = env.get_array_elements(&bytes, ReleaseMode::CopyBack) {
        if bytes.len() > 65536 {
            let _ = env.throw_new("java/lang/Exception", &format!("The ArrayBufferView's byte length ({}) exceeds the number of bytes of entropy available via this API (65536)", bytes.len()));
            return;
        }
        let bytes = std::slice::from_raw_parts_mut(bytes.as_mut_ptr() as *mut u8, bytes.len());
        let mut rng = rand::thread_rng();
        rng.fill(bytes);
    }
}


#[no_mangle]
pub unsafe extern "system" fn get_random_values_short(mut env: JNIEnv,
                                                      _: JClass, bytes: JShortArray) {
    if let Ok(mut bytes) = env.get_array_elements(&bytes, ReleaseMode::CopyBack) {
        if bytes.len() * 2 > 65536 {
            let _ = env.throw_new("java/lang/Exception", &format!("The ArrayBufferView's byte length ({}) exceeds the number of bytes of entropy available via this API (65536)", bytes.len() * 2));
            return;
        }
        let bytes = std::slice::from_raw_parts_mut(bytes.as_mut_ptr() as *mut u8, bytes.len() * 2);
        let mut rng = rand::thread_rng();
        rng.fill(bytes);
    }
}


#[no_mangle]
pub unsafe extern "system" fn get_random_values_int(mut env: JNIEnv,
                                                    _: JClass, bytes: JIntArray) {
    if let Ok(mut bytes) = env.get_array_elements(&bytes, ReleaseMode::CopyBack) {
        if bytes.len() * 4 > 65536 {
            let _ = env.throw_new("java/lang/Exception", &format!("The ArrayBufferView's byte length ({}) exceeds the number of bytes of entropy available via this API (65536)", bytes.len() * 4));
            return;
        }
        let bytes = std::slice::from_raw_parts_mut(bytes.as_mut_ptr() as *mut u8, bytes.len() * 4);
        let mut rng = rand::thread_rng();
        rng.fill(bytes);
    }
}

#[no_mangle]
pub unsafe extern "system" fn get_random_values_long(mut env: JNIEnv,
                                                     _: JClass, bytes: JLongArray) {
    if let Ok(mut bytes) = env.get_array_elements(&bytes, ReleaseMode::CopyBack) {
        if bytes.len() * 8 > 65536 {
            let _ = env.throw_new("java/lang/Exception", &format!("The ArrayBufferView's byte length ({}) exceeds the number of bytes of entropy available via this API (65536)", bytes.len() * 8));
            return;
        }
        let bytes = std::slice::from_raw_parts_mut(bytes.as_mut_ptr() as *mut u8, bytes.len() * 8);
        let mut rng = rand::thread_rng();
        rng.fill(bytes);
    }
}


#[no_mangle]
pub extern "system" fn get_random_values_buffer(mut env: JNIEnv,
                                                _: JClass, buffer: JByteBuffer) {

    if let (Ok(addr), Ok(len)) = ((env.get_direct_buffer_address(&buffer), env.get_direct_buffer_capacity(&buffer))) {
        if len > 65536 {
            let _ = env.throw_new("java/lang/Exception", &format!("The ArrayBufferView's byte length ({}) exceeds the number of bytes of entropy available via this API (65536)", len));
            return;
        }
        let bytes = unsafe { std::slice::from_raw_parts_mut(addr, len) };
        let mut rng = rand::thread_rng();
        rng.fill(bytes);
    }
}


#[no_mangle]
pub extern "system" fn get_random_values_buffer_short(mut env: JNIEnv,
                                                _: JClass, buffer: jobject) {
    let buffer = unsafe { JByteBuffer::from_raw(buffer) };
    if let (Ok(addr), Ok(len)) = ((env.get_direct_buffer_address(&buffer), env.get_direct_buffer_capacity(&buffer))) {
        let len = len * 2;
        if len > 65536 {
            let _ = env.throw_new("java/lang/Exception", &format!("The ArrayBufferView's byte length ({}) exceeds the number of bytes of entropy available via this API (65536)", len));
            return;
        }
        let bytes = unsafe { std::slice::from_raw_parts_mut(addr, len) };
        let mut rng = rand::thread_rng();
        rng.fill(bytes);
    }
}

#[no_mangle]
pub extern "system" fn get_random_values_buffer_int(mut env: JNIEnv,
                                                      _: JClass, buffer: jobject) {
    let buffer = unsafe { JByteBuffer::from_raw(buffer) };
    if let (Ok(addr), Ok(len)) = ((env.get_direct_buffer_address(&buffer), env.get_direct_buffer_capacity(&buffer))) {
        let len = len * 4;
        if len > 65536 {
            let _ = env.throw_new("java/lang/Exception", &format!("The ArrayBufferView's byte length ({}) exceeds the number of bytes of entropy available via this API (65536)", len));
            return;
        }
        let bytes = unsafe { std::slice::from_raw_parts_mut(addr, len) };
        let mut rng = rand::thread_rng();
        rng.fill(bytes);
    }
}

#[no_mangle]
pub extern "system" fn get_random_values_buffer_long(mut env: JNIEnv,
                                                      _: JClass, buffer: jobject) {
    let buffer = unsafe { JByteBuffer::from_raw(buffer) };
    if let (Ok(addr), Ok(len)) = ((env.get_direct_buffer_address(&buffer), env.get_direct_buffer_capacity(&buffer))) {
        let len = len * 8;
        if len > 65536 {
            let _ = env.throw_new("java/lang/Exception", &format!("The ArrayBufferView's byte length ({}) exceeds the number of bytes of entropy available via this API (65536)", len));
            return;
        }
        let bytes = unsafe { std::slice::from_raw_parts_mut(addr, len) };
        let mut rng = rand::thread_rng();
        rng.fill(bytes);
    }
}