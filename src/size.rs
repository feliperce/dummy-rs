use anyhow::{Context, Result};

pub fn parse_size(size_str: &str) -> Result<u64> {
    let size_str = size_str.trim().to_uppercase();

    if size_str.ends_with('K') {
        let num = size_str[..size_str.len() - 1].parse::<u64>()
            .context("Invalid size format")?;
        Ok(num * 1024)
    } else if size_str.ends_with('M') {
        let num = size_str[..size_str.len() - 1].parse::<u64>()
            .context("Invalid size format")?;
        Ok(num * 1024 * 1024)
    } else if size_str.ends_with('G') {
        let num = size_str[..size_str.len() - 1].parse::<u64>()
            .context("Invalid size format")?;
        Ok(num * 1024 * 1024 * 1024)
    } else {
        size_str.parse::<u64>().context("Invalid size format")
    }
}