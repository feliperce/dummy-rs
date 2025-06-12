use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about = "Create dummy files of specified size")]
pub struct Args {
    #[arg(help = "Name of the file to create")]
    pub filename: PathBuf,

    #[arg(help = "Size of the file (e.g., 1024, 1K, 2M, 3G)")]
    pub size: String,

    #[arg(long, help = "Fill the file with random data instead of zeros")]
    pub fill: bool,
}

impl Args {
    pub fn parse() -> Self {
        <Args as Parser>::parse()
    }
}