# duk [![Build Status](https://travis-ci.org/dflemstr/duk.svg?branch=master)](https://travis-ci.org/dflemstr/duk)
A high-level wrapper around the [Duktape][1] Javascript/EcmaScript
interpreter.

Currently, the focus is around supporting "extension"/"plug-in"
use cases, so the primary supported functionality is:

  * Loading code.
  * Calling functions and getting their result.

Other use-cases (like exposing Rust functions to JS) are not yet
implemented.

[1]: http://duktape.org/
