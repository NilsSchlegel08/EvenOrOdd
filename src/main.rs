use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufWriter};
use std::time::{Instant};

const INT_LIMIT: i32 = i32::MAX;

fn main() -> io::Result<()> {
    let start_time = Instant::now();
    let file = File::create("main.c")?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "#include <stdbool.h>")?;
    writeln!(writer, "#include <stdio.h>")?;
    writeln!(writer, "\nbool is_odd(int n) {{")?;

    for i in -INT_LIMIT..=INT_LIMIT {
        if i == -INT_LIMIT {
            writeln!(writer, "    if (n == {}) return {};", i, if i % 2 != 0 { "true" } else { "false" })?;
        } else {
            writeln!(writer, "    else if (n == {}) return {};", i, if i % 2 != 0 { "true" } else { "false" })?;
        }

        if i % 10_000_000 == 0 {
            println!("Generated elif for number: {}", i);
        }
    }

    writeln!(writer, "    return false;")?;
    writeln!(writer, "}}")?;

    writer.flush()?;

    let duration = start_time.elapsed();
    let file_size = std::fs::metadata("main.c")?.len();

    println!("\nmain.c generated successfully.");
    println!("File size: {} bytes", file_size);
    println!("Total time: {:.2?} seconds", duration);

    println!("\nPress Enter to exit...");
    let _ = io::stdin().read_line(&mut String::new());

    Ok(())
}
