extern crate bindgen;
extern crate gcc;

use std::borrow;
use std::env;
use std::path;
use std::fs;

fn main() {
    use std::io::Write;

    let value = &env::var("OUT_DIR").unwrap();
    let cur_dir = env::current_dir().unwrap();
    let out_dir = path::Path::new(value);

    let duktape_header_path = cur_dir.join("duktape/src/duktape.h");
    let wrapper_header_path = out_dir.join("wrapper.h");
    let wrapper_c_file_path = out_dir.join("wrapper.c");

    write_wrapper_header(&wrapper_header_path, &duktape_header_path);
    write_wrapper_c_file(&wrapper_c_file_path, &wrapper_header_path);

    gcc::compile_library(
        "libduktape.a", &["duktape/src/duktape.c", wrapper_c_file_path.to_str().unwrap()]);

    let bindings = bindgen::Builder::new()
        .header(wrapper_header_path.to_str().unwrap())
        .forbid_unknown_types()
        .derive_debug(true)
        .rust_enums(true)
        .builtins()
        .generate()
        .unwrap()
        .to_string()
        .replace("#![allow(dead_code,\n         non_camel_case_types,\n         non_upper_case_globals,\n         non_snake_case)]", "");

    fs::File::create(out_dir.join("ffi.rs"))
        .unwrap()
        .write_all(bindings.as_bytes())
        .unwrap();
}

fn write_wrapper_header(path: &path::Path, duktape_header_path: &path::Path) {
    use std::io::Write;

    let mut header_file = fs::File::create(path).unwrap();

    writeln!(header_file, "#pragma once").unwrap();
    writeln!(header_file, "#include {:?}", duktape_header_path).unwrap();

    for &(t, n) in MACRO_CONSTANTS {
        writeln!(header_file, "").unwrap();
        writeln!(header_file, "#pragma push_macro({:?})", n).unwrap();
        writeln!(header_file, "#undef {}", n).unwrap();
        writeln!(header_file, "const {} {};", t, n).unwrap();
        writeln!(header_file, "#pragma pop_macro({:?})", n).unwrap();
    }

    for &(rt, n, ps) in MACRO_FUNCTIONS {
        writeln!(header_file, "").unwrap();
        writeln!(header_file, "#pragma push_macro({:?})", n).unwrap();
        writeln!(header_file, "#undef {}", n).unwrap();
        let params = join(ps.iter().map(|&(pt, pn)| format!("{} {}", pt, pn)), ", ");
        writeln!(header_file, "{} {}({});", rt, n, params).unwrap();
        writeln!(header_file, "#pragma pop_macro({:?})", n).unwrap();
    }
}

fn write_wrapper_c_file(path: &path::Path, wrapper_header_path: &path::Path) {
    use std::io::Write;

    let mut c_file = fs::File::create(path).unwrap();

    writeln!(c_file, "#include {:?}", wrapper_header_path).unwrap();

    for &(t, n) in MACRO_CONSTANTS {
        writeln!(c_file, "").unwrap();
        writeln!(c_file, "#pragma push_macro({:?})", n).unwrap();
        writeln!(c_file, "#undef {}", n).unwrap();
        writeln!(c_file, "const {} {} =", t, n).unwrap();
        writeln!(c_file, "#pragma pop_macro({:?})", n).unwrap();
        writeln!(c_file, "  {};", n).unwrap();
    }

    for &(rt, n, ps) in MACRO_FUNCTIONS {
        writeln!(c_file, "").unwrap();
        writeln!(c_file, "#pragma push_macro({:?})", n).unwrap();
        writeln!(c_file, "#undef {}", n).unwrap();
        let params = join(ps.iter().map(|&(pt, pn)| format!("{} {}", pt, pn)), ", ");
        writeln!(c_file, "{} {}({}) {{", rt, n, params).unwrap();
        writeln!(c_file, "#pragma pop_macro({:?})", n).unwrap();
        let args = join(ps.iter().map(|&(_, pn)| pn), ", ");
        let maybe_return = if rt == "void" { "" } else { "return " };
        writeln!(c_file, "  {}{}({});", maybe_return, n, args).unwrap();
        writeln!(c_file, "}}").unwrap();
    }
}

fn join<S, I>(iter: I, sep: &str) -> String where S: borrow::Borrow<str>, I: Iterator<Item=S> {
    iter.collect::<Vec<_>>().join(sep)
}

const MACRO_CONSTANTS: &'static [(&'static str, &'static str)] = &[
    ("duk_uint_t", "DUK_VERSION"),

    ("char *const", "DUK_GIT_COMMIT"),
    ("char *const", "DUK_GIT_DESCRIBE"),
    ("char *const", "DUK_GIT_BRANCH"),

    ("duk_uint_t", "DUK_DEBUG_PROTOCOL_VERSION"),

    ("duk_idx_t", "DUK_INVALID_INDEX"),

    ("duk_int_t", "DUK_VARARGS"),

    ("duk_size_t", "DUK_API_ENTRY_STACK"),

    ("duk_int_t", "DUK_TYPE_MIN"),
    ("duk_int_t", "DUK_TYPE_NONE"),
    ("duk_int_t", "DUK_TYPE_UNDEFINED"),
    ("duk_int_t", "DUK_TYPE_NULL"),
    ("duk_int_t", "DUK_TYPE_BOOLEAN"),
    ("duk_int_t", "DUK_TYPE_NUMBER"),
    ("duk_int_t", "DUK_TYPE_STRING"),
    ("duk_int_t", "DUK_TYPE_OBJECT"),
    ("duk_int_t", "DUK_TYPE_BUFFER"),
    ("duk_int_t", "DUK_TYPE_POINTER"),
    ("duk_int_t", "DUK_TYPE_LIGHTFUNC"),
    ("duk_int_t", "DUK_TYPE_MAX"),

    ("duk_uint_t", "DUK_TYPE_MASK_NONE"),
    ("duk_uint_t", "DUK_TYPE_MASK_UNDEFINED"),
    ("duk_uint_t", "DUK_TYPE_MASK_NULL"),
    ("duk_uint_t", "DUK_TYPE_MASK_BOOLEAN"),
    ("duk_uint_t", "DUK_TYPE_MASK_NUMBER"),
    ("duk_uint_t", "DUK_TYPE_MASK_STRING"),
    ("duk_uint_t", "DUK_TYPE_MASK_OBJECT"),
    ("duk_uint_t", "DUK_TYPE_MASK_BUFFER"),
    ("duk_uint_t", "DUK_TYPE_MASK_POINTER"),
    ("duk_uint_t", "DUK_TYPE_MASK_LIGHTFUNC"),
    ("duk_uint_t", "DUK_TYPE_MASK_THROW"),

    ("duk_int_t", "DUK_HINT_NONE"),
    ("duk_int_t", "DUK_HINT_STRING"),
    ("duk_int_t", "DUK_HINT_NUMBER"),

    ("duk_uint_t", "DUK_ENUM_INCLUDE_NONENUMERABLE"),
    ("duk_uint_t", "DUK_ENUM_INCLUDE_INTERNAL"),
    ("duk_uint_t", "DUK_ENUM_OWN_PROPERTIES_ONLY"),
    ("duk_uint_t", "DUK_ENUM_ARRAY_INDICES_ONLY"),
    ("duk_uint_t", "DUK_ENUM_SORT_ARRAY_INDICES"),
    ("duk_uint_t", "DUK_ENUM_NO_PROXY_BEHAVIOR"),

    ("duk_uint_t", "DUK_COMPILE_EVAL"),
    ("duk_uint_t", "DUK_COMPILE_FUNCTION"),
    ("duk_uint_t", "DUK_COMPILE_STRICT"),
    ("duk_uint_t", "DUK_COMPILE_SAFE"),
    ("duk_uint_t", "DUK_COMPILE_NORESULT"),
    ("duk_uint_t", "DUK_COMPILE_NOSOURCE"),
    ("duk_uint_t", "DUK_COMPILE_STRLEN"),
    ("duk_uint_t", "DUK_COMPILE_NOFILENAME"),

    ("duk_uint_t", "DUK_DEFPROP_WRITABLE"),
    ("duk_uint_t", "DUK_DEFPROP_ENUMERABLE"),
    ("duk_uint_t", "DUK_DEFPROP_CONFIGURABLE"),
    ("duk_uint_t", "DUK_DEFPROP_HAVE_WRITABLE"),
    ("duk_uint_t", "DUK_DEFPROP_HAVE_ENUMERABLE"),
    ("duk_uint_t", "DUK_DEFPROP_HAVE_CONFIGURABLE"),
    ("duk_uint_t", "DUK_DEFPROP_HAVE_VALUE"),
    ("duk_uint_t", "DUK_DEFPROP_HAVE_GETTER"),
    ("duk_uint_t", "DUK_DEFPROP_HAVE_SETTER"),
    ("duk_uint_t", "DUK_DEFPROP_FORCE"),
    ("duk_uint_t", "DUK_DEFPROP_SET_WRITABLE"),
    ("duk_uint_t", "DUK_DEFPROP_CLEAR_WRITABLE"),
    ("duk_uint_t", "DUK_DEFPROP_SET_ENUMERABLE"),
    ("duk_uint_t", "DUK_DEFPROP_CLEAR_ENUMERABLE"),
    ("duk_uint_t", "DUK_DEFPROP_SET_CONFIGURABLE"),
    ("duk_uint_t", "DUK_DEFPROP_CLEAR_CONFIGURABLE"),

    ("duk_uint_t", "DUK_THREAD_NEW_GLOBAL_ENV"),

    ("duk_uint_t", "DUK_STRING_PUSH_SAFE"),

    ("duk_errcode_t", "DUK_ERR_NONE"),
    ("duk_errcode_t", "DUK_ERR_UNIMPLEMENTED_ERROR"),
    ("duk_errcode_t", "DUK_ERR_UNSUPPORTED_ERROR"),
    ("duk_errcode_t", "DUK_ERR_INTERNAL_ERROR"),
    ("duk_errcode_t", "DUK_ERR_ALLOC_ERROR"),
    ("duk_errcode_t", "DUK_ERR_ASSERTION_ERROR"),
    ("duk_errcode_t", "DUK_ERR_API_ERROR"),
    ("duk_errcode_t", "DUK_ERR_UNCAUGHT_ERROR"),

    ("duk_errcode_t", "DUK_ERR_ERROR"),
    ("duk_errcode_t", "DUK_ERR_EVAL_ERROR"),
    ("duk_errcode_t", "DUK_ERR_RANGE_ERROR"),
    ("duk_errcode_t", "DUK_ERR_REFERENCE_ERROR"),
    ("duk_errcode_t", "DUK_ERR_SYNTAX_ERROR"),
    ("duk_errcode_t", "DUK_ERR_TYPE_ERROR"),
    ("duk_errcode_t", "DUK_ERR_URI_ERROR"),

    ("duk_ret_t", "DUK_RET_UNIMPLEMENTED_ERROR"),
    ("duk_ret_t", "DUK_RET_UNSUPPORTED_ERROR"),
    ("duk_ret_t", "DUK_RET_INTERNAL_ERROR"),
    ("duk_ret_t", "DUK_RET_ALLOC_ERROR"),
    ("duk_ret_t", "DUK_RET_ASSERTION_ERROR"),
    ("duk_ret_t", "DUK_RET_API_ERROR"),
    ("duk_ret_t", "DUK_RET_UNCAUGHT_ERROR"),
    ("duk_ret_t", "DUK_RET_ERROR"),
    ("duk_ret_t", "DUK_RET_EVAL_ERROR"),
    ("duk_ret_t", "DUK_RET_RANGE_ERROR"),
    ("duk_ret_t", "DUK_RET_REFERENCE_ERROR"),
    ("duk_ret_t", "DUK_RET_SYNTAX_ERROR"),
    ("duk_ret_t", "DUK_RET_TYPE_ERROR"),
    ("duk_ret_t", "DUK_RET_URI_ERROR"),

    ("duk_ret_t", "DUK_EXEC_SUCCESS"),
    ("duk_ret_t", "DUK_EXEC_ERROR"),

    ("duk_int_t", "DUK_LOG_TRACE"),
    ("duk_int_t", "DUK_LOG_DEBUG"),
    ("duk_int_t", "DUK_LOG_INFO"),
    ("duk_int_t", "DUK_LOG_WARN"),
    ("duk_int_t", "DUK_LOG_ERROR"),
    ("duk_int_t", "DUK_LOG_FATAL"),
];

const MACRO_FUNCTIONS: &'static [(&'static str, &'static str, &'static [(&'static str, &'static str)])] = &[
    ("duk_context *", "duk_create_heap_default", &[]),

    ("void", "duk_xmove_top",
     &[("duk_context *", "to_ctx"), ("duk_context *", "from_ctx"), ("duk_idx_t", "count")]),
    ("void", "duk_xcopy_top",
     &[("duk_context *", "to_ctx"), ("duk_context *", "from_ctx"), ("duk_idx_t", "count")]),

    ("const char *", "duk_push_string_file",
     &[("duk_context *", "ctx"), ("const char *", "path")]),

    ("duk_idx_t", "duk_push_thread",
     &[("duk_context *", "ctx")]),
    ("duk_idx_t", "duk_push_thread_new_globalenv",
     &[("duk_context *", "ctx")]),

    ("void *", "duk_push_buffer",
     &[("duk_context *", "ctx"), ("duk_size_t", "size"), ("duk_bool_t", "dynamic")]),
    ("void *", "duk_push_fixed_buffer",
     &[("duk_context *", "ctx"), ("duk_size_t", "size")]),
    ("void *", "duk_push_dynamic_buffer",
     &[("duk_context *", "ctx"), ("duk_size_t", "size")]),
    ("void", "duk_push_external_buffer",
     &[("duk_context *", "ctx")]),

    ("duk_bool_t", "duk_is_callable",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index")]),
    ("duk_bool_t", "duk_is_primitive",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index")]),
    ("duk_bool_t", "duk_is_object_coercible",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index")]),

    ("duk_bool_t", "duk_is_error",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index")]),
    ("duk_bool_t", "duk_is_eval_error",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index")]),
    ("duk_bool_t", "duk_is_range_error",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index")]),
    ("duk_bool_t", "duk_is_reference_error",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index")]),
    ("duk_bool_t", "duk_is_syntax_error",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index")]),
    ("duk_bool_t", "duk_is_type_error",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index")]),
    ("duk_bool_t", "duk_is_uri_error",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index")]),

    ("void", "duk_require_type_mask",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index"), ("duk_uint_t", "mask")]),

    ("void", "duk_require_callable",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index")]),
    ("void", "duk_require_object_coercible",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index")]),

    ("void *", "duk_to_buffer",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index"), ("duk_size_t *", "out_size")]),
    ("void *", "duk_to_fixed_buffer",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index"), ("duk_size_t *", "out_size")]),
    ("void *", "duk_to_dynamic_buffer",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index"), ("duk_size_t *", "out_size")]),

    ("const char *", "duk_safe_to_string",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index")]),

    ("void", "duk_eval",
     &[("duk_context *", "ctx")]),
    ("void", "duk_eval_noresult",
     &[("duk_context *", "ctx")]),
    ("duk_int_t", "duk_peval",
     &[("duk_context *", "ctx")]),
    ("duk_int_t", "duk_peval_noresult",
     &[("duk_context *", "ctx")]),
    ("void", "duk_compile",
     &[("duk_context *", "ctx"), ("duk_uint_t", "flags")]),
    ("duk_int_t", "duk_pcompile",
     &[("duk_context *", "ctx"), ("duk_uint_t", "flags")]),

    ("void", "duk_eval_string",
     &[("duk_context *", "ctx"), ("const char *", "src")]),
    ("void", "duk_eval_string_noresult",
     &[("duk_context *", "ctx"), ("const char *", "src")]),
    ("duk_int_t", "duk_peval_string",
     &[("duk_context *", "ctx"), ("const char *", "src")]),
    ("duk_int_t", "duk_peval_string_noresult",
     &[("duk_context *", "ctx"), ("const char *", "src")]),
    ("void", "duk_compile_string",
     &[("duk_context *", "ctx"), ("duk_uint_t", "flags"), ("const char *", "src")]),
    ("void", "duk_compile_string_filename",
     &[("duk_context *", "ctx"), ("duk_uint_t", "flags"), ("const char *", "src")]),
    ("duk_int_t", "duk_pcompile_string",
     &[("duk_context *", "ctx"), ("duk_uint_t", "flags"), ("const char *", "src")]),
    ("duk_int_t", "duk_pcompile_string_filename",
     &[("duk_context *", "ctx"), ("duk_uint_t", "flags"), ("const char *", "src")]),

    ("void", "duk_eval_lstring",
     &[("duk_context *", "ctx"), ("const char *", "buf"), ("duk_size_t", "len")]),
    ("void", "duk_eval_lstring_noresult",
     &[("duk_context *", "ctx"), ("const char *", "buf"), ("duk_size_t", "len")]),
    ("duk_int_t", "duk_peval_lstring",
     &[("duk_context *", "ctx"), ("const char *", "buf"), ("duk_size_t", "len")]),
    ("duk_int_t", "duk_peval_lstring_noresult",
     &[("duk_context *", "ctx"), ("const char *", "buf"), ("duk_size_t", "len")]),
    ("void", "duk_compile_lstring",
     &[("duk_context *", "ctx"), ("duk_uint_t", "flags"), ("const char *", "buf"), ("duk_size_t", "len")]),
    ("void", "duk_compile_lstring_filename",
     &[("duk_context *", "ctx"), ("duk_uint_t", "flags"), ("const char *", "buf"), ("duk_size_t", "len")]),
    ("duk_int_t", "duk_pcompile_lstring",
     &[("duk_context *", "ctx"), ("duk_uint_t", "flags"), ("const char *", "buf"), ("duk_size_t", "len")]),
    ("duk_int_t", "duk_pcompile_lstring_filename",
     &[("duk_context *", "ctx"), ("duk_uint_t", "flags"), ("const char *", "buf"), ("duk_size_t", "len")]),

    ("void", "duk_eval_file",
     &[("duk_context *", "ctx"), ("const char *", "path")]),
    ("void", "duk_eval_file_noresult",
     &[("duk_context *", "ctx"), ("const char *", "path")]),
    ("duk_int_t", "duk_peval_file",
     &[("duk_context *", "ctx"), ("const char *", "path")]),
    ("duk_int_t", "duk_peval_file_noresult",
     &[("duk_context *", "ctx"), ("const char *", "path")]),
    ("void", "duk_compile_file",
     &[("duk_context *", "ctx"), ("duk_uint_t", "flags"), ("const char *", "path")]),
    ("duk_int_t", "duk_pcompile_file",
     &[("duk_context *", "ctx"), ("duk_uint_t", "flags"), ("const char *", "path")]),

    ("void", "duk_dump_context_stdout",
     &[("duk_context *", "ctx")]),
    ("void", "duk_dump_context_stderr",
     &[("duk_context *", "ctx")]),
];
