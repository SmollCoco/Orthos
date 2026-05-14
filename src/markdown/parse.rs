use comrak::nodes::AstNode;
use comrak::{Arena, Options, parse_document};

pub fn parse<'a>(input: &'a str, arena: &'a Arena<'a>) -> &'a AstNode<'a> {
    let mut options = Options::default();
    options.extension.footnotes = true;
    options.extension.math_dollars = true;
    options.extension.math_code = true;
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.superscript = true;
    options.parse.smart = true;

    parse_document(arena, input, &options)
}
