extern crate gcc;

fn main() {
    let mut config = gcc::Config::new();

    if cfg!(feature = "debug") {
        config.define("DUK_OPT_DEBUG", None);
        config.define("DUK_OPT_DPRINT", None);
        if cfg!(feature = "trace") {
            config.define("DUK_OPT_DDPRINT", None);

            if cfg!(feature = "spam") {
                config.define("DUK_OPT_DDDPRINT", None);
            }
        }
    }

    config.file("duktape/src/duktape.c");
    config.file("src/wrapper.c");

    config.compile("libduktape.a");
}
