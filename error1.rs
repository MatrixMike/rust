// Rust 2018

use std::fs::File;

fn main() -> Result<(), std::io::Error> {
	// Rust's error handling revolves around returning Result<T, E> and using ? to propagate errors. 
    let _f = File::open("bar.txt")?;    // recompile with f 
// https://doc.rust-lang.org/edition-guide/rust-2018/error-handling-and-panics/question-mark-in-main-and-tests.html
    Ok(())
}

