extern crate gcc;

fn main() {
    let mut config = gcc::Config::new();
    config.file("duktape/src/duktape.c");
    config.file("src/wrapper.c");

    if cfg!(debug) {
        config.define("DUK_USE_DEBUG", None);
        config.define("DUK_USE_DEBUG_WRITE", Some("__duktape_sys_debug_write"));
    }

    config.compile("libduktape.a");
}
