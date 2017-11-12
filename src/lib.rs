// Copyright 2015 Keegan McAllister.
// Copyright 2016 Corey Farwell.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// See `LICENSE` in this repository.

use std::intrinsics::abort;
use std::io::{self, Read};
use std::panic::{self, UnwindSafe};

/// Utility that reads a `Vec` of bytes from standard input (stdin)
/// and passes it to `closure`. All panics that occur within
/// `closure` will be treated as aborts. This is done so that
/// AFL considers a panic to be a crash.
///
/// # Examples
///
/// ```no_run
/// extern crate afl;
/// # struct Image;
/// # impl Image {
/// #     fn parse<R: ::std::io::Read>(_: R) {}
/// # }
///
/// fn main() {
///     afl::handle_bytes(|read| {
///         Image::parse(read)
///     })
/// }
/// ```
pub fn handle_bytes<F>(closure: F)
    where F: FnOnce(Vec<u8>) + UnwindSafe
{
    let mut input = vec![];
    let result = io::stdin().read_to_end(&mut input);
    if result.is_err() {
        return;
    }

    let result = panic::catch_unwind(|| {
        closure(input);
    });
    if result.is_err() {
        unsafe {
            abort();
        }
    }
}

/// Utility that reads a `String` from standard input (stdin) and
/// passes it to `closure`. If a `String` cannot be constructed from
/// the data provided by standard input, `closure` will _not_ be
/// called. All panics that occur within `closure` will be treated as
/// aborts. This is done so that AFL considers a panic to be a crash.
///
/// # Examples
///
/// ```no_run
/// extern crate afl;
/// # struct Url;
/// # impl Url {
/// #     fn parse(_: &str) {}
/// # }
///
/// fn main() {
///     afl::handle_string(|string| {
///         Url::parse(&string)
///     })
/// }
/// ```
pub fn handle_string<F>(closure: F)
    where F: FnOnce(String) + UnwindSafe
{
    let mut input = String::new();
    let result = io::stdin().read_to_string(&mut input);
    if result.is_err() {
        return;
    }
    let result = panic::catch_unwind(|| {
        closure(input);
    });
    if result.is_err() {
        unsafe {
            abort();
        }
    }
}

/// Utility that passes `Stdin` to `closure` for use with functions
/// that expect a structure that implements `Read`. All panics that
/// occur within `closure` will be treated as aborts. This is done
/// so that AFL considers a panic to be a crash.
///
/// # Examples
///
/// ```no_run
/// extern crate afl;
/// # struct Image;
/// # impl Image {
/// #     fn parse(_: &[u8]) {}
/// # }
///
/// fn main() {
///     afl::handle_read(|bytes| {
///         Image::parse(&bytes)
///     })
/// }
/// ```
pub fn handle_read<F>(closure: F)
    where F: FnOnce(io::Stdin) + UnwindSafe
{
    let result = panic::catch_unwind(|| {
        closure(io::stdin());
    });
    if result.is_err() {
        unsafe {
            abort();
        }
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use std::process::{Command, Stdio};
    use std::thread;
    use std::time;

    extern crate libc;
    extern crate tempdir;

    fn target_path() -> PathBuf {
        if PathBuf::from("../target/debug/cargo-afl-fuzz").exists() {
            PathBuf::from("../target/debug/")
        } else if PathBuf::from("target/debug/cargo-afl-fuzz").exists() {
            PathBuf::from("target/debug/")
        } else {
            panic!("Could not find cargo-afl-fuzz!");
        }
    }

    #[test]
    fn test_cargo_afl_fuzz() {
        let temp_dir = tempdir::TempDir::new("aflrs").expect("Could not create temporary directory");
        let temp_dir_path = temp_dir.path();
        let mut child = Command::new(target_path().join("cargo-afl-fuzz"))
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .arg("-i")
            .arg(".")
            .arg("-o")
            .arg(temp_dir_path)
            .arg(target_path().join("examples").join("hello"))
            .spawn()
            .expect("Could not run cargo-afl-fuzz");
        thread::sleep(time::Duration::from_secs(7));
        child.kill().unwrap();
        assert!(temp_dir_path.join("fuzzer_stats").is_file());
    }
}
