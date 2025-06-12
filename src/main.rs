use anyhow::{Context, Result};

use dummy_rs::args::Args;
use dummy_rs::file_ops::create_dummy_file;
use dummy_rs::format::format_size;
use dummy_rs::size::parse_size;

fn main() -> Result<()> {
    let args = Args::parse();
    let size = parse_size(&args.size)?;

    println!("Creating file: {}", args.filename.display());
    println!("Size: {}", format_size(size));
    println!("Fill with random data: {}", args.fill);

    create_dummy_file(&args.filename, size, args.fill)
        .context("Failed to create dummy file")?;

    println!("File created successfully!");
    Ok(())
}
