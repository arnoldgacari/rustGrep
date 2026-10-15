use memmap2::Mmap;
use std::fs::File;
use std::io;
use std::path::Path;

pub fn map_file<P: AsRef<Path>>(path: P) -> io::Result<(File, Mmap)> {
    let file = File::open(path)?;
    // SAFETY: Memory mapping a file relies on the file not being concurrently 
    // truncated externally, which is standard for read-only log processing tools.
    let mmap = unsafe { Mmap::map(&file)? };
    Ok((file, mmap))
}