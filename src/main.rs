use std::env;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(s) => reelpath::run(s)?,
        None => reelpath::help(),
    }
    Ok(())
}
