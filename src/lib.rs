//! [reelpath](https://github.com/nickgerace/reelpath) finds the absolute path for a given file or directory.
//! This library exists for the `reelpath` CLI and has not been tested for other use cases.

use std::fs;
use std::io;

/// This function prints general usage information to STDOUT for the `reelpath` CLI.
pub fn help() {
    let version = option_env!("CARGO_PKG_VERSION").unwrap_or("v?");
    println!(
        "reelpath {}
https://github.com/nickgerace/reelpath

Find the absolute path of a given file or directory.

USAGE:
    reelpath [path]",
        version
    );
}

/// This function is the primary driver for the `reelpath` CLI. In errorless scenarios, this
/// function will print its results to STDOUT. Otherwise, it will return any error encountered.
pub fn run(s: &str) -> io::Result<()> {
    match s {
        "-h" | "-help" | "--help" => help(),
        _ => match fs::canonicalize(s) {
            Ok(o) => println!("{}", o.display()),
            Err(e) => return Err(e),
        },
    }
    Ok(())
}
