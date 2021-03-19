//! [reelpath](https://github.com/nickgerace/reelpath) finds the absolute path for a given file or directory.
//! This library exists for the `reelpath` CLI and has not been tested for other use cases.

use std::fs;
use std::io;

/// This function is the primary driver for the `reelpath` CLI. In errorless scenarios, this
/// function will print its results to STDOUT. Otherwise, it will return any error encountered.
pub fn run(s: &[String]) -> io::Result<()> {
    let help = || {
        println!(
            "reelpath {}
https://github.com/nickgerace/reelpath

Find the absolute path of a given file or directory.
To evaluate more than one path, an additional argument per path.
Wildcards can be used in a single argument.

USAGE:
    reelpath [path]...",
            option_env!("CARGO_PKG_VERSION").unwrap_or("v?")
        );
    };

    // I'm not super happy with this code, but it gets the job done.
    let mut print_help = false;
    for i in s {
        match i.as_str() {
            "-h" | "-help" | "--help" => {
                print_help = true;
                break;
            }
            _ => continue,
        }
    }
    if s.is_empty() || print_help {
        help();
    } else {
        for i in s {
            match fs::canonicalize(i) {
                Ok(o) => println!("{}", o.display()),
                Err(e) => return Err(e),
            }
        }
    }
    Ok(())
}
