use comrak::Arena;
use miette::{Context, miette};

use crate::markdown::parse;
use crate::typst::{compile, convert, template};

pub fn run(input: &str) -> Result<Vec<u8>, miette::Report> {
    let arena = Arena::new();
    let root = parse::parse(input, &arena);

    let preamble = template::preamble();
    let body = convert::convert(root);

    let markup = format!("{}{}", preamble, body);

    compile::compile(&markup)
        .map_err(|e| miette!("Compilation failed: {}", e))
        .wrap_err("Failed to compile document")
}
