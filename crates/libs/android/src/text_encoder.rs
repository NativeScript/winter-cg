use std::ffi::c_void;

use encoding_rs::UTF_8;
use jni::{JNIEnv, NativeMethod};
use jni::objects::{JClass, JObject, JString, JValue};
use jni::signature::Primitive::Void;
use jni::signature::ReturnType;
use jni::sys::{jlong, jobject, jstring, jvalue};

use crate::buffer::ByteBufMut;

#[derive(Clone)]
pub struct TextEncoder {
    inner: &'static encoding_rs::Encoding,
}

impl TextEncoder {
    pub fn new(encoding: Option<&str>) -> Self {
        let encoding = encoding.unwrap_or("utf-8");
        let encoder = encoding_rs::Encoding::for_label(encoding.as_bytes())
            .unwrap_or(UTF_8.output_encoding());
        Self { inner: encoder }
    }

    pub fn encode(&self, text: &str) -> Vec<u8> {
        let result = self.inner.encode(text);
        Vec::from(result.0)
    }

    pub fn encoding(&self) -> &str {
        self.inner.name()
    }
}


pub(crate) const TEXT_ENCODER_CLASS: &str = "org/nativescript/wcg/core/TextEncoder";

pub fn register_native_methods(env: &mut JNIEnv, api_level: i32) {
    let text_encoder_class = env.find_class(TEXT_ENCODER_CLASS).unwrap();

    let text_encoder_method_names = [
        "nativeCreate",
        "nativeEncode",
        "nativeEncodeBuffer",
        "nativeDestroy",
        "nativeGetEncoding"
    ];

    let text_encoder_signatures = if api_level >= crate::ANDROID_O {
        [
            "(Ljava/lang/String;)J",
            "(JLjava/lang/String;)[B",
            "(JLjava/lang/String;)Ljava/nio/ByteBuffer;",
            "(J)V",
            "(J)Ljava/lang/String;"
        ]
    } else {
        [
            "!(Ljava/lang/String;)J",
            "!(JLjava/lang/String;)[B",
            "!(JLjava/lang/String;)Ljava/nio/ByteBuffer;",
            "!(J)V",
            "!(J)Ljava/lang/String;"
        ]
    };


    let text_encoder_methods = if api_level >= crate::ANDROID_O {
        [
            text_encoder_create as *mut c_void,
            text_encoder_encode as *mut c_void,
            text_encoder_encode_buffer as *mut c_void,
            text_encoder_destroy as *mut c_void,
            text_encoder_get_encoding as *mut c_void
        ]
    } else {
        [
            text_encoder_create as *mut c_void,
            text_encoder_encode as *mut c_void,
            text_encoder_encode_buffer as *mut c_void,
            text_encoder_destroy_normal as *mut c_void,
            text_encoder_get_encoding as *mut c_void
        ]
    };

    let text_encoder_native_methods: Vec<NativeMethod> =
        itertools::izip!(text_encoder_method_names, text_encoder_signatures, text_encoder_methods)
            .map(|(name, signature, method)| NativeMethod {
                name: name.into(),
                sig: signature.into(),
                fn_ptr: method,
            })
            .collect();

    let _ = env.register_native_methods(&text_encoder_class, text_encoder_native_methods.as_slice());
}

pub fn watch_item(env: &mut JNIEnv, id: jlong, buffer: jvalue) {
    if let Some(method_id) = crate::GC_STATIC_WATCH_OBJECT_METHOD.get() {
        unsafe {
            env.call_static_method_unchecked(
                method_id.clazz(), method_id.id, ReturnType::Primitive(Void), &[JValue::Long(id).as_jni(), buffer],
            ).unwrap();
        }
    }
}


#[no_mangle]
pub extern "system" fn text_encoder_create(mut env: JNIEnv, _: JClass, encoding: JString) -> jlong {
    if encoding.is_null() {
        return 0;
    }

    match env.get_string(&encoding) {
        Ok(encoding) => {
            let encoding = encoding.to_string_lossy();
            Box::into_raw(Box::new(
                TextEncoder::new(Some(encoding.as_ref()))
            )) as jlong
        }
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "system" fn text_encoder_encode(
    mut env: JNIEnv,
    _: JClass,
    encoder: jlong,
    text: JString,
) -> jobject {
    if encoder == 0 || text.is_null() {
        return env.new_byte_array(0).unwrap().into_raw();
    }
    let encoder = encoder as *const TextEncoder;
    let encoder = unsafe { &*encoder };


    match env.get_string(&text) {
        Ok(text) => {
            let text = text.to_string_lossy();
            let encoded = encoder.encode(text.as_ref());
            env.byte_array_from_slice(encoded.as_slice()).unwrap_or(env.new_byte_array(0).unwrap())
        }
        Err(_) => env.new_byte_array(0).unwrap(),
    }.into_raw()
}


#[no_mangle]
pub extern "system" fn text_encoder_encode_buffer(
    mut env: JNIEnv,
    _: JClass,
    encoder: jlong,
    text: JString,
) -> jobject {
    if encoder == 0 {
        return JObject::null().into_raw();
    }
    unsafe {
        if let Ok(text) = env.get_string(&text) {
            let text = text.to_string_lossy();
            let encoder: *mut TextEncoder = encoder as _;
            let encoder = &mut *encoder;
            let array = encoder.encode(text.as_ref());
            let buffer = ByteBufMut::from(array);
            let db = env.new_direct_byte_buffer(buffer.data, buffer.len).unwrap();
            let buf = Box::into_raw(Box::new(buffer));
            let value: JValue = (&db).into();
            watch_item(&mut env, buf as jlong, value.as_jni());
            return db.into_raw();
        }
    }
    JObject::null().into_raw()
}

#[no_mangle]
pub extern "system" fn text_encoder_get_encoding(
    env: JNIEnv,
    _: JClass,
    encoder: jlong,
) -> jstring {
    if encoder == 0 {
        return env.new_string("").unwrap().into_raw();
    }

    let encoder = encoder as *const TextEncoder;
    let encoder = unsafe { &*encoder };

    let ret = encoder.encoding().to_lowercase();
    env.new_string(ret).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn text_encoder_destroy(encoder: jlong) {
    if encoder == 0 {
        return;
    }

    let encoder = encoder as *mut TextEncoder;

    let _ = unsafe { Box::from_raw(encoder) };
}


#[no_mangle]
pub extern "system" fn text_encoder_destroy_normal(_: JNIEnv, _: JClass, encoder: jlong) {
    if encoder == 0 {
        return;
    }

    let encoder = encoder as *mut TextEncoder;

    let _ = unsafe { Box::from_raw(encoder) };
}