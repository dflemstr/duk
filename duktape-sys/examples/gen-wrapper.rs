extern crate bindgen;

use std::borrow;
use std::fs;
use std::io;

fn main() {
    write_wrapper_header().unwrap();
    write_wrapper_c_file().unwrap();
    write_ffi().unwrap();
}

fn write_ffi() -> io::Result<()> {
    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .clang_arg("-Iduktape/src")
        .clang_arg("-Iduktape/extras/logging")
        .clang_arg("-Iduktape/extras/module-node")
        .clang_arg("-std=c99")
        .whitelist_function("duk_.*")
        .whitelist_function("DUK_.*")
        .whitelist_var("duk_.*")
        .whitelist_var("DUK_.*")
        .whitelist_type("duk_.*")
        .whitelist_type("DUK_.*")
        .derive_debug(true)
        .generate()
        .unwrap();

    bindings.write_to_file("src/ffi.rs")?;

    Ok(())
}

fn write_wrapper_header() -> io::Result<()> {
    use std::io::Write;

    let mut header_file = fs::File::create("src/wrapper.h")?;

    writeln!(header_file, "#pragma once")?;
    writeln!(header_file, "#include \"duktape.h\"")?;
    writeln!(header_file, "#include \"duk_logging.h\"")?;
    writeln!(header_file, "#include \"duk_module_node.h\"")?;

    for &(t, n) in MACRO_CONSTANTS {
        writeln!(header_file, "")?;
        writeln!(header_file, "#pragma push_macro({:?})", n)?;
        writeln!(header_file, "#undef {}", n)?;
        writeln!(header_file, "const {} {};", t, n)?;
        writeln!(header_file, "#pragma pop_macro({:?})", n)?;
    }

    for &(rt, n, ps) in MACRO_FUNCTIONS {
        writeln!(header_file, "")?;
        writeln!(header_file, "#pragma push_macro({:?})", n)?;
        writeln!(header_file, "#undef {}", n)?;
        let params = join(ps.iter().map(|&(pt, pn)| format!("{} {}", pt, pn)), ", ");
        writeln!(header_file, "{} {}({});", rt, n, params)?;
        writeln!(header_file, "#pragma pop_macro({:?})", n)?;
    }
    Ok(())
}

fn write_wrapper_c_file() -> io::Result<()> {
    use std::io::Write;

    let mut c_file = fs::File::create("src/wrapper.c")?;

    writeln!(c_file, "#include \"wrapper.h\"")?;

    for &(t, n) in MACRO_CONSTANTS {
        writeln!(c_file, "")?;
        writeln!(c_file, "#pragma push_macro({:?})", n)?;
        writeln!(c_file, "#undef {}", n)?;
        writeln!(c_file, "const {} {} =", t, n)?;
        writeln!(c_file, "#pragma pop_macro({:?})", n)?;
        writeln!(c_file, "  {};", n)?;
    }

    for &(rt, n, ps) in MACRO_FUNCTIONS {
        writeln!(c_file, "")?;
        writeln!(c_file, "#pragma push_macro({:?})", n)?;
        writeln!(c_file, "#undef {}", n)?;
        let params = join(ps.iter().map(|&(pt, pn)| format!("{} {}", pt, pn)), ", ");
        writeln!(c_file, "{} {}({}) {{", rt, n, params)?;
        writeln!(c_file, "#pragma pop_macro({:?})", n)?;
        let args = join(ps.iter().map(|&(_, pn)| pn), ", ");
        let maybe_return = if rt == "void" { "" } else { "return " };
        writeln!(c_file, "  {}{}({});", maybe_return, n, args)?;
        writeln!(c_file, "}}")?;
    }
    Ok(())
}

fn join<S, I>(iter: I, sep: &str) -> String
where
    S: borrow::Borrow<str>,
    I: Iterator<Item = S>,
{
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
    ("duk_errcode_t", "DUK_ERR_NONE"),
    ("duk_errcode_t", "DUK_ERR_ERROR"),
    ("duk_errcode_t", "DUK_ERR_EVAL_ERROR"),
    ("duk_errcode_t", "DUK_ERR_RANGE_ERROR"),
    ("duk_errcode_t", "DUK_ERR_REFERENCE_ERROR"),
    ("duk_errcode_t", "DUK_ERR_SYNTAX_ERROR"),
    ("duk_errcode_t", "DUK_ERR_TYPE_ERROR"),
    ("duk_errcode_t", "DUK_ERR_URI_ERROR"),
    ("duk_ret_t", "DUK_RET_ERROR"),
    ("duk_ret_t", "DUK_RET_EVAL_ERROR"),
    ("duk_ret_t", "DUK_RET_RANGE_ERROR"),
    ("duk_ret_t", "DUK_RET_REFERENCE_ERROR"),
    ("duk_ret_t", "DUK_RET_SYNTAX_ERROR"),
    ("duk_ret_t", "DUK_RET_TYPE_ERROR"),
    ("duk_ret_t", "DUK_RET_URI_ERROR"),
    ("duk_int_t", "DUK_EXEC_SUCCESS"),
    ("duk_int_t", "DUK_EXEC_ERROR"),
    ("long", "DUK_LEVEL_DEBUG"),
    ("long", "DUK_LEVEL_DDEBUG"),
    ("long", "DUK_LEVEL_DDDEBUG"),
    ("duk_int_t", "DUK_LOG_TRACE"),
    ("duk_int_t", "DUK_LOG_DEBUG"),
    ("duk_int_t", "DUK_LOG_INFO"),
    ("duk_int_t", "DUK_LOG_WARN"),
    ("duk_int_t", "DUK_LOG_ERROR"),
    ("duk_int_t", "DUK_LOG_FATAL"),
];

const MACRO_FUNCTIONS: &'static [(
    &'static str,
    &'static str,
    &'static [(&'static str, &'static str)],
)] = &[
    ("duk_context *", "duk_create_heap_default", &[]),
    (
        "void",
        "duk_xmove_top",
        &[
            ("duk_context *", "to_ctx"),
            ("duk_context *", "from_ctx"),
            ("duk_idx_t", "count"),
        ],
    ),
    (
        "void",
        "duk_xcopy_top",
        &[
            ("duk_context *", "to_ctx"),
            ("duk_context *", "from_ctx"),
            ("duk_idx_t", "count"),
        ],
    ),
    (
        "const char *",
        "duk_push_string_file",
        &[("duk_context *", "ctx"), ("const char *", "path")],
    ),
    ("duk_idx_t", "duk_push_thread", &[("duk_context *", "ctx")]),
    (
        "duk_idx_t",
        "duk_push_thread_new_globalenv",
        &[("duk_context *", "ctx")],
    ),
    (
        "duk_idx_t",
        "duk_push_error_object",
        &[
            ("duk_context *", "ctx"),
            ("duk_errcode_t", "err_code"),
            ("const char *", "fmt"),
        ],
    ),
    (
        "void *",
        "duk_push_buffer",
        &[
            ("duk_context *", "ctx"),
            ("duk_size_t", "size"),
            ("duk_bool_t", "dynamic"),
        ],
    ),
    (
        "void *",
        "duk_push_fixed_buffer",
        &[("duk_context *", "ctx"), ("duk_size_t", "size")],
    ),
    (
        "void *",
        "duk_push_dynamic_buffer",
        &[("duk_context *", "ctx"), ("duk_size_t", "size")],
    ),
    (
        "void",
        "duk_push_external_buffer",
        &[("duk_context *", "ctx")],
    ),
    (
        "duk_bool_t",
        "duk_is_callable",
        &[("duk_context *", "ctx"), ("duk_idx_t", "index")],
    ),
    (
        "duk_bool_t",
        "duk_is_primitive",
        &[("duk_context *", "ctx"), ("duk_idx_t", "index")],
    ),
    (
        "duk_bool_t",
        "duk_is_object_coercible",
        &[("duk_context *", "ctx"), ("duk_idx_t", "index")],
    ),
    (
        "duk_bool_t",
        "duk_is_error",
        &[("duk_context *", "ctx"), ("duk_idx_t", "index")],
    ),
    (
        "duk_bool_t",
        "duk_is_eval_error",
        &[("duk_context *", "ctx"), ("duk_idx_t", "index")],
    ),
    (
        "duk_bool_t",
        "duk_is_range_error",
        &[("duk_context *", "ctx"), ("duk_idx_t", "index")],
    ),
    (
        "duk_bool_t",
        "duk_is_reference_error",
        &[("duk_context *", "ctx"), ("duk_idx_t", "index")],
    ),
    (
        "duk_bool_t",
        "duk_is_syntax_error",
        &[("duk_context *", "ctx"), ("duk_idx_t", "index")],
    ),
    (
        "duk_bool_t",
        "duk_is_type_error",
        &[("duk_context *", "ctx"), ("duk_idx_t", "index")],
    ),
    (
        "duk_bool_t",
        "duk_is_uri_error",
        &[("duk_context *", "ctx"), ("duk_idx_t", "index")],
    ),
    (
        "void",
        "duk_require_type_mask",
        &[
            ("duk_context *", "ctx"),
            ("duk_idx_t", "index"),
            ("duk_uint_t", "mask"),
        ],
    ),
    (
        "void",
        "duk_require_callable",
        &[("duk_context *", "ctx"), ("duk_idx_t", "index")],
    ),
    (
        "void",
        "duk_require_object_coercible",
        &[("duk_context *", "ctx"), ("duk_idx_t", "index")],
    ),
    (
        "void *",
        "duk_to_buffer",
        &[
            ("duk_context *", "ctx"),
            ("duk_idx_t", "index"),
            ("duk_size_t *", "out_size"),
        ],
    ),
    (
        "void *",
        "duk_to_fixed_buffer",
        &[
            ("duk_context *", "ctx"),
            ("duk_idx_t", "index"),
            ("duk_size_t *", "out_size"),
        ],
    ),
    (
        "void *",
        "duk_to_dynamic_buffer",
        &[
            ("duk_context *", "ctx"),
            ("duk_idx_t", "index"),
            ("duk_size_t *", "out_size"),
        ],
    ),
    (
        "const char *",
        "duk_safe_to_string",
        &[("duk_context *", "ctx"), ("duk_idx_t", "index")],
    ),
    ("void", "duk_eval", &[("duk_context *", "ctx")]),
    ("void", "duk_eval_noresult", &[("duk_context *", "ctx")]),
    ("duk_int_t", "duk_peval", &[("duk_context *", "ctx")]),
    (
        "duk_int_t",
        "duk_peval_noresult",
        &[("duk_context *", "ctx")],
    ),
    (
        "void",
        "duk_compile",
        &[("duk_context *", "ctx"), ("duk_uint_t", "flags")],
    ),
    (
        "duk_int_t",
        "duk_pcompile",
        &[("duk_context *", "ctx"), ("duk_uint_t", "flags")],
    ),
    (
        "void",
        "duk_eval_string",
        &[("duk_context *", "ctx"), ("const char *", "src")],
    ),
    (
        "void",
        "duk_eval_string_noresult",
        &[("duk_context *", "ctx"), ("const char *", "src")],
    ),
    (
        "duk_int_t",
        "duk_peval_string",
        &[("duk_context *", "ctx"), ("const char *", "src")],
    ),
    (
        "duk_int_t",
        "duk_peval_string_noresult",
        &[("duk_context *", "ctx"), ("const char *", "src")],
    ),
    (
        "void",
        "duk_compile_string",
        &[
            ("duk_context *", "ctx"),
            ("duk_uint_t", "flags"),
            ("const char *", "src"),
        ],
    ),
    (
        "void",
        "duk_compile_string_filename",
        &[
            ("duk_context *", "ctx"),
            ("duk_uint_t", "flags"),
            ("const char *", "src"),
        ],
    ),
    (
        "duk_int_t",
        "duk_pcompile_string",
        &[
            ("duk_context *", "ctx"),
            ("duk_uint_t", "flags"),
            ("const char *", "src"),
        ],
    ),
    (
        "duk_int_t",
        "duk_pcompile_string_filename",
        &[
            ("duk_context *", "ctx"),
            ("duk_uint_t", "flags"),
            ("const char *", "src"),
        ],
    ),
    (
        "void",
        "duk_eval_lstring",
        &[
            ("duk_context *", "ctx"),
            ("const char *", "buf"),
            ("duk_size_t", "len"),
        ],
    ),
    (
        "void",
        "duk_eval_lstring_noresult",
        &[
            ("duk_context *", "ctx"),
            ("const char *", "buf"),
            ("duk_size_t", "len"),
        ],
    ),
    (
        "duk_int_t",
        "duk_peval_lstring",
        &[
            ("duk_context *", "ctx"),
            ("const char *", "buf"),
            ("duk_size_t", "len"),
        ],
    ),
    (
        "duk_int_t",
        "duk_peval_lstring_noresult",
        &[
            ("duk_context *", "ctx"),
            ("const char *", "buf"),
            ("duk_size_t", "len"),
        ],
    ),
    (
        "void",
        "duk_compile_lstring",
        &[
            ("duk_context *", "ctx"),
            ("duk_uint_t", "flags"),
            ("const char *", "buf"),
            ("duk_size_t", "len"),
        ],
    ),
    (
        "void",
        "duk_compile_lstring_filename",
        &[
            ("duk_context *", "ctx"),
            ("duk_uint_t", "flags"),
            ("const char *", "buf"),
            ("duk_size_t", "len"),
        ],
    ),
    (
        "duk_int_t",
        "duk_pcompile_lstring",
        &[
            ("duk_context *", "ctx"),
            ("duk_uint_t", "flags"),
            ("const char *", "buf"),
            ("duk_size_t", "len"),
        ],
    ),
    (
        "duk_int_t",
        "duk_pcompile_lstring_filename",
        &[
            ("duk_context *", "ctx"),
            ("duk_uint_t", "flags"),
            ("const char *", "buf"),
            ("duk_size_t", "len"),
        ],
    ),
    (
        "void",
        "duk_eval_file",
        &[("duk_context *", "ctx"), ("const char *", "path")],
    ),
    (
        "void",
        "duk_eval_file_noresult",
        &[("duk_context *", "ctx"), ("const char *", "path")],
    ),
    (
        "duk_int_t",
        "duk_peval_file",
        &[("duk_context *", "ctx"), ("const char *", "path")],
    ),
    (
        "duk_int_t",
        "duk_peval_file_noresult",
        &[("duk_context *", "ctx"), ("const char *", "path")],
    ),
    (
        "void",
        "duk_compile_file",
        &[
            ("duk_context *", "ctx"),
            ("duk_uint_t", "flags"),
            ("const char *", "path"),
        ],
    ),
    (
        "duk_int_t",
        "duk_pcompile_file",
        &[
            ("duk_context *", "ctx"),
            ("duk_uint_t", "flags"),
            ("const char *", "path"),
        ],
    ),
    (
        "void",
        "duk_dump_context_stdout",
        &[("duk_context *", "ctx")],
    ),
    (
        "void",
        "duk_dump_context_stderr",
        &[("duk_context *", "ctx")],
    ),
];
