use std::fs;
use std::path::PathBuf;

use clap::Parser;
use miette::{Context, IntoDiagnostic};

mod font;
mod markdown;
mod pipeline;
mod typst;

#[derive(Parser)]
#[command(name = "orthos", version, about = "Markdown to PDF compiler")]
struct Cli {
    input: String,

    #[arg(short = 'o', long)]
    output: Option<String>,
}

fn main() -> miette::Result<()> {
    let cli = Cli::parse();

    let source = fs::read_to_string(&cli.input)
        .into_diagnostic()
        .wrap_err_with(|| format!("Failed to read input file '{}'", cli.input))?;

    let pdf = pipeline::run(&source)?;

    let output = cli.output.unwrap_or_else(|| {
        let input = PathBuf::from(&cli.input);
        input.with_extension("pdf").to_string_lossy().to_string()
    });

    fs::write(&output, &pdf)
        .into_diagnostic()
        .wrap_err_with(|| format!("Failed to write output file '{}'", output))?;

    Ok(())
}
