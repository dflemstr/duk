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

#[cfg(debug)]
#[no_mangle]
extern "C" fn __duktape_sys_debug_write(
    level: libc::c_long,
    file: *const libc::c_char,
    func: *const libc::c_char,
    msg: *const libc::c_char) {
    let file_str = ::std::ffi::CStr::from_ptr(file);
    let func_str = ::std::ffi::CStr::from_ptr(func);
    let msg = ::std::ffi::CStr::from_ptr(msg);

    let target = format!("{}.{}", file.to_str().unwrap(), func.to_str().unwrap());
    let log_level = if level == DUK_LOG_TRACE {
        log::LogLevel::Trace
    } else if level == DUK_LOG_DEBUG {
        log::LogLevel::Debug
    } else if level == DUK_LOG_INFO {
        log::LogLevel::Info
    } else if level == DUK_LOG_WARN {
        log::LogLevel::Warn
    } else {
        log::LogLevel::Error
    };

    log!(target, log_level, msg.to_str().unwrap())
}
