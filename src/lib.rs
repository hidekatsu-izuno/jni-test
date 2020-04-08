#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

use std::ffi::{ CStr, CString };

use jni::JNIEnv;
use jni::objects::{ JClass, JObject, JValue };
use jni::sys::{ jint, jlong };

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[no_mangle]
pub extern "system" fn Java_ZipFile_open(
    env: JNIEnv,
    _class: JClass,
    j_filename: JObject
) -> jlong {
    let c_filename = CString::new(
            env.convert_byte_array(*j_filename).unwrap()
        ).unwrap();
    let mut c_errorp = 0;

    let c_file = unsafe { zip_open(c_filename.as_ptr(), ZIP_RDONLY as i32, &mut c_errorp) } as jlong;
    if c_errorp != 0 {
        env.throw_new("java/io/IOException", match c_errorp as u32 {
            ZIP_ER_EXISTS => "Exists",
            ZIP_ER_INCONS => "Incons",
            ZIP_ER_INVAL => "Inval",
            ZIP_ER_MEMORY => "Memory",
            ZIP_ER_NOENT => "Noent",
            ZIP_ER_NOZIP => "Nozip",
            ZIP_ER_OPEN => "Open",
            ZIP_ER_READ => "Read",
            ZIP_ER_SEEK => "Seek",
            _ => "I/O Error"
        }).unwrap();
        return 0;
    }

    return c_file;
}

#[no_mangle]
pub extern "system" fn Java_ZipFile_entries(
    env: JNIEnv,
    _class: JClass,
    c_file: jlong,
    j_func: JObject
) {
    let len = unsafe { zip_get_num_entries(c_file as *mut zip, 0) };
    if len == -1 {
        return;
    }

    for i in 0..len {
        let c_name = unsafe { zip_get_name(c_file as *mut zip, i as u64, 0) };
        let name = if !c_name.is_null() {
            unsafe { CStr::from_ptr(c_name) }.to_str().unwrap()
        } else {
            ""
        };
        let j_name = env.new_string(name).unwrap();

        let j_args = [
            JValue::Int(i as jint),
            JValue::Object(*j_name)
        ];
        env.call_method(j_func, "accept", "(ILjava/lang/String;)V", &j_args).unwrap();
    }
}

#[no_mangle]
pub extern "system" fn Java_ZipFile_close(
    env: JNIEnv,
    _class: JClass,
    c_file: jlong
) {
    let c_errorp = unsafe { zip_close(c_file as *mut zip) };
    if c_errorp != 0 {
        env.throw_new("java/io/IOException", match c_errorp as u32 {
            _ => "I/O Error"
        }).unwrap();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // new_zip_file("./tests/LICENSE.zip");
    }
}
