use rayon::prelude::*;
use regex::Regex;

pub fn search_content<'a>(content: &'a str, pattern: &str) -> Result<Vec<(usize, &'a str)>, regex::Error> {
    let re = Regex::new(pattern)?;

    let matches: Vec<(usize, &str)> = content
        .lines()
        .enumerate()
        .par_bridge()
        .filter_map(|(idx, line)| {
            if re.is_match(line) {
                Some((idx + 1, line))
            } else {
                None
            }
        })
        .collect();

    Ok(matches)
}
