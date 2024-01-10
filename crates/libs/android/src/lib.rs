use std::ffi::c_void;

use base64::Engine;
use jni::{JavaVM, JNIEnv, NativeMethod};
use jni::objects::{GlobalRef, JClass, JObject, JStaticMethodID, JString};
use jni::signature::JavaType;
use jni::sys::{jint, jlong, jobject};
use once_cell::sync::OnceCell;

pub mod url;
pub mod text_decoder;
pub mod text_encoder;
mod buffer;
mod crypto;

const ANDROID_O: i32 = 26;


pub static JVM: OnceCell<JavaVM> = OnceCell::new();

pub static API_LEVEL: OnceCell<i32> = OnceCell::new();

struct MethodInfo {
    id: JStaticMethodID,
    clazz: GlobalRef,
}

impl MethodInfo {
    pub fn clazz(&self) -> JClass {
        unsafe { JClass::from_raw(self.clazz.as_raw()) }
    }
}


pub static GC_STATIC_WATCH_OBJECT_METHOD: OnceCell<MethodInfo> = OnceCell::new();

pub(crate) const BUILD_VERSION_CLASS: &str = "android/os/Build$VERSION";
pub(crate) const GC_CLASS: &str = "org/nativescript/wcg/core/GC";

pub(crate) const BASE64_CLASS: &str = "org/nativescript/wcg/core/Base64";

#[no_mangle]
pub extern "system" fn JNI_OnLoad(vm: JavaVM, _reserved: *const c_void) -> jint {
    //   android_logger::init_once(Config::default());

    if let Ok(mut env) = vm.get_env() {
        GC_STATIC_WATCH_OBJECT_METHOD.get_or_init(|| {
            let clazz = env.find_class(GC_CLASS).unwrap();
            let id = env.get_static_method_id(
                clazz, "watchObject", "(JLjava/nio/ByteBuffer;)V",
            ).unwrap();
            let clazz = env.find_class(GC_CLASS).unwrap();
            let clazz_ref = env.new_global_ref(clazz).unwrap();
            MethodInfo {
                id,
                clazz: clazz_ref,
            }
        });
        API_LEVEL.get_or_init(|| {
            let clazz = env.find_class(BUILD_VERSION_CLASS).unwrap();

            let sdk_int_id = env.get_static_field_id(&clazz, "SDK_INT", "I").unwrap();

            let sdk_int = env.get_static_field_unchecked(
                clazz,
                sdk_int_id,
                JavaType::Primitive(jni::signature::Primitive::Int),
            );

            let ret = sdk_int.unwrap().i().unwrap();

            register_native_methods(&mut env, ret);
            url::register_native_methods(&mut env, ret);
            text_encoder::register_native_methods(&mut env, ret);
            text_decoder::register_native_methods(&mut env, ret);
            crypto::register_native_methods(&mut env, ret);

            ret
        });
    }


    JVM.get_or_init(|| vm);


    jni::sys::JNI_VERSION_1_6
}


pub fn register_native_methods(env: &mut JNIEnv, api_level: i32) {
    let base_64_class = env.find_class(BASE64_CLASS).unwrap();

    let base_64_method_names = [
        "nativeAtob",
        "nativeBtoa",
    ];

    let base_64_signatures = if api_level >= crate::ANDROID_O {
        [
            "(Ljava/lang/String;)Ljava/lang/String;",
            "(Ljava/lang/String;)Ljava/lang/String;"
        ]
    } else {
        [
            "!(Ljava/lang/String;)Ljava/lang/String;",
            "!(Ljava/lang/String;)Ljava/lang/String;"
        ]
    };


    let base_64_methods = [
        atob as *mut c_void,
        btoa as *mut c_void
    ];

    let base_64_native_methods: Vec<NativeMethod> =
        itertools::izip!(base_64_method_names, base_64_signatures, base_64_methods)
            .map(|(name, signature, method)| NativeMethod {
                name: name.into(),
                sig: signature.into(),
                fn_ptr: method,
            })
            .collect();

    let _ = env.register_native_methods(&base_64_class, base_64_native_methods.as_slice());
}


#[no_mangle]
pub extern "system" fn Java_org_nativescript_wcg_core_GC_disposeByteBufMut(
    _env: JNIEnv,
    _: JClass,
    buf: jlong,
) {
    let buf: *mut buffer::ByteBufMut = buf as _;
    if !buf.is_null() {
        let _ = unsafe { Box::from_raw(buf) };
    }
}


#[no_mangle]
pub extern "system" fn atob(mut env: JNIEnv, _: JClass, value: JString) -> jobject {
    if value.is_null() {
        return JObject::null().into_raw();
    }
    if let Ok(value) = env.get_string(&value) {
        let value = value.to_string_lossy();
        return match base64::engine::general_purpose::STANDARD.decode(value.as_bytes()) {
            Ok(value) => {
                String::from_utf8(value).map(|value| {
                    env.new_string(value).unwrap().into_raw()
                }).unwrap_or_else(|_| JObject::null().into_raw())
            }
            Err(_) => JObject::null().into_raw(),
        };
    }

    return JObject::null().into_raw();
}

#[no_mangle]
pub extern "system" fn btoa(mut env: JNIEnv, _: JClass, value: JString) -> jobject {
    if value.is_null() {
        return JObject::null().into_raw();
    }

    if let Ok(value) = env.get_string(&value) {
        let value = value.to_string_lossy();
        let decoded = base64::engine::general_purpose::STANDARD.encode(value.as_bytes());
        return env.new_string(decoded).unwrap().into_raw();
    }


    JObject::null().into_raw()
}