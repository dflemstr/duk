#include "wrapper.h"

#pragma push_macro("DUK_VERSION")
#undef DUK_VERSION
const duk_uint_t DUK_VERSION =
#pragma pop_macro("DUK_VERSION")
  DUK_VERSION;

#pragma push_macro("DUK_GIT_COMMIT")
#undef DUK_GIT_COMMIT
const char *const DUK_GIT_COMMIT =
#pragma pop_macro("DUK_GIT_COMMIT")
  DUK_GIT_COMMIT;

#pragma push_macro("DUK_GIT_DESCRIBE")
#undef DUK_GIT_DESCRIBE
const char *const DUK_GIT_DESCRIBE =
#pragma pop_macro("DUK_GIT_DESCRIBE")
  DUK_GIT_DESCRIBE;

#pragma push_macro("DUK_GIT_BRANCH")
#undef DUK_GIT_BRANCH
const char *const DUK_GIT_BRANCH =
#pragma pop_macro("DUK_GIT_BRANCH")
  DUK_GIT_BRANCH;

#pragma push_macro("DUK_DEBUG_PROTOCOL_VERSION")
#undef DUK_DEBUG_PROTOCOL_VERSION
const duk_uint_t DUK_DEBUG_PROTOCOL_VERSION =
#pragma pop_macro("DUK_DEBUG_PROTOCOL_VERSION")
  DUK_DEBUG_PROTOCOL_VERSION;

#pragma push_macro("DUK_INVALID_INDEX")
#undef DUK_INVALID_INDEX
const duk_idx_t DUK_INVALID_INDEX =
#pragma pop_macro("DUK_INVALID_INDEX")
  DUK_INVALID_INDEX;

#pragma push_macro("DUK_VARARGS")
#undef DUK_VARARGS
const duk_int_t DUK_VARARGS =
#pragma pop_macro("DUK_VARARGS")
  DUK_VARARGS;

#pragma push_macro("DUK_API_ENTRY_STACK")
#undef DUK_API_ENTRY_STACK
const duk_size_t DUK_API_ENTRY_STACK =
#pragma pop_macro("DUK_API_ENTRY_STACK")
  DUK_API_ENTRY_STACK;

#pragma push_macro("DUK_TYPE_MIN")
#undef DUK_TYPE_MIN
const duk_int_t DUK_TYPE_MIN =
#pragma pop_macro("DUK_TYPE_MIN")
  DUK_TYPE_MIN;

#pragma push_macro("DUK_TYPE_NONE")
#undef DUK_TYPE_NONE
const duk_int_t DUK_TYPE_NONE =
#pragma pop_macro("DUK_TYPE_NONE")
  DUK_TYPE_NONE;

#pragma push_macro("DUK_TYPE_UNDEFINED")
#undef DUK_TYPE_UNDEFINED
const duk_int_t DUK_TYPE_UNDEFINED =
#pragma pop_macro("DUK_TYPE_UNDEFINED")
  DUK_TYPE_UNDEFINED;

#pragma push_macro("DUK_TYPE_NULL")
#undef DUK_TYPE_NULL
const duk_int_t DUK_TYPE_NULL =
#pragma pop_macro("DUK_TYPE_NULL")
  DUK_TYPE_NULL;

#pragma push_macro("DUK_TYPE_BOOLEAN")
#undef DUK_TYPE_BOOLEAN
const duk_int_t DUK_TYPE_BOOLEAN =
#pragma pop_macro("DUK_TYPE_BOOLEAN")
  DUK_TYPE_BOOLEAN;

#pragma push_macro("DUK_TYPE_NUMBER")
#undef DUK_TYPE_NUMBER
const duk_int_t DUK_TYPE_NUMBER =
#pragma pop_macro("DUK_TYPE_NUMBER")
  DUK_TYPE_NUMBER;

#pragma push_macro("DUK_TYPE_STRING")
#undef DUK_TYPE_STRING
const duk_int_t DUK_TYPE_STRING =
#pragma pop_macro("DUK_TYPE_STRING")
  DUK_TYPE_STRING;

#pragma push_macro("DUK_TYPE_OBJECT")
#undef DUK_TYPE_OBJECT
const duk_int_t DUK_TYPE_OBJECT =
#pragma pop_macro("DUK_TYPE_OBJECT")
  DUK_TYPE_OBJECT;

#pragma push_macro("DUK_TYPE_BUFFER")
#undef DUK_TYPE_BUFFER
const duk_int_t DUK_TYPE_BUFFER =
#pragma pop_macro("DUK_TYPE_BUFFER")
  DUK_TYPE_BUFFER;

#pragma push_macro("DUK_TYPE_POINTER")
#undef DUK_TYPE_POINTER
const duk_int_t DUK_TYPE_POINTER =
#pragma pop_macro("DUK_TYPE_POINTER")
  DUK_TYPE_POINTER;

#pragma push_macro("DUK_TYPE_LIGHTFUNC")
#undef DUK_TYPE_LIGHTFUNC
const duk_int_t DUK_TYPE_LIGHTFUNC =
#pragma pop_macro("DUK_TYPE_LIGHTFUNC")
  DUK_TYPE_LIGHTFUNC;

#pragma push_macro("DUK_TYPE_MAX")
#undef DUK_TYPE_MAX
const duk_int_t DUK_TYPE_MAX =
#pragma pop_macro("DUK_TYPE_MAX")
  DUK_TYPE_MAX;

#pragma push_macro("DUK_TYPE_MASK_NONE")
#undef DUK_TYPE_MASK_NONE
const duk_uint_t DUK_TYPE_MASK_NONE =
#pragma pop_macro("DUK_TYPE_MASK_NONE")
  DUK_TYPE_MASK_NONE;

#pragma push_macro("DUK_TYPE_MASK_UNDEFINED")
#undef DUK_TYPE_MASK_UNDEFINED
const duk_uint_t DUK_TYPE_MASK_UNDEFINED =
#pragma pop_macro("DUK_TYPE_MASK_UNDEFINED")
  DUK_TYPE_MASK_UNDEFINED;

#pragma push_macro("DUK_TYPE_MASK_NULL")
#undef DUK_TYPE_MASK_NULL
const duk_uint_t DUK_TYPE_MASK_NULL =
#pragma pop_macro("DUK_TYPE_MASK_NULL")
  DUK_TYPE_MASK_NULL;

#pragma push_macro("DUK_TYPE_MASK_BOOLEAN")
#undef DUK_TYPE_MASK_BOOLEAN
const duk_uint_t DUK_TYPE_MASK_BOOLEAN =
#pragma pop_macro("DUK_TYPE_MASK_BOOLEAN")
  DUK_TYPE_MASK_BOOLEAN;

#pragma push_macro("DUK_TYPE_MASK_NUMBER")
#undef DUK_TYPE_MASK_NUMBER
const duk_uint_t DUK_TYPE_MASK_NUMBER =
#pragma pop_macro("DUK_TYPE_MASK_NUMBER")
  DUK_TYPE_MASK_NUMBER;

#pragma push_macro("DUK_TYPE_MASK_STRING")
#undef DUK_TYPE_MASK_STRING
const duk_uint_t DUK_TYPE_MASK_STRING =
#pragma pop_macro("DUK_TYPE_MASK_STRING")
  DUK_TYPE_MASK_STRING;

#pragma push_macro("DUK_TYPE_MASK_OBJECT")
#undef DUK_TYPE_MASK_OBJECT
const duk_uint_t DUK_TYPE_MASK_OBJECT =
#pragma pop_macro("DUK_TYPE_MASK_OBJECT")
  DUK_TYPE_MASK_OBJECT;

#pragma push_macro("DUK_TYPE_MASK_BUFFER")
#undef DUK_TYPE_MASK_BUFFER
const duk_uint_t DUK_TYPE_MASK_BUFFER =
#pragma pop_macro("DUK_TYPE_MASK_BUFFER")
  DUK_TYPE_MASK_BUFFER;

#pragma push_macro("DUK_TYPE_MASK_POINTER")
#undef DUK_TYPE_MASK_POINTER
const duk_uint_t DUK_TYPE_MASK_POINTER =
#pragma pop_macro("DUK_TYPE_MASK_POINTER")
  DUK_TYPE_MASK_POINTER;

#pragma push_macro("DUK_TYPE_MASK_LIGHTFUNC")
#undef DUK_TYPE_MASK_LIGHTFUNC
const duk_uint_t DUK_TYPE_MASK_LIGHTFUNC =
#pragma pop_macro("DUK_TYPE_MASK_LIGHTFUNC")
  DUK_TYPE_MASK_LIGHTFUNC;

#pragma push_macro("DUK_TYPE_MASK_THROW")
#undef DUK_TYPE_MASK_THROW
const duk_uint_t DUK_TYPE_MASK_THROW =
#pragma pop_macro("DUK_TYPE_MASK_THROW")
  DUK_TYPE_MASK_THROW;

#pragma push_macro("DUK_HINT_NONE")
#undef DUK_HINT_NONE
const duk_int_t DUK_HINT_NONE =
#pragma pop_macro("DUK_HINT_NONE")
  DUK_HINT_NONE;

#pragma push_macro("DUK_HINT_STRING")
#undef DUK_HINT_STRING
const duk_int_t DUK_HINT_STRING =
#pragma pop_macro("DUK_HINT_STRING")
  DUK_HINT_STRING;

#pragma push_macro("DUK_HINT_NUMBER")
#undef DUK_HINT_NUMBER
const duk_int_t DUK_HINT_NUMBER =
#pragma pop_macro("DUK_HINT_NUMBER")
  DUK_HINT_NUMBER;

#pragma push_macro("DUK_ENUM_INCLUDE_NONENUMERABLE")
#undef DUK_ENUM_INCLUDE_NONENUMERABLE
const duk_uint_t DUK_ENUM_INCLUDE_NONENUMERABLE =
#pragma pop_macro("DUK_ENUM_INCLUDE_NONENUMERABLE")
  DUK_ENUM_INCLUDE_NONENUMERABLE;

#pragma push_macro("DUK_ENUM_OWN_PROPERTIES_ONLY")
#undef DUK_ENUM_OWN_PROPERTIES_ONLY
const duk_uint_t DUK_ENUM_OWN_PROPERTIES_ONLY =
#pragma pop_macro("DUK_ENUM_OWN_PROPERTIES_ONLY")
  DUK_ENUM_OWN_PROPERTIES_ONLY;

#pragma push_macro("DUK_ENUM_ARRAY_INDICES_ONLY")
#undef DUK_ENUM_ARRAY_INDICES_ONLY
const duk_uint_t DUK_ENUM_ARRAY_INDICES_ONLY =
#pragma pop_macro("DUK_ENUM_ARRAY_INDICES_ONLY")
  DUK_ENUM_ARRAY_INDICES_ONLY;

#pragma push_macro("DUK_ENUM_SORT_ARRAY_INDICES")
#undef DUK_ENUM_SORT_ARRAY_INDICES
const duk_uint_t DUK_ENUM_SORT_ARRAY_INDICES =
#pragma pop_macro("DUK_ENUM_SORT_ARRAY_INDICES")
  DUK_ENUM_SORT_ARRAY_INDICES;

#pragma push_macro("DUK_ENUM_NO_PROXY_BEHAVIOR")
#undef DUK_ENUM_NO_PROXY_BEHAVIOR
const duk_uint_t DUK_ENUM_NO_PROXY_BEHAVIOR =
#pragma pop_macro("DUK_ENUM_NO_PROXY_BEHAVIOR")
  DUK_ENUM_NO_PROXY_BEHAVIOR;

#pragma push_macro("DUK_COMPILE_EVAL")
#undef DUK_COMPILE_EVAL
const duk_uint_t DUK_COMPILE_EVAL =
#pragma pop_macro("DUK_COMPILE_EVAL")
  DUK_COMPILE_EVAL;

#pragma push_macro("DUK_COMPILE_FUNCTION")
#undef DUK_COMPILE_FUNCTION
const duk_uint_t DUK_COMPILE_FUNCTION =
#pragma pop_macro("DUK_COMPILE_FUNCTION")
  DUK_COMPILE_FUNCTION;

#pragma push_macro("DUK_COMPILE_STRICT")
#undef DUK_COMPILE_STRICT
const duk_uint_t DUK_COMPILE_STRICT =
#pragma pop_macro("DUK_COMPILE_STRICT")
  DUK_COMPILE_STRICT;

#pragma push_macro("DUK_COMPILE_SAFE")
#undef DUK_COMPILE_SAFE
const duk_uint_t DUK_COMPILE_SAFE =
#pragma pop_macro("DUK_COMPILE_SAFE")
  DUK_COMPILE_SAFE;

#pragma push_macro("DUK_COMPILE_NORESULT")
#undef DUK_COMPILE_NORESULT
const duk_uint_t DUK_COMPILE_NORESULT =
#pragma pop_macro("DUK_COMPILE_NORESULT")
  DUK_COMPILE_NORESULT;

#pragma push_macro("DUK_COMPILE_NOSOURCE")
#undef DUK_COMPILE_NOSOURCE
const duk_uint_t DUK_COMPILE_NOSOURCE =
#pragma pop_macro("DUK_COMPILE_NOSOURCE")
  DUK_COMPILE_NOSOURCE;

#pragma push_macro("DUK_COMPILE_STRLEN")
#undef DUK_COMPILE_STRLEN
const duk_uint_t DUK_COMPILE_STRLEN =
#pragma pop_macro("DUK_COMPILE_STRLEN")
  DUK_COMPILE_STRLEN;

#pragma push_macro("DUK_COMPILE_NOFILENAME")
#undef DUK_COMPILE_NOFILENAME
const duk_uint_t DUK_COMPILE_NOFILENAME =
#pragma pop_macro("DUK_COMPILE_NOFILENAME")
  DUK_COMPILE_NOFILENAME;

#pragma push_macro("DUK_DEFPROP_WRITABLE")
#undef DUK_DEFPROP_WRITABLE
const duk_uint_t DUK_DEFPROP_WRITABLE =
#pragma pop_macro("DUK_DEFPROP_WRITABLE")
  DUK_DEFPROP_WRITABLE;

#pragma push_macro("DUK_DEFPROP_ENUMERABLE")
#undef DUK_DEFPROP_ENUMERABLE
const duk_uint_t DUK_DEFPROP_ENUMERABLE =
#pragma pop_macro("DUK_DEFPROP_ENUMERABLE")
  DUK_DEFPROP_ENUMERABLE;

#pragma push_macro("DUK_DEFPROP_CONFIGURABLE")
#undef DUK_DEFPROP_CONFIGURABLE
const duk_uint_t DUK_DEFPROP_CONFIGURABLE =
#pragma pop_macro("DUK_DEFPROP_CONFIGURABLE")
  DUK_DEFPROP_CONFIGURABLE;

#pragma push_macro("DUK_DEFPROP_HAVE_WRITABLE")
#undef DUK_DEFPROP_HAVE_WRITABLE
const duk_uint_t DUK_DEFPROP_HAVE_WRITABLE =
#pragma pop_macro("DUK_DEFPROP_HAVE_WRITABLE")
  DUK_DEFPROP_HAVE_WRITABLE;

#pragma push_macro("DUK_DEFPROP_HAVE_ENUMERABLE")
#undef DUK_DEFPROP_HAVE_ENUMERABLE
const duk_uint_t DUK_DEFPROP_HAVE_ENUMERABLE =
#pragma pop_macro("DUK_DEFPROP_HAVE_ENUMERABLE")
  DUK_DEFPROP_HAVE_ENUMERABLE;

#pragma push_macro("DUK_DEFPROP_HAVE_CONFIGURABLE")
#undef DUK_DEFPROP_HAVE_CONFIGURABLE
const duk_uint_t DUK_DEFPROP_HAVE_CONFIGURABLE =
#pragma pop_macro("DUK_DEFPROP_HAVE_CONFIGURABLE")
  DUK_DEFPROP_HAVE_CONFIGURABLE;

#pragma push_macro("DUK_DEFPROP_HAVE_VALUE")
#undef DUK_DEFPROP_HAVE_VALUE
const duk_uint_t DUK_DEFPROP_HAVE_VALUE =
#pragma pop_macro("DUK_DEFPROP_HAVE_VALUE")
  DUK_DEFPROP_HAVE_VALUE;

#pragma push_macro("DUK_DEFPROP_HAVE_GETTER")
#undef DUK_DEFPROP_HAVE_GETTER
const duk_uint_t DUK_DEFPROP_HAVE_GETTER =
#pragma pop_macro("DUK_DEFPROP_HAVE_GETTER")
  DUK_DEFPROP_HAVE_GETTER;

#pragma push_macro("DUK_DEFPROP_HAVE_SETTER")
#undef DUK_DEFPROP_HAVE_SETTER
const duk_uint_t DUK_DEFPROP_HAVE_SETTER =
#pragma pop_macro("DUK_DEFPROP_HAVE_SETTER")
  DUK_DEFPROP_HAVE_SETTER;

#pragma push_macro("DUK_DEFPROP_FORCE")
#undef DUK_DEFPROP_FORCE
const duk_uint_t DUK_DEFPROP_FORCE =
#pragma pop_macro("DUK_DEFPROP_FORCE")
  DUK_DEFPROP_FORCE;

#pragma push_macro("DUK_DEFPROP_SET_WRITABLE")
#undef DUK_DEFPROP_SET_WRITABLE
const duk_uint_t DUK_DEFPROP_SET_WRITABLE =
#pragma pop_macro("DUK_DEFPROP_SET_WRITABLE")
  DUK_DEFPROP_SET_WRITABLE;

#pragma push_macro("DUK_DEFPROP_CLEAR_WRITABLE")
#undef DUK_DEFPROP_CLEAR_WRITABLE
const duk_uint_t DUK_DEFPROP_CLEAR_WRITABLE =
#pragma pop_macro("DUK_DEFPROP_CLEAR_WRITABLE")
  DUK_DEFPROP_CLEAR_WRITABLE;

#pragma push_macro("DUK_DEFPROP_SET_ENUMERABLE")
#undef DUK_DEFPROP_SET_ENUMERABLE
const duk_uint_t DUK_DEFPROP_SET_ENUMERABLE =
#pragma pop_macro("DUK_DEFPROP_SET_ENUMERABLE")
  DUK_DEFPROP_SET_ENUMERABLE;

#pragma push_macro("DUK_DEFPROP_CLEAR_ENUMERABLE")
#undef DUK_DEFPROP_CLEAR_ENUMERABLE
const duk_uint_t DUK_DEFPROP_CLEAR_ENUMERABLE =
#pragma pop_macro("DUK_DEFPROP_CLEAR_ENUMERABLE")
  DUK_DEFPROP_CLEAR_ENUMERABLE;

#pragma push_macro("DUK_DEFPROP_SET_CONFIGURABLE")
#undef DUK_DEFPROP_SET_CONFIGURABLE
const duk_uint_t DUK_DEFPROP_SET_CONFIGURABLE =
#pragma pop_macro("DUK_DEFPROP_SET_CONFIGURABLE")
  DUK_DEFPROP_SET_CONFIGURABLE;

#pragma push_macro("DUK_DEFPROP_CLEAR_CONFIGURABLE")
#undef DUK_DEFPROP_CLEAR_CONFIGURABLE
const duk_uint_t DUK_DEFPROP_CLEAR_CONFIGURABLE =
#pragma pop_macro("DUK_DEFPROP_CLEAR_CONFIGURABLE")
  DUK_DEFPROP_CLEAR_CONFIGURABLE;

#pragma push_macro("DUK_THREAD_NEW_GLOBAL_ENV")
#undef DUK_THREAD_NEW_GLOBAL_ENV
const duk_uint_t DUK_THREAD_NEW_GLOBAL_ENV =
#pragma pop_macro("DUK_THREAD_NEW_GLOBAL_ENV")
  DUK_THREAD_NEW_GLOBAL_ENV;

#pragma push_macro("DUK_ERR_NONE")
#undef DUK_ERR_NONE
const duk_errcode_t DUK_ERR_NONE =
#pragma pop_macro("DUK_ERR_NONE")
  DUK_ERR_NONE;

#pragma push_macro("DUK_ERR_ERROR")
#undef DUK_ERR_ERROR
const duk_errcode_t DUK_ERR_ERROR =
#pragma pop_macro("DUK_ERR_ERROR")
  DUK_ERR_ERROR;

#pragma push_macro("DUK_ERR_EVAL_ERROR")
#undef DUK_ERR_EVAL_ERROR
const duk_errcode_t DUK_ERR_EVAL_ERROR =
#pragma pop_macro("DUK_ERR_EVAL_ERROR")
  DUK_ERR_EVAL_ERROR;

#pragma push_macro("DUK_ERR_RANGE_ERROR")
#undef DUK_ERR_RANGE_ERROR
const duk_errcode_t DUK_ERR_RANGE_ERROR =
#pragma pop_macro("DUK_ERR_RANGE_ERROR")
  DUK_ERR_RANGE_ERROR;

#pragma push_macro("DUK_ERR_REFERENCE_ERROR")
#undef DUK_ERR_REFERENCE_ERROR
const duk_errcode_t DUK_ERR_REFERENCE_ERROR =
#pragma pop_macro("DUK_ERR_REFERENCE_ERROR")
  DUK_ERR_REFERENCE_ERROR;

#pragma push_macro("DUK_ERR_SYNTAX_ERROR")
#undef DUK_ERR_SYNTAX_ERROR
const duk_errcode_t DUK_ERR_SYNTAX_ERROR =
#pragma pop_macro("DUK_ERR_SYNTAX_ERROR")
  DUK_ERR_SYNTAX_ERROR;

#pragma push_macro("DUK_ERR_TYPE_ERROR")
#undef DUK_ERR_TYPE_ERROR
const duk_errcode_t DUK_ERR_TYPE_ERROR =
#pragma pop_macro("DUK_ERR_TYPE_ERROR")
  DUK_ERR_TYPE_ERROR;

#pragma push_macro("DUK_ERR_URI_ERROR")
#undef DUK_ERR_URI_ERROR
const duk_errcode_t DUK_ERR_URI_ERROR =
#pragma pop_macro("DUK_ERR_URI_ERROR")
  DUK_ERR_URI_ERROR;

#pragma push_macro("DUK_RET_ERROR")
#undef DUK_RET_ERROR
const duk_ret_t DUK_RET_ERROR =
#pragma pop_macro("DUK_RET_ERROR")
  DUK_RET_ERROR;

#pragma push_macro("DUK_RET_EVAL_ERROR")
#undef DUK_RET_EVAL_ERROR
const duk_ret_t DUK_RET_EVAL_ERROR =
#pragma pop_macro("DUK_RET_EVAL_ERROR")
  DUK_RET_EVAL_ERROR;

#pragma push_macro("DUK_RET_RANGE_ERROR")
#undef DUK_RET_RANGE_ERROR
const duk_ret_t DUK_RET_RANGE_ERROR =
#pragma pop_macro("DUK_RET_RANGE_ERROR")
  DUK_RET_RANGE_ERROR;

#pragma push_macro("DUK_RET_REFERENCE_ERROR")
#undef DUK_RET_REFERENCE_ERROR
const duk_ret_t DUK_RET_REFERENCE_ERROR =
#pragma pop_macro("DUK_RET_REFERENCE_ERROR")
  DUK_RET_REFERENCE_ERROR;

#pragma push_macro("DUK_RET_SYNTAX_ERROR")
#undef DUK_RET_SYNTAX_ERROR
const duk_ret_t DUK_RET_SYNTAX_ERROR =
#pragma pop_macro("DUK_RET_SYNTAX_ERROR")
  DUK_RET_SYNTAX_ERROR;

#pragma push_macro("DUK_RET_TYPE_ERROR")
#undef DUK_RET_TYPE_ERROR
const duk_ret_t DUK_RET_TYPE_ERROR =
#pragma pop_macro("DUK_RET_TYPE_ERROR")
  DUK_RET_TYPE_ERROR;

#pragma push_macro("DUK_RET_URI_ERROR")
#undef DUK_RET_URI_ERROR
const duk_ret_t DUK_RET_URI_ERROR =
#pragma pop_macro("DUK_RET_URI_ERROR")
  DUK_RET_URI_ERROR;

#pragma push_macro("DUK_EXEC_SUCCESS")
#undef DUK_EXEC_SUCCESS
const duk_int_t DUK_EXEC_SUCCESS =
#pragma pop_macro("DUK_EXEC_SUCCESS")
  DUK_EXEC_SUCCESS;

#pragma push_macro("DUK_EXEC_ERROR")
#undef DUK_EXEC_ERROR
const duk_int_t DUK_EXEC_ERROR =
#pragma pop_macro("DUK_EXEC_ERROR")
  DUK_EXEC_ERROR;

#pragma push_macro("DUK_LEVEL_DEBUG")
#undef DUK_LEVEL_DEBUG
const long DUK_LEVEL_DEBUG =
#pragma pop_macro("DUK_LEVEL_DEBUG")
  DUK_LEVEL_DEBUG;

#pragma push_macro("DUK_LEVEL_DDEBUG")
#undef DUK_LEVEL_DDEBUG
const long DUK_LEVEL_DDEBUG =
#pragma pop_macro("DUK_LEVEL_DDEBUG")
  DUK_LEVEL_DDEBUG;

#pragma push_macro("DUK_LEVEL_DDDEBUG")
#undef DUK_LEVEL_DDDEBUG
const long DUK_LEVEL_DDDEBUG =
#pragma pop_macro("DUK_LEVEL_DDDEBUG")
  DUK_LEVEL_DDDEBUG;

#pragma push_macro("DUK_LOG_TRACE")
#undef DUK_LOG_TRACE
const duk_int_t DUK_LOG_TRACE =
#pragma pop_macro("DUK_LOG_TRACE")
  DUK_LOG_TRACE;

#pragma push_macro("DUK_LOG_DEBUG")
#undef DUK_LOG_DEBUG
const duk_int_t DUK_LOG_DEBUG =
#pragma pop_macro("DUK_LOG_DEBUG")
  DUK_LOG_DEBUG;

#pragma push_macro("DUK_LOG_INFO")
#undef DUK_LOG_INFO
const duk_int_t DUK_LOG_INFO =
#pragma pop_macro("DUK_LOG_INFO")
  DUK_LOG_INFO;

#pragma push_macro("DUK_LOG_WARN")
#undef DUK_LOG_WARN
const duk_int_t DUK_LOG_WARN =
#pragma pop_macro("DUK_LOG_WARN")
  DUK_LOG_WARN;

#pragma push_macro("DUK_LOG_ERROR")
#undef DUK_LOG_ERROR
const duk_int_t DUK_LOG_ERROR =
#pragma pop_macro("DUK_LOG_ERROR")
  DUK_LOG_ERROR;

#pragma push_macro("DUK_LOG_FATAL")
#undef DUK_LOG_FATAL
const duk_int_t DUK_LOG_FATAL =
#pragma pop_macro("DUK_LOG_FATAL")
  DUK_LOG_FATAL;

#pragma push_macro("duk_create_heap_default")
#undef duk_create_heap_default
duk_context * duk_create_heap_default() {
#pragma pop_macro("duk_create_heap_default")
  return duk_create_heap_default();
}

#pragma push_macro("duk_xmove_top")
#undef duk_xmove_top
void duk_xmove_top(duk_context * to_ctx, duk_context * from_ctx, duk_idx_t count) {
#pragma pop_macro("duk_xmove_top")
  duk_xmove_top(to_ctx, from_ctx, count);
}

#pragma push_macro("duk_xcopy_top")
#undef duk_xcopy_top
void duk_xcopy_top(duk_context * to_ctx, duk_context * from_ctx, duk_idx_t count) {
#pragma pop_macro("duk_xcopy_top")
  duk_xcopy_top(to_ctx, from_ctx, count);
}

#pragma push_macro("duk_push_string_file")
#undef duk_push_string_file
const char * duk_push_string_file(duk_context * ctx, const char * path) {
#pragma pop_macro("duk_push_string_file")
  return duk_push_string_file(ctx, path);
}

#pragma push_macro("duk_push_thread")
#undef duk_push_thread
duk_idx_t duk_push_thread(duk_context * ctx) {
#pragma pop_macro("duk_push_thread")
  return duk_push_thread(ctx);
}

#pragma push_macro("duk_push_thread_new_globalenv")
#undef duk_push_thread_new_globalenv
duk_idx_t duk_push_thread_new_globalenv(duk_context * ctx) {
#pragma pop_macro("duk_push_thread_new_globalenv")
  return duk_push_thread_new_globalenv(ctx);
}

#pragma push_macro("duk_push_error_object")
#undef duk_push_error_object
duk_idx_t duk_push_error_object(duk_context * ctx, duk_errcode_t err_code, const char * fmt) {
#pragma pop_macro("duk_push_error_object")
  return duk_push_error_object(ctx, err_code, fmt);
}

#pragma push_macro("duk_push_buffer")
#undef duk_push_buffer
void * duk_push_buffer(duk_context * ctx, duk_size_t size, duk_bool_t dynamic) {
#pragma pop_macro("duk_push_buffer")
  return duk_push_buffer(ctx, size, dynamic);
}

#pragma push_macro("duk_push_fixed_buffer")
#undef duk_push_fixed_buffer
void * duk_push_fixed_buffer(duk_context * ctx, duk_size_t size) {
#pragma pop_macro("duk_push_fixed_buffer")
  return duk_push_fixed_buffer(ctx, size);
}

#pragma push_macro("duk_push_dynamic_buffer")
#undef duk_push_dynamic_buffer
void * duk_push_dynamic_buffer(duk_context * ctx, duk_size_t size) {
#pragma pop_macro("duk_push_dynamic_buffer")
  return duk_push_dynamic_buffer(ctx, size);
}

#pragma push_macro("duk_push_external_buffer")
#undef duk_push_external_buffer
void duk_push_external_buffer(duk_context * ctx) {
#pragma pop_macro("duk_push_external_buffer")
  duk_push_external_buffer(ctx);
}

#pragma push_macro("duk_is_callable")
#undef duk_is_callable
duk_bool_t duk_is_callable(duk_context * ctx, duk_idx_t index) {
#pragma pop_macro("duk_is_callable")
  return duk_is_callable(ctx, index);
}

#pragma push_macro("duk_is_primitive")
#undef duk_is_primitive
duk_bool_t duk_is_primitive(duk_context * ctx, duk_idx_t index) {
#pragma pop_macro("duk_is_primitive")
  return duk_is_primitive(ctx, index);
}

#pragma push_macro("duk_is_object_coercible")
#undef duk_is_object_coercible
duk_bool_t duk_is_object_coercible(duk_context * ctx, duk_idx_t index) {
#pragma pop_macro("duk_is_object_coercible")
  return duk_is_object_coercible(ctx, index);
}

#pragma push_macro("duk_is_error")
#undef duk_is_error
duk_bool_t duk_is_error(duk_context * ctx, duk_idx_t index) {
#pragma pop_macro("duk_is_error")
  return duk_is_error(ctx, index);
}

#pragma push_macro("duk_is_eval_error")
#undef duk_is_eval_error
duk_bool_t duk_is_eval_error(duk_context * ctx, duk_idx_t index) {
#pragma pop_macro("duk_is_eval_error")
  return duk_is_eval_error(ctx, index);
}

#pragma push_macro("duk_is_range_error")
#undef duk_is_range_error
duk_bool_t duk_is_range_error(duk_context * ctx, duk_idx_t index) {
#pragma pop_macro("duk_is_range_error")
  return duk_is_range_error(ctx, index);
}

#pragma push_macro("duk_is_reference_error")
#undef duk_is_reference_error
duk_bool_t duk_is_reference_error(duk_context * ctx, duk_idx_t index) {
#pragma pop_macro("duk_is_reference_error")
  return duk_is_reference_error(ctx, index);
}

#pragma push_macro("duk_is_syntax_error")
#undef duk_is_syntax_error
duk_bool_t duk_is_syntax_error(duk_context * ctx, duk_idx_t index) {
#pragma pop_macro("duk_is_syntax_error")
  return duk_is_syntax_error(ctx, index);
}

#pragma push_macro("duk_is_type_error")
#undef duk_is_type_error
duk_bool_t duk_is_type_error(duk_context * ctx, duk_idx_t index) {
#pragma pop_macro("duk_is_type_error")
  return duk_is_type_error(ctx, index);
}

#pragma push_macro("duk_is_uri_error")
#undef duk_is_uri_error
duk_bool_t duk_is_uri_error(duk_context * ctx, duk_idx_t index) {
#pragma pop_macro("duk_is_uri_error")
  return duk_is_uri_error(ctx, index);
}

#pragma push_macro("duk_require_type_mask")
#undef duk_require_type_mask
void duk_require_type_mask(duk_context * ctx, duk_idx_t index, duk_uint_t mask) {
#pragma pop_macro("duk_require_type_mask")
  duk_require_type_mask(ctx, index, mask);
}

#pragma push_macro("duk_require_callable")
#undef duk_require_callable
void duk_require_callable(duk_context * ctx, duk_idx_t index) {
#pragma pop_macro("duk_require_callable")
  duk_require_callable(ctx, index);
}

#pragma push_macro("duk_require_object_coercible")
#undef duk_require_object_coercible
void duk_require_object_coercible(duk_context * ctx, duk_idx_t index) {
#pragma pop_macro("duk_require_object_coercible")
  duk_require_object_coercible(ctx, index);
}

#pragma push_macro("duk_to_buffer")
#undef duk_to_buffer
void * duk_to_buffer(duk_context * ctx, duk_idx_t index, duk_size_t * out_size) {
#pragma pop_macro("duk_to_buffer")
  return duk_to_buffer(ctx, index, out_size);
}

#pragma push_macro("duk_to_fixed_buffer")
#undef duk_to_fixed_buffer
void * duk_to_fixed_buffer(duk_context * ctx, duk_idx_t index, duk_size_t * out_size) {
#pragma pop_macro("duk_to_fixed_buffer")
  return duk_to_fixed_buffer(ctx, index, out_size);
}

#pragma push_macro("duk_to_dynamic_buffer")
#undef duk_to_dynamic_buffer
void * duk_to_dynamic_buffer(duk_context * ctx, duk_idx_t index, duk_size_t * out_size) {
#pragma pop_macro("duk_to_dynamic_buffer")
  return duk_to_dynamic_buffer(ctx, index, out_size);
}

#pragma push_macro("duk_safe_to_string")
#undef duk_safe_to_string
const char * duk_safe_to_string(duk_context * ctx, duk_idx_t index) {
#pragma pop_macro("duk_safe_to_string")
  return duk_safe_to_string(ctx, index);
}

#pragma push_macro("duk_eval")
#undef duk_eval
void duk_eval(duk_context * ctx) {
#pragma pop_macro("duk_eval")
  duk_eval(ctx);
}

#pragma push_macro("duk_eval_noresult")
#undef duk_eval_noresult
void duk_eval_noresult(duk_context * ctx) {
#pragma pop_macro("duk_eval_noresult")
  duk_eval_noresult(ctx);
}

#pragma push_macro("duk_peval")
#undef duk_peval
duk_int_t duk_peval(duk_context * ctx) {
#pragma pop_macro("duk_peval")
  return duk_peval(ctx);
}

#pragma push_macro("duk_peval_noresult")
#undef duk_peval_noresult
duk_int_t duk_peval_noresult(duk_context * ctx) {
#pragma pop_macro("duk_peval_noresult")
  return duk_peval_noresult(ctx);
}

#pragma push_macro("duk_compile")
#undef duk_compile
void duk_compile(duk_context * ctx, duk_uint_t flags) {
#pragma pop_macro("duk_compile")
  duk_compile(ctx, flags);
}

#pragma push_macro("duk_pcompile")
#undef duk_pcompile
duk_int_t duk_pcompile(duk_context * ctx, duk_uint_t flags) {
#pragma pop_macro("duk_pcompile")
  return duk_pcompile(ctx, flags);
}

#pragma push_macro("duk_eval_string")
#undef duk_eval_string
void duk_eval_string(duk_context * ctx, const char * src) {
#pragma pop_macro("duk_eval_string")
  duk_eval_string(ctx, src);
}

#pragma push_macro("duk_eval_string_noresult")
#undef duk_eval_string_noresult
void duk_eval_string_noresult(duk_context * ctx, const char * src) {
#pragma pop_macro("duk_eval_string_noresult")
  duk_eval_string_noresult(ctx, src);
}

#pragma push_macro("duk_peval_string")
#undef duk_peval_string
duk_int_t duk_peval_string(duk_context * ctx, const char * src) {
#pragma pop_macro("duk_peval_string")
  return duk_peval_string(ctx, src);
}

#pragma push_macro("duk_peval_string_noresult")
#undef duk_peval_string_noresult
duk_int_t duk_peval_string_noresult(duk_context * ctx, const char * src) {
#pragma pop_macro("duk_peval_string_noresult")
  return duk_peval_string_noresult(ctx, src);
}

#pragma push_macro("duk_compile_string")
#undef duk_compile_string
void duk_compile_string(duk_context * ctx, duk_uint_t flags, const char * src) {
#pragma pop_macro("duk_compile_string")
  duk_compile_string(ctx, flags, src);
}

#pragma push_macro("duk_compile_string_filename")
#undef duk_compile_string_filename
void duk_compile_string_filename(duk_context * ctx, duk_uint_t flags, const char * src) {
#pragma pop_macro("duk_compile_string_filename")
  duk_compile_string_filename(ctx, flags, src);
}

#pragma push_macro("duk_pcompile_string")
#undef duk_pcompile_string
duk_int_t duk_pcompile_string(duk_context * ctx, duk_uint_t flags, const char * src) {
#pragma pop_macro("duk_pcompile_string")
  return duk_pcompile_string(ctx, flags, src);
}

#pragma push_macro("duk_pcompile_string_filename")
#undef duk_pcompile_string_filename
duk_int_t duk_pcompile_string_filename(duk_context * ctx, duk_uint_t flags, const char * src) {
#pragma pop_macro("duk_pcompile_string_filename")
  return duk_pcompile_string_filename(ctx, flags, src);
}

#pragma push_macro("duk_eval_lstring")
#undef duk_eval_lstring
void duk_eval_lstring(duk_context * ctx, const char * buf, duk_size_t len) {
#pragma pop_macro("duk_eval_lstring")
  duk_eval_lstring(ctx, buf, len);
}

#pragma push_macro("duk_eval_lstring_noresult")
#undef duk_eval_lstring_noresult
void duk_eval_lstring_noresult(duk_context * ctx, const char * buf, duk_size_t len) {
#pragma pop_macro("duk_eval_lstring_noresult")
  duk_eval_lstring_noresult(ctx, buf, len);
}

#pragma push_macro("duk_peval_lstring")
#undef duk_peval_lstring
duk_int_t duk_peval_lstring(duk_context * ctx, const char * buf, duk_size_t len) {
#pragma pop_macro("duk_peval_lstring")
  return duk_peval_lstring(ctx, buf, len);
}

#pragma push_macro("duk_peval_lstring_noresult")
#undef duk_peval_lstring_noresult
duk_int_t duk_peval_lstring_noresult(duk_context * ctx, const char * buf, duk_size_t len) {
#pragma pop_macro("duk_peval_lstring_noresult")
  return duk_peval_lstring_noresult(ctx, buf, len);
}

#pragma push_macro("duk_compile_lstring")
#undef duk_compile_lstring
void duk_compile_lstring(duk_context * ctx, duk_uint_t flags, const char * buf, duk_size_t len) {
#pragma pop_macro("duk_compile_lstring")
  duk_compile_lstring(ctx, flags, buf, len);
}

#pragma push_macro("duk_compile_lstring_filename")
#undef duk_compile_lstring_filename
void duk_compile_lstring_filename(duk_context * ctx, duk_uint_t flags, const char * buf, duk_size_t len) {
#pragma pop_macro("duk_compile_lstring_filename")
  duk_compile_lstring_filename(ctx, flags, buf, len);
}

#pragma push_macro("duk_pcompile_lstring")
#undef duk_pcompile_lstring
duk_int_t duk_pcompile_lstring(duk_context * ctx, duk_uint_t flags, const char * buf, duk_size_t len) {
#pragma pop_macro("duk_pcompile_lstring")
  return duk_pcompile_lstring(ctx, flags, buf, len);
}

#pragma push_macro("duk_pcompile_lstring_filename")
#undef duk_pcompile_lstring_filename
duk_int_t duk_pcompile_lstring_filename(duk_context * ctx, duk_uint_t flags, const char * buf, duk_size_t len) {
#pragma pop_macro("duk_pcompile_lstring_filename")
  return duk_pcompile_lstring_filename(ctx, flags, buf, len);
}

#pragma push_macro("duk_eval_file")
#undef duk_eval_file
void duk_eval_file(duk_context * ctx, const char * path) {
#pragma pop_macro("duk_eval_file")
  duk_eval_file(ctx, path);
}

#pragma push_macro("duk_eval_file_noresult")
#undef duk_eval_file_noresult
void duk_eval_file_noresult(duk_context * ctx, const char * path) {
#pragma pop_macro("duk_eval_file_noresult")
  duk_eval_file_noresult(ctx, path);
}

#pragma push_macro("duk_peval_file")
#undef duk_peval_file
duk_int_t duk_peval_file(duk_context * ctx, const char * path) {
#pragma pop_macro("duk_peval_file")
  return duk_peval_file(ctx, path);
}

#pragma push_macro("duk_peval_file_noresult")
#undef duk_peval_file_noresult
duk_int_t duk_peval_file_noresult(duk_context * ctx, const char * path) {
#pragma pop_macro("duk_peval_file_noresult")
  return duk_peval_file_noresult(ctx, path);
}

#pragma push_macro("duk_compile_file")
#undef duk_compile_file
void duk_compile_file(duk_context * ctx, duk_uint_t flags, const char * path) {
#pragma pop_macro("duk_compile_file")
  duk_compile_file(ctx, flags, path);
}

#pragma push_macro("duk_pcompile_file")
#undef duk_pcompile_file
duk_int_t duk_pcompile_file(duk_context * ctx, duk_uint_t flags, const char * path) {
#pragma pop_macro("duk_pcompile_file")
  return duk_pcompile_file(ctx, flags, path);
}

#pragma push_macro("duk_dump_context_stdout")
#undef duk_dump_context_stdout
void duk_dump_context_stdout(duk_context * ctx) {
#pragma pop_macro("duk_dump_context_stdout")
  duk_dump_context_stdout(ctx);
}

#pragma push_macro("duk_dump_context_stderr")
#undef duk_dump_context_stderr
void duk_dump_context_stderr(duk_context * ctx) {
#pragma pop_macro("duk_dump_context_stderr")
  duk_dump_context_stderr(ctx);
}
