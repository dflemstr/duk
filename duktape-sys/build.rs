extern crate gcc;

fn main() {
    let mut config = gcc::Config::new();

    let wire_debug = if cfg!(feature = "spam") {
        config.define("DUK_OPT_DEBUG_LEVEL", Some("DUK_LEVEL_DDDEBUG"));
        true
    } else if cfg!(feature = "trace") {
        config.define("DUK_OPT_DEBUG_LEVEL", Some("DUK_LEVEL_DDEBUG"));
        true
    } else if cfg!(feature = "debug") {
        config.define("DUK_OPT_DEBUG_LEVEL", Some("DUK_LEVEL_DEBUG"));
        true
    } else {
        false
    };

    if wire_debug {
        config.define("DUK_OPT_DEBUG", None);
        config.define("DUK_OPT_DEBUG_WRITE", Some("__duktape_sys_debug_write"));
    }

    config.define("DUK_LOGGING_FLUSH", None);

    config.include("duktape/src");
    config.include("duktape/extras/logging");
    config.include("duktape/extras/module-node");
    config.flag("-std=c99");
    config.file("duktape/src/duktape.c");
    config.file("duktape/extras/logging/duk_logging.c");
    config.file("duktape/extras/module-node/duk_module_node.c");
    // Add FileIo.readfile
    config.file("duktape/examples/eventloop/fileio.c");
    // Add print
    config.file("duktape/extras/print-alert/duk_print_alert.c");

    config.file("src/wrapper.c");

    config.compile("libduktape.a");
}
