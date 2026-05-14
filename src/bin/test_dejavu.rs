use chrono::{Datelike, Utc};
use std::path::PathBuf;
use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime};
use typst::layout::PagedDocument;
use typst::syntax::{FileId, Source};
use typst::text::{Font, FontBook, FontStyle, FontVariant, FontWeight};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt, World};

// No built-in fonts - only load system fonts

struct OrthosWorld {
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    fonts: Vec<Font>,
    main_source: Source,
}

impl World for OrthosWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }
    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }
    fn main(&self) -> FileId {
        self.main_source.id()
    }
    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.main_source.id() {
            Ok(self.main_source.clone())
        } else {
            Err(FileError::NotFound(PathBuf::from(
                id.vpath().as_rooted_path(),
            )))
        }
    }
    fn file(&self, _id: FileId) -> FileResult<Bytes> {
        Err(FileError::NotFound(PathBuf::from(
            _id.vpath().as_rooted_path(),
        )))
    }
    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }
    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        let d = Utc::now().date_naive();
        Datetime::from_ymd(d.year(), (d.month0() + 1) as u8, (d.day0() + 1) as u8)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let markup = if args.len() > 1 { &args[1] } else { "Hello" };

    // Load DejaVu Math TeX Gyre font
    let mut all_fonts = vec![];

    let math_data = std::fs::read("/usr/share/fonts/TTF/DejaVuMathTeXGyre.ttf").unwrap();
    if let Some(f) = Font::new(Bytes::new(math_data), 0) {
        let info = f.info();
        println!("Font: family={:?}, flags={:?}", info.family, info.flags);
        all_fonts.push(f);
    }

    println!("Loaded {} fonts", all_fonts.len());

    let book = FontBook::from_fonts(&all_fonts);

    println!("Book families:");
    for (fam, _) in book.families() {
        println!("  {:?}", fam);
    }

    // Check matching with "new computer modern math" (one of math fallbacks)
    for name in &[
        "new computer modern math",
        "dejavu math tex gyre",
        "libertinus serif",
    ] {
        let name2 = name.to_lowercase();
        let r = book.select(
            &name2,
            FontVariant::new(FontStyle::Normal, FontWeight::REGULAR, Default::default()),
        );
        println!("book.select({:?}, Normal): {:?}", name2, r);
    }

    let library = LazyHash::new(Library::default());
    let main_source = Source::detached(markup);
    let world = OrthosWorld {
        library,
        book: LazyHash::new(book),
        fonts: all_fonts,
        main_source,
    };

    let result = typst::compile::<PagedDocument>(&world);
    match result.output {
        Ok(doc) => {
            let pdf = typst_pdf::pdf(&doc, &Default::default()).unwrap();
            std::fs::write("/tmp/out_dv.pdf", &pdf).unwrap();
            println!("OK: wrote /tmp/out_dv.pdf ({} bytes)", pdf.len());
        }
        Err(diags) => {
            for d in &diags {
                println!("ERR: {}", d.message);
            }
        }
    }
}
