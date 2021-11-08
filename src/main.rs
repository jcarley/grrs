use anyhow::{Context, Result};
use std::fs::File;
use std::io::stdout;
use std::io::BufReader;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::from_args();

    anyhow::ensure!(
        !args.pattern.is_empty(),
        "Missing required parameter 'pattern'"
    );

    anyhow::ensure!(
        !args.path.to_str().unwrap().is_empty(),
        "Missing required parameter 'path'"
    );

    let f = File::open(&args.path)
        .with_context(|| format!("could not read file '{}'", &args.path.to_str().unwrap()))?;
    let mut reader = BufReader::new(f);

    grrs::find_matches(&mut reader, &args.pattern, &mut stdout())?;

    Ok(())
}
