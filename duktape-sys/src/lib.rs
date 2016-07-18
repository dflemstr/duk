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
