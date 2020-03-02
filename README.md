<h1 align="center">
  <a href="https://github.com/frewsxcv/afl.rs/issues/66"><img src="etc/logo.gif" alt="afl.rs logo"></a>
  <br>
  afl.rs
</h1>

<h4 align="center">Fuzzing <a href="https://www.rust-lang.org">Rust</a> code with <a href="http://lcamtuf.coredump.cx/afl/">american fuzzy lop (AFL)</a></h4>

## What is it?

[Fuzz testing][] is a software testing technique used to find security and stability issues by providing pseudo-random data as input to the software. [American fuzzy lop][american-fuzzy-lop] is a popular, effective, and modern fuzz testing tool. This library, afl.rs, allows one to run AFL on code written in [the Rust programming language][rust].

## Documentation

Documentation can be found in the [Rust Fuzz Book](https://rust-fuzz.github.io/book/afl.html).

## What does it look like?

<img src="etc/screencap.gif" width="563" height="368" alt="Screen recording of afl">

Screen recording of AFL running on Rust code.

[conditional compilation]: https://doc.rust-lang.org/reference.html#conditional-compilation
[Cargo feature]: http://doc.crates.io/manifest.html#the-[features]-section
[example-defer]: https://github.com/frewsxcv/afl.rs/blob/master/examples/deferred-init.rs
[LLVM pass]: https://github.com/frewsxcv/afl.rs/blob/master/plugin/src/afl-llvm-pass.o.cc
[example]: https://github.com/frewsxcv/afl.rs/blob/master/afl/examples/hello.rs
[Cargo]: http://doc.crates.io/
[unresolved issue]: https://github.com/frewsxcv/afl.rs/issues/11
[fuzz testing]: https://en.wikipedia.org/wiki/Fuzz_testing
[rustup]: https://rustup.rs/
[american-fuzzy-lop]: http://lcamtuf.coredump.cx/afl/
[rust]: https://www.rust-lang.org

## `lazy_static` variables

`lazy_static` variables present problems for AFL's persistent mode, which afl.rs uses. Such variables can cause AFL to give incorrectly low stability reports, or fail to report timeouts, for example.

To address such problems, rust-fuzz provides a ["resettable" version](https://github.com/rust-fuzz/resettable-lazy-static.rs) of `lazy_static`. To use it, make the following two changes to your target's `Cargo.toml` file.

1. Add a `[patch.crates-io]` section and overide the `lazy_static` dependency with the rust-fuzz version:
    ```toml
    [patch.crates-io]
    lazy_static = { git = "https://github.com/rust-fuzz/resettable-lazy-static.rs" }

    ```
2. Enable the `reset_lazy_staic` feature on afl.rs:
    ```toml
    [dependencies]
    afl = { version = "0.6.0", features = ["reset_lazy_static"] }
    ```
