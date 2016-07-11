extern crate duktape_sys;

use std::mem;
use std::slice;
use std::str;

pub struct Context(*mut duktape_sys::duk_context);

#[derive(Debug, PartialEq)]
pub enum Value {
    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
}

impl Context {
    pub fn new() -> Context {
        let ctx = unsafe { duktape_sys::duk_create_heap_default() };
        Context(ctx)
    }

    pub fn eval_string(&mut self, string: &str) -> Value {
        unsafe {
            duktape_sys::duk_eval_lstring(self.0, string.as_ptr() as *const i8, string.len());
            let v = self.get_value(-1);
            duktape_sys::duk_pop(self.0);
            v
        }
    }

    unsafe fn get_value(&mut self, index: duktape_sys::duk_idx_t) -> Value {
        let t = duktape_sys::duk_get_type(self.0, index);
        if t == duktape_sys::DUK_TYPE_UNDEFINED {
            Value::Undefined
        } else if t == duktape_sys::DUK_TYPE_NULL {
            Value::Null
        } else if t == duktape_sys::DUK_TYPE_BOOLEAN {
            Value::Boolean(duktape_sys::duk_get_boolean(self.0, index) != 0)
        } else if t == duktape_sys::DUK_TYPE_NUMBER {
            Value::Number(duktape_sys::duk_get_number(self.0, index))
        } else if t == duktape_sys::DUK_TYPE_STRING {
            let mut len = mem::uninitialized();
            let data = duktape_sys::duk_get_lstring(self.0, index, &mut len);
            let slice = slice::from_raw_parts(data as *const u8, len);
            Value::String(String::from(str::from_utf8(slice).unwrap()))
        } else {
            unimplemented!()
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { duktape_sys::duk_destroy_heap(self.0) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_eval_undefined() {
        let mut ctx = Context::new();
        let value = ctx.eval_string("undefined");
        assert_eq!(Value::Undefined, value);
    }

    #[test]
    fn basic_eval_null() {
        let mut ctx = Context::new();
        let value = ctx.eval_string("null");
        assert_eq!(Value::Null, value);
    }

    #[test]
    fn basic_eval_boolean_true() {
        let mut ctx = Context::new();
        let value = ctx.eval_string("true");
        assert_eq!(Value::Boolean(true), value);
    }

    #[test]
    fn basic_eval_boolean_false() {
        let mut ctx = Context::new();
        let value = ctx.eval_string("false");
        assert_eq!(Value::Boolean(false), value);
    }

    #[test]
    fn basic_eval_number_integral() {
        let mut ctx = Context::new();
        let value = ctx.eval_string("4");
        assert_eq!(Value::Number(4.0), value);
    }

    #[test]
    fn basic_eval_number_fractional() {
        let mut ctx = Context::new();
        let value = ctx.eval_string("0.5");
        assert_eq!(Value::Number(0.5), value);
    }

    #[test]
    fn basic_eval_string() {
        let mut ctx = Context::new();
        let value = ctx.eval_string("'ab'");
        assert_eq!(Value::String("ab".to_owned()), value);
    }
}
