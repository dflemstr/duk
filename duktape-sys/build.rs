extern crate gcc;

fn main() {
    gcc::compile_library("libduktape.a", &["duktape/src/duktape.c", "src/wrapper.c"]);
}
