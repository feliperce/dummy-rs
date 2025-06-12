use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;

use anyhow::{Context, Result};
use indicatif::ProgressBar;
use rand::Rng;

use crate::format::{format_duration, format_size};

pub fn create_dummy_file(filename: &PathBuf, size: u64, fill: bool) -> Result<()> {
    let file = File::create(filename)
        .context("Failed to create file")?;

    let start_time = Instant::now();

    let pb = crate::progress::create_progress_bar(size);

    if fill {
        fill_with_random_data(file, size, &pb)
            .context("Failed to fill file with random data")?;
    } else {
        fill_with_zeros(file, size, &pb)
            .context("Failed to fill file with zeros")?;
    }

    let elapsed = start_time.elapsed();

    pb.finish_with_message(format!("Done in {}", format_duration(elapsed)));

    println!("\nFile size: {}", format_size(size));
    println!("Time elapsed: {}", format_duration(elapsed));

    if size > 0 {
        let speed = (size as f64) / elapsed.as_secs_f64() / 1_048_576.0;
        println!("Write speed: {:.2} MB/s", speed);
    }

    Ok(())
}

fn fill_with_zeros(mut file: File, size: u64, pb: &ProgressBar) -> Result<()> {
    const BUFFER_SIZE: usize = 8192;
    let buffer = vec![0u8; BUFFER_SIZE];

    let mut remaining = size;

    while remaining > 0 {
        let to_write = std::cmp::min(remaining, BUFFER_SIZE as u64);
        file.write_all(&buffer[..to_write as usize])
            .context("Failed to write zeros to file")?;

        remaining -= to_write;
        pb.inc(to_write);
    }

    Ok(())
}

fn fill_with_random_data(mut file: File, size: u64, pb: &ProgressBar) -> Result<()> {
    const BUFFER_SIZE: usize = 8192;
    let mut buffer = vec![0u8; BUFFER_SIZE];
    let mut rng = rand::thread_rng();

    let mut remaining = size;

    while remaining > 0 {
        let to_write = std::cmp::min(remaining, BUFFER_SIZE as u64) as usize;

        // Fill buffer with random data
        rng.fill(&mut buffer[..to_write]);

        file.write_all(&buffer[..to_write])
            .context("Failed to write random data to file")?;

        remaining -= to_write as u64;
        pb.inc(to_write as u64);
    }

    Ok(())
}