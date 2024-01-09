use std::ffi::c_void;

use ada_url::Url;
use jni::{JNIEnv, NativeMethod};
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jboolean, jlong, JNI_FALSE, JNI_TRUE, jstring};

pub struct URL(Url);

pub(crate) const URL_CLASS: &str = "org/nativescript/wcg/core/URL";

pub fn register_native_methods(env: &mut JNIEnv, api_level: i32) {
    let url_class = env.find_class(URL_CLASS).unwrap();

    let url_method_names = [
        "nativeCanParse",
        "nativeCreate",
        "nativeDestroy",
        "nativeToString",
        "nativeSetHash",
        "nativeGetHash",
        "nativeSetHost",
        "nativeGetHost",
        "nativeSetHostName",
        "nativeGetHostName",
        "nativeSetHref",
        "nativeGetHref",
        "nativeGetOrigin",
        "nativeSetPassword",
        "nativeGetPassword",
        "nativeSetPathName",
        "nativeGetPathName",
        "nativeSetPort",
        "nativeGetPort",
        "nativeSetProtocol",
        "nativeGetProtocol",
        "nativeSetSearch",
        "nativeGetSearch",
        "nativeSetUserName",
        "nativeGetUserName"
    ];

    let url_signatures = if api_level >= crate::ANDROID_O {
        [
            "(Ljava/lang/String;Ljava/lang/String;)Z",
            "(Ljava/lang/String;Ljava/lang/String;)J",
            "(J)V",
            "(J)Ljava/lang/String;",
            "(JLjava/lang/String;)V",
            "(J)Ljava/lang/String;",
            "(JLjava/lang/String;)V",
            "(J)Ljava/lang/String;",
            "(JLjava/lang/String;)V",
            "(J)Ljava/lang/String;",
            "(JLjava/lang/String;)V",
            "(J)Ljava/lang/String;",
            "(J)Ljava/lang/String;",
            "(JLjava/lang/String;)V",
            "(J)Ljava/lang/String;",
            "(JLjava/lang/String;)V",
            "(J)Ljava/lang/String;",
            "(JLjava/lang/String;)V",
            "(J)Ljava/lang/String;",
            "(JLjava/lang/String;)V",
            "(J)Ljava/lang/String;",
            "(JLjava/lang/String;)V",
            "(J)Ljava/lang/String;",
            "(JLjava/lang/String;)V",
        ]
    } else {
        [
            "!(Ljava/lang/String;Ljava/lang/String;)Z",
            "!(Ljava/lang/String;Ljava/lang/String;)J",
            "!(J)V",
            "!(J)Ljava/lang/String;",
            "!(JLjava/lang/String;)V",
            "!(J)Ljava/lang/String;",
            "!(JLjava/lang/String;)V",
            "!(J)Ljava/lang/String;",
            "!(JLjava/lang/String;)V",
            "!(J)Ljava/lang/String;",
            "!(JLjava/lang/String;)V",
            "!(J)Ljava/lang/String;",
            "!(J)Ljava/lang/String;",
            "!(JLjava/lang/String;)V",
            "!(J)Ljava/lang/String;",
            "!(JLjava/lang/String;)V",
            "!(J)Ljava/lang/String;",
            "!(JLjava/lang/String;)V",
            "!(J)Ljava/lang/String;",
            "!(JLjava/lang/String;)V",
            "!(J)Ljava/lang/String;",
            "!(JLjava/lang/String;)V",
            "!(J)Ljava/lang/String;",
            "!(JLjava/lang/String;)V",
        ]
    };


    let url_methods = if api_level >= crate::ANDROID_O {
        [
            can_parse as *mut c_void,
            create as *mut c_void,
            destroy as *mut c_void,
            to_string as *mut c_void,
            hash as *mut c_void,
            set_hash as *mut c_void,
            host as *mut c_void,
            set_host as *mut c_void,
            host_name as *mut c_void,
            set_host_name as *mut c_void,
            href as *mut c_void,
            set_href as *mut c_void,
            origin as *mut c_void,
            password as *mut c_void,
            set_password as *mut c_void,
            pathname as *mut c_void,
            set_pathname as *mut c_void,
            port as *mut c_void,
            set_port as *mut c_void,
            protocol as *mut c_void,
            set_protocol as *mut c_void,
            search as *mut c_void,
            set_search as *mut c_void,
            username as *mut c_void,
            set_username as *mut c_void
        ]
    } else {
        [
            can_parse as *mut c_void,
            create as *mut c_void,
            destroy_normal as *mut c_void,
            to_string as *mut c_void,
            hash as *mut c_void,
            set_hash as *mut c_void,
            host as *mut c_void,
            set_host as *mut c_void,
            host_name as *mut c_void,
            set_host_name as *mut c_void,
            href as *mut c_void,
            set_href as *mut c_void,
            origin as *mut c_void,
            password as *mut c_void,
            set_password as *mut c_void,
            pathname as *mut c_void,
            set_pathname as *mut c_void,
            port as *mut c_void,
            set_port as *mut c_void,
            protocol as *mut c_void,
            set_protocol as *mut c_void,
            search as *mut c_void,
            set_search as *mut c_void,
            username as *mut c_void,
            set_username as *mut c_void
        ]
    };

    let url_native_methods: Vec<NativeMethod> =
        itertools::izip!(url_method_names, url_signatures, url_methods)
            .map(|(name, signature, method)| NativeMethod {
                name: name.into(),
                sig: signature.into(),
                fn_ptr: method,
            })
            .collect();

    let _ = env.register_native_methods(&url_class, url_native_methods.as_slice());
}

#[no_mangle]
pub extern "system" fn can_parse(mut env: JNIEnv, _: JClass, value: JString, base: JString) -> jboolean {
    let value_null = value.is_null();
    let base_null = base.is_null();
    if value_null && base_null {
        return JNI_FALSE;
    }

    if value_null {
        return match env.get_string(&value) {
            Ok(base) => {
                let base = base.to_string_lossy();
                return if Url::can_parse("", Some(base.as_ref())) { JNI_TRUE } else { JNI_FALSE };
            }
            Err(_) => JNI_FALSE,
        };
    }

    match (env.get_string(&base), env.get_string(&value)) {
        (Err(_), Ok(value)) => {
            let value = value.to_string_lossy();
            if Url::can_parse(value.as_ref(), None) { JNI_TRUE } else { JNI_FALSE }
        }
        (Ok(base), Ok(value)) => {
            let base = base.to_string_lossy();
            let value = value.to_string_lossy();
            if Url::can_parse(value.as_ref(), Some(base.as_ref())) { JNI_TRUE } else { JNI_FALSE }
        }
        (_, _) => JNI_FALSE
    }
}

#[no_mangle]
pub extern "system" fn create(mut env: JNIEnv, _: JClass, value: JString, base: JString) -> jlong {
    match (env.get_string(&base), env.get_string(&value)) {
        (Err(_), Ok(value)) => {
            let value = value.to_string_lossy();
            let url = Url::parse(value.as_ref(), None);

            match url {
                Ok(url) => Box::into_raw(Box::new(URL(url))) as jlong,
                Err(_) => 0
            }
        }
        (Ok(base), Ok(value)) => {
            let base = base.to_string_lossy();
            let value = value.to_string_lossy();
            let url = Url::parse(value.as_ref(), Some(base.as_ref()));

            match url {
                Ok(url) => Box::into_raw(Box::new(URL(url))) as jlong,
                Err(_) => 0
            }
        }
        (_, _) => {
            0
        }
    }
}

#[no_mangle]
pub extern "system" fn destroy(url: jlong) {
    if url == 0 {
        let _ = unsafe { Box::from_raw(url as *mut URL) };
    }
}

#[no_mangle]
pub extern "system" fn destroy_normal(_env: JNIEnv, _: JClass, url: jlong) {
    if url == 0 {
        let _ = unsafe { Box::from_raw(url as *mut URL) };
    }
}

#[no_mangle]
pub extern "system" fn to_string(env: JNIEnv, _: JClass, url: jlong) -> jstring {
    if url == 0 {
        return JObject::null().into_raw();
    }

    let url = url as *mut URL;
    let url = unsafe { &mut *url };
    let url = &mut url.0;

    env.new_string(url.to_string()).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn hash(env: JNIEnv, _: JClass, url: jlong) -> jstring {
    if url == 0 {
        return JObject::null().into_raw();
    }

    let url = url as *mut URL;
    let url = unsafe { &mut *url };
    let url = &mut url.0;

    env.new_string(url.hash()).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn set_hash(mut env: JNIEnv, _: JClass, url: jlong, hash: JString) {
    if url == 0 {
        return;
    }

    let url = url as *mut URL;
    let url = unsafe { &mut *url };
    let url = &mut url.0;

    if hash.is_null() {
        url.set_hash(None);
    } else {
        if let Ok(hash) = env.get_string(&hash) {
            let hash = hash.to_string_lossy();
            url.set_hash(Some(hash.as_ref()));
        }
    }
}

#[no_mangle]
pub extern "system" fn host(env: JNIEnv, _: JClass, url: jlong) -> jstring {
    if url == 0 {
        return JObject::null().into_raw();
    }

    let url = url as *mut URL;
    let url = unsafe { &mut *url };
    let url = &mut url.0;

    env.new_string(url.host()).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn set_host(mut env: JNIEnv, _: JClass, url: jlong, host: JString) {
    if url == 0 {
        return;
    }

    let url = url as *mut URL;
    let url = unsafe { &mut *url };
    let url = &mut url.0;

    if host.is_null() {
        url.set_hash(None);
    } else {
        if let Ok(host) = env.get_string(&host) {
            let host = host.to_string_lossy();
            let _ = url.set_host(Some(host.as_ref()));
        }
    }
}

#[no_mangle]
pub extern "system" fn host_name(env: JNIEnv, _: JClass, url: jlong) -> jstring {
    if url == 0 {
        return JObject::null().into_raw();
    }

    let url = url as *mut URL;
    let url = unsafe { &mut *url };
    let url = &mut url.0;

    env.new_string(url.hostname()).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn set_host_name(mut env: JNIEnv, _: JClass, url: jlong, hostname: JString) {
    if url == 0 {
        return;
    }

    let url = url as *mut URL;
    let url = unsafe { &mut *url };
    let url = &mut url.0;

    if hostname.is_null() {
        let _ = url.set_hostname(None);
    } else {
        if let Ok(hostname) = env.get_string(&hostname) {
            let hostname = hostname.to_string_lossy();
            let _ = url.set_hostname(Some(hostname.as_ref()));
        }
    }
}

#[no_mangle]
pub extern "system" fn href(env: JNIEnv, _: JClass, url: jlong) -> jstring {
    if url == 0 {
        return JObject::null().into_raw();
    }

    let url = url as *mut URL;
    let url = unsafe { &mut *url };
    let url = &mut url.0;

    env.new_string(url.href()).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn set_href(mut env: JNIEnv, _: JClass, url: jlong, href: JString) {
    if url == 0 {
        return;
    }

    let url = url as *mut URL;
    let url = unsafe { &mut *url };
    let url = &mut url.0;

    if !href.is_null() {
        if let Ok(href) = env.get_string(&href) {
            let href = href.to_string_lossy();
            let _ = url.set_href(href.as_ref());
        }
    }
}

#[no_mangle]
pub extern "system" fn origin(env: JNIEnv, _: JClass, url: jlong) -> jstring {
    if url == 0 {
        return JObject::null().into_raw();
    }

    let url = url as *mut URL;
    let url = unsafe { &mut *url };
    let url = &mut url.0;

    env.new_string(url.origin()).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn password(env: JNIEnv, _: JClass, url: jlong) -> jstring {
    if url == 0 {
        return JObject::null().into_raw();
    }

    let url = url as *mut URL;
    let url = unsafe { &mut *url };
    let url = &mut url.0;

    env.new_string(url.password()).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn set_password(mut env: JNIEnv, _: JClass, url: jlong, password: JString) {
    if url == 0 {
        return;
    }

    let url = url as *mut URL;
    let url = unsafe { &mut *url };
    let url = &mut url.0;

    if password.is_null() {
        let _ = url.set_password(None);
    } else {
        if let Ok(password) = env.get_string(&password) {
            let password = password.to_string_lossy();
            let _ = url.set_password(Some(password.as_ref()));
        }
    }
}

#[no_mangle]
pub extern "system" fn pathname<'local>(env: JNIEnv<'local>, _: JClass<'local>, url: jlong) -> JString<'local> {
    if url == 0 {
        return env.new_string("").unwrap();
    }

    let url = url as *mut URL;
    let url = unsafe { &mut *url };
    let url = &mut url.0;

    env.new_string(url.pathname()).unwrap()
}

#[no_mangle]
pub extern "system" fn set_pathname(mut env: JNIEnv, _: JClass, url: jlong, pathname: JString) {
    if url == 0 {
        return;
    }

    let url = url as *mut URL;
    let url = unsafe { &mut *url };
    let url = &mut url.0;

    if pathname.is_null() {
        let _ = url.set_pathname(None);
    } else {
        if let Ok(pathname) = env.get_string(&pathname) {
            let pathname = pathname.to_string_lossy();
            let _ = url.set_pathname(Some(pathname.as_ref()));
        }
    }
}

#[no_mangle]
pub extern "system" fn port(env: JNIEnv, _: JClass, url: jlong) -> jstring {
    if url == 0 {
        return JObject::null().into_raw();
    }

    let url = url as *mut URL;
    let url = unsafe { &mut *url };
    let url = &mut url.0;

    env.new_string(url.port()).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn set_port(mut env: JNIEnv, _: JClass, url: jlong, port: JString) {
    if url == 0 {
        return;
    }

    let url = url as *mut URL;
    let url = unsafe { &mut *url };
    let url = &mut url.0;

    if port.is_null() {
        let _ = url.set_port(None);
    } else {
        if let Ok(port) = env.get_string(&port) {
            let port = port.to_string_lossy();
            let _ = url.set_port(Some(port.as_ref()));
        }
    }
}

#[no_mangle]
pub extern "system" fn protocol(env: JNIEnv, _: JClass, url: jlong) -> jstring {
    if url == 0 {
        return JObject::null().into_raw();
    }

    let url = url as *mut URL;
    let url = unsafe { &mut *url };
    let url = &mut url.0;

    env.new_string(url.protocol()).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn set_protocol(mut env: JNIEnv, _: JClass, url: jlong, protocol: JString) {
    if url == 0 {
        return;
    }

    let url = url as *mut URL;
    let url = unsafe { &mut *url };
    let url = &mut url.0;

    if !protocol.is_null() {
        if let Ok(protocol) = env.get_string(&protocol) {
            let protocol = protocol.to_string_lossy();
            let _ = url.set_protocol(protocol.as_ref());
        }
    }
}

#[no_mangle]
pub extern "system" fn search(env: JNIEnv, _: JClass, url: jlong) -> jstring {
    if url == 0 {
        return JObject::null().into_raw();
    }

    let url = url as *mut URL;
    let url = unsafe { &mut *url };
    let url = &mut url.0;

    env.new_string(url.search()).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn set_search(mut env: JNIEnv, _: JClass, url: jlong, search: JString) {
    if url == 0 {
        return;
    }

    let url = url as *mut URL;
    let url = unsafe { &mut *url };
    let url = &mut url.0;

    if search.is_null() {
        let _ = url.set_search(None);
    } else {
        if let Ok(search) = env.get_string(&search) {
            let search = search.to_string_lossy();
            let _ = url.set_search(Some(search.as_ref()));
        }
    }
}

#[no_mangle]
pub extern "system" fn username(env: JNIEnv, _: JClass, url: jlong) -> jstring {
    if url == 0 {
        return JObject::null().into_raw();
    }

    let url = url as *mut URL;
    let url = unsafe { &mut *url };
    let url = &mut url.0;

    env.new_string(url.username()).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn set_username(mut env: JNIEnv, _: JClass, url: jlong, username: JString) {
    if url == 0 {
        return;
    }

    let url = url as *mut URL;
    let url = unsafe { &mut *url };
    let url = &mut url.0;

    if username.is_null() {
        let _ = url.set_username(None);
    } else {
        if let Ok(username) = env.get_string(&username) {
            let username = username.to_string_lossy();
            let _ = url.set_username(Some(username.as_ref()));
        }
    }
}