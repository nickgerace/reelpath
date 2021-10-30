use std::{env, fs, process};

const HELP: &str = "
https://github.com/nickgerace/reelpath

Find the absolute path of a given file or directory.
To evaluate more than one path, an additional argument per path.
Wildcards can be used in a single argument.

USAGE:
    reelpath [path]...";

fn contains_help_flag_and_is_not_empty(args: &[String]) -> bool {
    for arg in args {
        if arg == "-h" || arg == "--help" {
            return true;
        }
    }
    false
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match contains_help_flag_and_is_not_empty(&args) {
        true => println!("reelpath {}{}", env!("CARGO_PKG_VERSION"), HELP),
        false => {
            let mut errors: usize = 0;
            for arg in &args {
                match fs::canonicalize(arg) {
                    Ok(o) => println!("{}", o.display()),
                    Err(_) => errors += 1,
                }
            }
            if errors > 0 {
                eprintln!("could not canonicalize {} targets provided", errors);
                process::exit(1);
            }
        }
    }
}
