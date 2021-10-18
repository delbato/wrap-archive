use std::{fs::File, sync::Arc};
use std::ffi::CStr;

use libc::{
    c_char
};

use crate::{
    Archive
};

pub struct WrapReader();

#[no_mangle]
pub extern fn wrap_hello_world() {
    println!("Hello wrap!");
}

#[no_mangle]
pub extern fn wrap_reader_new(file_path: *const c_char) -> *mut WrapReader {
    let file_path = unsafe {
        let c_str = CStr::from_ptr(file_path);
        c_str.to_str().unwrap()
    };
    let mut file = File::open(file_path).unwrap();
    let mut archive = Archive::new(file).unwrap();
    let archive_box = Box::new(archive);
    let archive_ptr = Box::into_raw(archive_box);
    archive_ptr as *mut WrapReader
}

#[no_mangle]
pub extern fn wrap_reader_destroy(reader: *mut WrapReader) {
    let archive_ptr = reader as *mut Archive<File>;
    let archive_box = unsafe { Box::from_raw(archive_ptr) };
    drop(archive_box);
}