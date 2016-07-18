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

    config.file("duktape/src/duktape.c");
    config.file("src/wrapper.c");

    config.compile("libduktape.a");
}
