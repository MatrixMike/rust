// Rust 2018

use std::fs::File;

fn main() -> Result<(), std::io::Error> {
    let f = File::open("bar.txt")?;

    Ok(())
}

