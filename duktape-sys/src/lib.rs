//! An auto-generated wrapper around the [Duktape][1] library.
//!
//! The API of this wrapper is not stable, and currently exposes
//! transient library APIs.
//!
//! [1]: http://duktape.org/
#![allow(non_camel_case_types, non_snake_case)]

extern crate libc;
#[macro_use]
extern crate log;

mod ffi;

pub use ffi::*;

#[no_mangle]
pub unsafe extern "C" fn __duktape_sys_debug_write(
    level: libc::c_long,
    file: *const libc::c_char,
    func: *const libc::c_char,
    msg: *const libc::c_char) {
    let file_str = ::std::ffi::CStr::from_ptr(file);
    let func_str = ::std::ffi::CStr::from_ptr(func);
    let msg_str = ::std::ffi::CStr::from_ptr(msg);

    let target = format!("{}.{}", file_str.to_str().unwrap(), func_str.to_str().unwrap());
    let log_level = if level == DUK_LOG_TRACE as i64 {
        log::LogLevel::Trace
    } else if level == DUK_LOG_DEBUG as i64 {
        log::LogLevel::Debug
    } else if level == DUK_LOG_INFO as i64 {
        log::LogLevel::Info
    } else if level == DUK_LOG_WARN as i64 {
        log::LogLevel::Warn
    } else {
        log::LogLevel::Error
    };

    log!(target: &target, log_level, "{}", msg_str.to_str().unwrap())
}
