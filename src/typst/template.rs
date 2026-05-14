pub fn preamble() -> &'static str {
    r#"#set page(
  paper: "a4",
  margin: (top: 3cm, bottom: 3cm, left: 3cm, right: 3cm),
  fill: rgb("faf9f8"),
)
#set text(font: ("Lora"), size: 11pt, lang: "en")
#show math.equation: set text(font: "Latin Modern Math")
#set par(justify: true, leading: 0.8em)

#show heading: set block(above: 1.2em, below: 0.5em)
#show heading.where(level: 1): set text(size: 1.6em, weight: "bold")
#show heading.where(level: 2): set text(size: 1.35em, weight: "bold")
#show heading.where(level: 3): set text(size: 1.15em, weight: 600)
#show heading.where(level: 4): set text(size: 1.05em, weight: 600)

#show raw.where(block: true): set block(
  fill: rgb("f0ede8"),
  inset: (top: 10pt, bottom: 10pt, left: 14pt, right: 14pt),
  radius: 6pt,
  stroke: 0.5pt + rgb("e2ddd5"),
  above: 0.8em,
  below: 0.8em,
)
#show raw.where(block: true): set text(font: "JetBrains Mono", size: 9.5pt)
#show raw.where(block: false): set block(
  fill: rgb("e8e2d8"),
  inset: (left: 5pt, right: 5pt, top: 1pt, bottom: 1pt),
  radius: 3pt,
)
#show raw.where(block: false): set text(
  font: "JetBrains Mono",
  fill: rgb("5c4f3f"),
)

#show quote: set block(
  stroke: (left: 3pt + rgb("a27e5c")),
  fill: rgb("f4f0eb"),
  inset: (top: 8pt, bottom: 8pt, left: 14pt, right: 14pt),
  radius: 0pt,
  above: 1em,
  below: 1em,
)
#show quote: set text(style: "italic", size: 0.95em, fill: rgb("5c4f3f"))

#show link: set text(fill: rgb("3674b5"))

#show list: set block(below: 0.5em)

#show table: set table(
  inset: 8pt,
  stroke: 0.5pt + rgb("d8d0c4"),
)
#show table.header: set block(fill: rgb("ece6dc"))
#show table.cell.where(y: 0): set text(weight: "bold")
"#
}
