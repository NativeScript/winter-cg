use std::ffi::{c_void, CStr};

use encoding_rs::UTF_8;
use jni::{JNIEnv, NativeMethod};
use jni::objects::{JByteArray, JByteBuffer, JClass, JObject, JString, ReleaseMode};
use jni::sys::{jlong, jstring};

#[derive(Clone)]
pub struct TextDecoder {
    inner: &'static encoding_rs::Encoding,
}

impl TextDecoder {
    pub fn new(decoding: Option<&str>) -> Self {
        let decoding = decoding.unwrap_or("utf-8");
        let decoder = encoding_rs::Encoding::for_label(decoding.as_bytes())
            .unwrap_or(UTF_8.output_encoding());
        Self { inner: decoder }
    }

    pub fn decode(&self, data: &[u8]) -> String {
        let (res, _) = self.inner.decode_with_bom_removal(data);

        match CStr::from_bytes_with_nul(res.as_bytes()) {
            Ok(res) => res.to_string_lossy().to_string(),
            Err(_) => res.chars().filter(|&c| c != '\0').collect(),
        }
    }


    pub fn encoding(&self) -> &str {
        self.inner.name()
    }
}


pub(crate) const TEXT_DECODER_CLASS: &str = "org/nativescript/wcg/core/TextDecoder";

pub fn register_native_methods(env: &mut JNIEnv, api_level: i32) {
    let text_decoder_class = env.find_class(TEXT_DECODER_CLASS).unwrap();

    let text_decoder_method_names = [
        "nativeCreate",
        "nativeDecode",
        "nativeDecodeBuffer",
        "nativeDestroy",
        "nativeGetEncoding"
    ];

    let text_decoder_signatures = if api_level >= crate::ANDROID_O {
        [
            "(Ljava/lang/String;)J",
            "(J[B)Ljava/lang/String;",
            "(JLjava/nio/ByteBuffer;)Ljava/lang/String;",
            "(J)V",
            "(J)Ljava/lang/String;"
        ]
    } else {
        [
            "!(Ljava/lang/String;)J",
            "!(J[B)Ljava/lang/String;",
            "!(JLjava/nio/ByteBuffer;)Ljava/lang/String;",
            "!(J)V",
            "!(J)Ljava/lang/String;"
        ]
    };


    let text_decoder_methods = if api_level >= crate::ANDROID_O {
        [
            text_decoder_create as *mut c_void,
            text_decoder_decode as *mut c_void,
            text_decoder_decode_buffer as *mut c_void,
            text_decoder_destroy as *mut c_void,
            text_decoder_get_encoding as *mut c_void
        ]
    } else {
        [
            text_decoder_create as *mut c_void,
            text_decoder_decode as *mut c_void,
            text_decoder_decode_buffer as *mut c_void,
            text_decoder_destroy_normal as *mut c_void,
            text_decoder_get_encoding as *mut c_void
        ]
    };

    let text_decoder_native_methods: Vec<NativeMethod> =
        itertools::izip!(text_decoder_method_names, text_decoder_signatures, text_decoder_methods)
            .map(|(name, signature, method)| NativeMethod {
                name: name.into(),
                sig: signature.into(),
                fn_ptr: method,
            })
            .collect();

    let _ = env.register_native_methods(&text_decoder_class, text_decoder_native_methods.as_slice());
}


#[no_mangle]
pub extern "system" fn text_decoder_create(mut env: JNIEnv, _: JClass, decoding: JString) -> jlong {
    if decoding.is_null() {
        return 0;
    }
    match env.get_string(&decoding) {
        Ok(decoding) => {
            let decoding = decoding.to_string_lossy();
            Box::into_raw(Box::new(
                TextDecoder::new(Some(decoding.as_ref()))
            )) as jlong
        }
        Err(_) => 0,
    }
}

#[no_mangle]
pub unsafe extern "system" fn text_decoder_decode(
    mut env: JNIEnv,
    _: JClass,
    decoder: jlong,
    data: JByteArray,
) -> jstring {
    if decoder == 0 || data.is_null() {
        return JObject::null().into_raw();
    }

    let decoder = decoder as *const TextDecoder;
    let decoder = unsafe { &*decoder };
    match env.get_array_elements(&data, ReleaseMode::NoCopyBack) {
        Ok(data) => {
            let data = unsafe { std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len()) };
            let ret = decoder.decode(data);
            env.new_string(ret).unwrap().into_raw()
        }
        Err(_) => {
            JObject::null().into_raw()
        }
    }
}


#[no_mangle]
pub unsafe extern "system" fn text_decoder_decode_buffer(
    mut env: JNIEnv,
    _: JClass,
    decoder: jlong,
    data: JByteBuffer,
) -> jstring {
    if decoder == 0 || data.is_null() {
        return JObject::null().into_raw();
    }

    let decoder = decoder as *const TextDecoder;
    let decoder = unsafe { &*decoder };

    match (env.get_direct_buffer_address(&data), env.get_direct_buffer_capacity(&data)) {
        (Ok(buffer), Ok(len)) => {
            let buffer = unsafe { std::slice::from_raw_parts(buffer, len) };
            let ret = decoder.decode(buffer);
            env.new_string(ret).unwrap().into_raw()
        }
        (_, _) => { JObject::null().into_raw() }
    }
}


#[no_mangle]
pub extern "C" fn text_decoder_destroy(decoder: jlong) {
    if decoder == 0 {
        return;
    }

    let decoder =  decoder as *mut TextDecoder;

    let _ = unsafe { Box::from_raw(decoder) };
}

#[no_mangle]
pub extern "C" fn text_decoder_destroy_normal(_: JNIEnv, _: JClass, decoder: jlong) {
    if decoder == 0 {
        return;
    }

    let decoder = decoder as *mut TextDecoder;

    let _ = unsafe { Box::from_raw(decoder) };
}


#[no_mangle]
pub extern "C" fn text_decoder_get_encoding(
    env: JNIEnv,
    _: JClass,
    decoder: jlong,
) -> jstring {
    if decoder == 0 {
        return JObject::null().into_raw();
    }
    let decoder = decoder as *const TextDecoder;
    let decoder = unsafe { &*decoder };

    let ret = decoder.encoding().to_lowercase();

    env.new_string(ret).unwrap().into_raw()
}