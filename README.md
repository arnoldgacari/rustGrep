# rustgrep

A high-performance, concurrent log analysis and search tool built in Rust. Designed for speed, `rustgrep` leverages zero-copy memory mapping and data parallelism to rip through large log files.

## Features

- **Zero-Copy Memory Mapping**: Uses `memmap2` to map files directly into memory, bypassing traditional buffer overhead.
- **Data Parallelism**: Powered by `rayon` to process lines concurrently across multiple CPU threads.
- **Robust Pattern Matching**: Utilizes `regex` for flexible and accurate query matching.
- **Modern CLI**: Built with `clap` for clean, expressive command-line argument parsing.

## Installation & Usage

1. **Clone the repository:**
   ```bash
   git clone 
   cd rustgrep

2. Build the release binary:
cargo build --release

3. Run a search
cargo run --release -- --pattern "ERROR" --path path/to/your/logfile.log
