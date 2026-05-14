use comrak::nodes::{AstNode, ListType, NodeValue};
use std::borrow::Cow;
use std::collections::HashMap;

fn convert_inner<'a>(node: &'a AstNode<'a>, footnotes: &HashMap<String, String>) -> String {
    let data = node.data.borrow();
    match &data.value {
        NodeValue::Text(t) => match t {
            Cow::Borrowed(s) => escape_typst(s),
            Cow::Owned(s) => escape_typst(s),
        },

        NodeValue::SoftBreak => " ".to_string(),

        NodeValue::LineBreak => "\\\n".to_string(),

        NodeValue::Strong => {
            let inner = render_children(node, footnotes);
            format!("*{}*", inner)
        }

        NodeValue::Emph => {
            let inner = render_children(node, footnotes);
            format!("_{}_", inner)
        }

        NodeValue::Strikethrough => {
            let inner = render_children(node, footnotes);
            format!("#strike[{}]", inner)
        }

        NodeValue::Code(c) => format!("`{}`", c.literal),

        NodeValue::Link(link) => {
            let inner = render_children(node, footnotes);
            let url = escape_url(&link.url);
            if inner.is_empty() {
                format!("#link(\"{}\")", url)
            } else {
                format!("#link(\"{}\")[{}]", url, inner)
            }
        }

        NodeValue::Image(img) => {
            let url = if img.url.starts_with('/')
                || img.url.starts_with("http")
                || img.url.starts_with("https")
            {
                img.url.clone()
            } else {
                format!("./{}", img.url)
            };
            if !img.url.starts_with("http")
                && !img.url.starts_with("https")
                && !std::path::Path::new(&img.url).exists()
            {
                String::new()
            } else {
                format!("#image(\"{}\")", url)
            }
        }

        NodeValue::Heading(h) => {
            let prefix = "=".repeat(h.level as usize);
            let inner = render_children(node, footnotes);
            format!("\n{} {}\n\n", prefix, inner)
        }

        NodeValue::Paragraph => {
            let inner = render_children(node, footnotes);
            format!("{}\n\n", inner.trim())
        }

        NodeValue::CodeBlock(cb) => {
            let lang = cb.info.as_str();
            let code = &cb.literal;
            if lang.is_empty() {
                format!("```\n{}```\n\n", code)
            } else {
                format!("```{}\n{}```\n\n", lang, code)
            }
        }

        NodeValue::ThematicBreak => "\n#line(length: 100%)\n\n".to_string(),

        NodeValue::List(list) => {
            let default_marker = match list.list_type {
                ListType::Ordered => "+",
                ListType::Bullet => "-",
            };
            let mut out = String::new();
            for child in node.children() {
                let value = &child.data.borrow().value;
                let inner = render_children(child, footnotes);
                let trimmed = inner.trim();
                match value {
                    NodeValue::TaskItem(task) => {
                        let checked = task.symbol.is_some();
                        out.push_str(&format!(
                            "- {} {}\n",
                            if checked { "[x]" } else { "[ ]" },
                            trimmed
                        ));
                    }
                    NodeValue::Item(_) => {
                        out.push_str(&format!("{} {}\n", default_marker, trimmed));
                    }
                    _ => {}
                }
            }
            format!("\n{}\n", out)
        }

        NodeValue::BlockQuote => {
            let inner = render_children(node, footnotes);
            format!("#quote[{}]\n\n", inner.trim())
        }

        NodeValue::FootnoteReference(f) => match footnotes.get(&f.name) {
            Some(body) => format!("#footnote[{}]", body),
            None => {
                format!("#footnote[{}]", f.name)
            }
        },

        NodeValue::FootnoteDefinition(_) => String::new(),

        NodeValue::Math(m) => {
            let content = &m.literal;
            if content.trim().is_empty() {
                String::new()
            } else {
                let prepared = latex_to_typst_math(content.trim());
                if m.display_math {
                    format!("$ {} $\n\n", prepared)
                } else {
                    format!("$ {} $", prepared)
                }
            }
        }

        NodeValue::Item(_) => render_children(node, footnotes),

        NodeValue::Document => render_children(node, footnotes),

        NodeValue::Table(alignment) => {
            let cols: Vec<&str> =
                std::iter::repeat_n("auto", alignment.alignments.len().max(1)).collect();
            let mut out = format!("\n#table(columns: ({}),\n", cols.join(", "));
            for child in node.children() {
                let row_content = convert_inner(child, footnotes);
                if !row_content.is_empty() {
                    out.push_str(&format!("  {},\n", row_content.trim_end_matches(',')));
                }
            }
            out.push_str(")\n\n");
            out
        }

        NodeValue::TableRow(_) => {
            let cells: Vec<String> = node
                .children()
                .map(|c| convert_inner(c, footnotes))
                .collect();
            cells.join(", ")
        }

        NodeValue::TableCell => {
            let inner = render_children(node, footnotes);
            format!("[{}]", inner.trim())
        }

        NodeValue::TaskItem(node_task) => {
            let checked = node_task.symbol.is_some();
            let inner = render_children(node, footnotes);
            format!("{} {}", if checked { "[x]" } else { "[ ]" }, inner.trim())
        }

        NodeValue::HtmlInline(html) => convert_img_tag(html),

        NodeValue::HtmlBlock(hb) => convert_img_tag(&hb.literal),

        NodeValue::FrontMatter(_) => String::new(),

        _ => String::new(),
    }
}

fn convert_img_tag(html: &str) -> String {
    let lower = html.to_lowercase();
    let tag_start = match lower.find("<img") {
        Some(p) => p,
        None => return String::new(),
    };
    let after_tag = &html[tag_start + 4..];
    let src_pos = match after_tag.find("src=") {
        Some(p) => p,
        None => return String::new(),
    };
    let after_src = after_tag[src_pos + 4..].trim_start();
    let quote = match after_src.chars().next() {
        Some('"') => '"',
        Some('\'') => '\'',
        _ => return String::new(),
    };
    let end = match after_src[1..].find(quote) {
        Some(p) => p,
        None => return String::new(),
    };
    let src = &after_src[1..][..end];
    if !src.starts_with("http") && !src.starts_with("https") && !std::path::Path::new(src).exists()
    {
        String::new()
    } else {
        format!("#image(\"{}\")\n\n", src)
    }
}

fn render_children<'a>(node: &'a AstNode<'a>, footnotes: &HashMap<String, String>) -> String {
    let mut out = String::new();
    for child in node.children() {
        out.push_str(&convert_inner(child, footnotes));
    }
    out
}

fn latex_to_typst_math(s: &str) -> String {
    let commands: HashMap<&str, &str> = [
        ("int", "integral"),
        ("iint", "integral.double"),
        ("iiint", "integral.triple"),
        ("oint", "integral.contour"),
        ("infty", "oo"),
        ("sum", "sum"),
        ("prod", "product"),
        ("coprod", "coprod"),
        ("pi", "pi"),
        ("alpha", "alpha"),
        ("beta", "beta"),
        ("gamma", "gamma"),
        ("delta", "delta"),
        ("epsilon", "epsilon"),
        ("varepsilon", "epsilon"),
        ("zeta", "zeta"),
        ("eta", "eta"),
        ("theta", "theta"),
        ("vartheta", "theta"),
        ("iota", "iota"),
        ("kappa", "kappa"),
        ("lambda", "lambda"),
        ("mu", "mu"),
        ("nu", "nu"),
        ("xi", "xi"),
        ("rho", "rho"),
        ("varrho", "rho"),
        ("sigma", "sigma"),
        ("varsigma", "sigma"),
        ("tau", "tau"),
        ("upsilon", "upsilon"),
        ("phi", "phi"),
        ("varphi", "varphi"),
        ("chi", "chi"),
        ("psi", "psi"),
        ("omega", "omega"),
        ("Gamma", "Gamma"),
        ("Delta", "Delta"),
        ("Theta", "Theta"),
        ("Lambda", "Lambda"),
        ("Xi", "Xi"),
        ("Pi", "Pi"),
        ("Sigma", "Sigma"),
        ("Phi", "Phi"),
        ("Psi", "Psi"),
        ("Omega", "Omega"),
        ("to", "->"),
        ("rightarrow", "->"),
        ("Rightarrow", "=>"),
        ("leftarrow", "<-"),
        ("Leftarrow", "<="),
        ("leftrightarrow", "<->"),
        ("mapsto", "|->"),
        ("implies", "=>"),
        ("iff", "<=>"),
        ("partial", "partial"),
        ("nabla", "nabla"),
        ("forall", "forall"),
        ("exists", "exists"),
        ("nexists", "not.exists"),
        ("in", "in"),
        ("notin", "not.in"),
        ("subset", "subset"),
        ("subseteq", "subset.eq"),
        ("supset", "supset"),
        ("supseteq", "supset.eq"),
        ("cup", "union"),
        ("cap", "intersect"),
        ("emptyset", "nothing"),
        ("varnothing", "nothing"),
        ("sin", "sin"),
        ("cos", "cos"),
        ("tan", "tan"),
        ("csc", "csc"),
        ("sec", "sec"),
        ("cot", "cot"),
        ("sinh", "sinh"),
        ("cosh", "cosh"),
        ("tanh", "tanh"),
        ("arcsin", "arcsin"),
        ("arccos", "arccos"),
        ("arctan", "arctan"),
        ("log", "log"),
        ("ln", "ln"),
        ("lg", "lg"),
        ("lim", "lim"),
        ("det", "det"),
        ("gcd", "gcd"),
        ("lcm", "lcm"),
        ("max", "max"),
        ("min", "min"),
        ("sup", "sup"),
        ("inf", "inf"),
        ("arg", "arg"),
        ("deg", "deg"),
        ("dim", "dim"),
        ("hom", "hom"),
        ("ker", "ker"),
        ("Pr", "Pr"),
        ("exp", "exp"),
        ("approx", "~="),
        ("equiv", "=="),
        ("ne", "!="),
        ("neq", "!="),
        ("ge", ">="),
        ("geq", ">="),
        ("le", "<="),
        ("leq", "<="),
        ("times", "*"),
        ("div", "/"),
        ("pm", "plus.minus"),
        ("mp", "minus.plus"),
        ("cdot", "dot"),
        ("cdots", ".."),
        ("dots", "..."),
        ("ldots", "..."),
        ("circ", "circ"),
        ("bullet", "bullet"),
        ("star", "star"),
        ("ast", "ast"),
        ("dagger", "dagger"),
        ("ddagger", "ddagger"),
        ("setminus", "\\"),
        ("sim", "~"),
        ("simeq", "~="),
        ("cong", "~="),
        ("propto", "prop"),
        ("perp", "perp"),
        ("parallel", "parallel"),
        ("mid", "|"),
        ("ell", "ell"),
        ("hbar", "hbar"),
        ("hslash", "hslash"),
        ("Re", "Re"),
        ("Im", "Im"),
        ("aleph", "aleph"),
        ("prime", "'"),
        ("neg", "not"),
        ("lnot", "not"),
        ("wedge", "and"),
        ("land", "and"),
        ("vee", "or"),
        ("lor", "or"),
        ("oplus", "oplus"),
        ("ominus", "ominus"),
        ("otimes", "otimes"),
        ("oslash", "oslash"),
        ("odot", "odot"),
        ("angle", "angle"),
        ("measuredangle", "angle"),
        ("surd", "surd"),
        ("colon", ":"),
        ("vert", "|"),
        ("Vert", "||"),
        ("lvert", "|"),
        ("rvert", "|"),
        ("lVert", "||"),
        ("rVert", "||"),
    ]
    .iter()
    .cloned()
    .collect();

    let mut out = String::new();
    let mut prev_alpha = false;
    let mut in_command = false;
    let mut command_buf = String::new();

    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        match chars[i] {
            '\\' => {
                command_buf.clear();
                in_command = true;
                i += 1;
            }
            _ if in_command => {
                if chars[i].is_ascii_alphabetic() {
                    command_buf.push(chars[i]);
                    i += 1;
                    if i < len {
                        continue;
                    }
                }
                in_command = false;

                let replacement = commands.get(command_buf.as_str());
                match replacement {
                    Some(repl) => {
                        out.push_str(repl);
                        if command_buf == "int"
                            || command_buf == "iint"
                            || command_buf == "iiint"
                            || command_buf == "oint"
                            || command_buf == "sum"
                            || command_buf == "prod"
                        {
                            if i < len && chars[i] == '_' {
                                out.push('_');
                                i += 1;
                            } else if i < len && chars[i] == '^' {
                                out.push('^');
                                i += 1;
                            }
                        }
                    }
                    None => {
                        if command_buf == "frac" {
                            let num = parse_math_group(&chars, &mut i, len);
                            let den = parse_math_group(&chars, &mut i, len);
                            out.push('(');
                            out.push_str(&num);
                            out.push_str(")/(");
                            out.push_str(&den);
                            out.push(')');
                        } else if command_buf == "sqrt" {
                            if i < len && chars[i] == '[' {
                                i += 1;
                                let n = parse_math_content(&chars, &mut i, len, ']');
                                let radicand = parse_math_group(&chars, &mut i, len);
                                out.push_str("root(");
                                out.push_str(&radicand);
                                out.push(',');
                                out.push_str(&n);
                                out.push(')');
                            } else {
                                let radicand = parse_math_group(&chars, &mut i, len);
                                out.push_str("sqrt(");
                                out.push_str(&radicand);
                                out.push(')');
                            }
                        } else if command_buf == "mathrm" {
                            let arg = parse_math_group(&chars, &mut i, len);
                            out.push_str("upright(");
                            out.push_str(&arg);
                            out.push(')');
                        } else if command_buf == "mathbf" {
                            let arg = parse_math_group(&chars, &mut i, len);
                            out.push_str("bold(");
                            out.push_str(&arg);
                            out.push(')');
                        } else if command_buf == "mathcal" || command_buf == "mathscr" {
                            let arg = parse_math_group(&chars, &mut i, len);
                            out.push_str("cal(");
                            out.push_str(&arg);
                            out.push(')');
                        } else if command_buf == "mathbb" {
                            if i < len && chars[i] == '{' {
                                let arg = parse_math_group(&chars, &mut i, len);
                                out.push_str("bb(");
                                out.push_str(&arg);
                                out.push(')');
                            } else {
                                let arg = parse_math_group(&chars, &mut i, len);
                                out.push_str("frak(");
                                out.push_str(&arg);
                                out.push(')');
                            }
                        } else if command_buf == "operatorname" {
                            let arg = parse_math_group(&chars, &mut i, len);
                            if arg.starts_with('(') && arg.ends_with(')') {
                                out.push_str(&arg[1..arg.len() - 1]);
                            } else {
                                out.push_str(&arg);
                            }
                        } else if command_buf == "text" {
                            let arg = parse_math_group(&chars, &mut i, len);
                            out.push_str(&arg);
                        } else if command_buf == "left" || command_buf == "right" {
                            if i < len {
                                match chars[i] {
                                    '(' => out.push('('),
                                    ')' => out.push(')'),
                                    '[' => out.push('['),
                                    ']' => out.push(']'),
                                    '{' => out.push('{'),
                                    '}' => out.push('}'),
                                    '|' => out.push('|'),
                                    '.' => {}
                                    '\\' => {}
                                    c => out.push(c),
                                }
                                i += 1;
                            }
                        } else {
                            out.push('\\');
                            out.push_str(&command_buf);
                        }
                    }
                }
                if i >= len {
                    break;
                }
                prev_alpha = false;
            }
            c if c.is_ascii_alphabetic() => {
                if prev_alpha {
                    out.push(' ');
                }
                out.push(c);
                prev_alpha = true;
                i += 1;
            }
            '{' => {
                out.push('(');
                i += 1;
                prev_alpha = false;
            }
            '}' => {
                out.push(')');
                i += 1;
                prev_alpha = false;
            }
            _ => {
                out.push(chars[i]);
                prev_alpha = false;
                i += 1;
            }
        }
    }

    out
}

fn parse_math_group(chars: &[char], i: &mut usize, len: usize) -> String {
    while *i < len && chars[*i].is_whitespace() {
        *i += 1;
    }
    if *i < len && chars[*i] == '{' {
        *i += 1;
        let mut depth = 1usize;
        let mut content = String::new();
        while *i < len && depth > 0 {
            match chars[*i] {
                '{' => {
                    depth += 1;
                    content.push('{');
                    *i += 1;
                }
                '}' => {
                    depth -= 1;
                    if depth > 0 {
                        content.push('}');
                    }
                    *i += 1;
                }
                c => {
                    content.push(c);
                    *i += 1;
                }
            }
        }
        latex_to_typst_math(&content)
    } else {
        parse_math_content(chars, i, len, ' ')
    }
}

fn parse_math_content(chars: &[char], i: &mut usize, len: usize, terminator: char) -> String {
    let mut content = String::new();
    while *i < len {
        match chars[*i] {
            c if c == terminator => {
                *i += 1;
                break;
            }
            '{' => {
                let inner = parse_math_group(chars, i, len);
                content.push('{');
                content.push_str(&inner);
                content.push('}');
            }
            '}' => {
                break;
            }
            c => {
                content.push(c);
                *i += 1;
            }
        }
    }
    latex_to_typst_math(&content)
}

fn escape_typst(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('#', "\\#")
        .replace('$', "\\$")
        .replace('"', "\\\"")
}

fn escape_url(s: &str) -> String {
    s.replace('"', "%22")
}

pub fn convert<'a>(root: &'a AstNode<'a>) -> String {
    let mut footnotes: HashMap<String, String> = HashMap::new();
    let empty: HashMap<String, String> = HashMap::new();
    for child in root.children() {
        let data = child.data.borrow();
        if let NodeValue::FootnoteDefinition(fd) = &data.value {
            let body = render_children(child, &empty);
            footnotes
                .entry(fd.name.clone())
                .or_insert(body.trim().to_string());
        }
    }

    let mut body = String::new();
    for child in root.children() {
        body.push_str(&convert_inner(child, &footnotes));
    }
    body
}
