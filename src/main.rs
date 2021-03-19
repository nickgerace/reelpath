use std::env;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    reelpath::run(&args[1..args.len()])?;
    Ok(())
}
