extern crate gcc;

fn main() {
    let mut config = gcc::Config::new();

    config.define("DUK_OPT_DEBUG", None);
    config.define("DUK_USE_DEBUG_WRITE", Some("__duktape_sys_debug_write"));
    config.define("DUK_OPT_DPRINT", None);
    config.define("DUK_OPT_DDPRINT", None);

    config.file("duktape/src/duktape.c");
    config.file("src/wrapper.c");

    config.compile("libduktape.a");
}
