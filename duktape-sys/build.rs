extern crate gcc;

fn main() {
    let mut config = gcc::Config::new();
    config.file("duktape/src/duktape.c");
    config.file("src/wrapper.c");

    if cfg!(debug) {
        config.define("DUK_USE_DEBUG", Some("1"));
        config.define("DUK_USE_DEBUG_WRITE", Some("__duktape_sys_debug_write"));
        config.define("DUK_USE_DDPRINT", Some("1"));
        config.define("DUK_USE_DDDPRINT", Some("1"));
    }

    config.compile("libduktape.a");
}
