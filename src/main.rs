use clap::Parser;

#[derive(Parser)]
#[command(name = "orthos", version, about = "Markdown to PDF compiler")]
struct Cli {
    input: String,

    #[arg(short = 'o', long)]
    output: Option<String>,
}

fn main() -> miette::Result<()> {
    let _cli = Cli::parse();
    eprintln!("orthos 0.1.0 — not yet implemented");
    Ok(())
}
