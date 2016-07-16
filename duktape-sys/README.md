# duktape-sys

An auto-generated wrapper around the [Duktape][1] library.

The API of this wrapper is not stable, and currently exposes
transient library APIs.

[1]: http://duktape.org/

## Generated code

This code base contains auto-generated C and Rust wrapper code.  The
code generator is shipped as a cargo example.  To re-generate the
wrapper code, run:

```bash
cargo run --example gen-wrapper
```
