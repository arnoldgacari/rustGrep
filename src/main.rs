mod args;
mod reader;
mod search;

use args::Args;
use clap::Parser;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // 1. Map the file into memory
    let (_file, mmap) = reader::map_file(&args.path)?;
    
    // 2. Convert bytes to a UTF-8 string view (lossy to safely handle malformed log lines)
    let content = String::from_utf8_lossy(&mmap);

    // 3. Run the concurrent search
    let matches = search::search_content(&content, &args.pattern)?;

    // 4. Output results efficiently using a locked stdout buffer
    let stdout = io::stdout();
    let mut lock = stdout.lock();

    for (line_num, line) in matches {
        writeln!(lock, "{}: {}", line_num, line)?;
    }

    Ok(())
}