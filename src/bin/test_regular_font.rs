use chrono::{Datelike, Utc};
use std::path::PathBuf;
use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime};
use typst::layout::PagedDocument;
use typst::syntax::{FileId, Source};
use typst::text::{Font, FontBook, FontStyle, FontVariant, FontWeight};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt, World};

static LATIN_MODERN_MATH: &[u8] = include_bytes!("../../assets/fonts/latinmodern-math.otf");

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
    let font_paths = vec![
        "/usr/share/fonts/liberation/LiberationSerif-Regular.ttf",
        "/usr/share/fonts/liberation/LiberationSerif-Italic.ttf",
    ];

    let mut all_fonts = vec![];
    for path in &font_paths {
        if let Ok(data) = std::fs::read(path)
            && let Some(f) = Font::new(Bytes::new(data), 0)
        {
            let info = f.info();
            eprintln!(
                "Font: family={:?}, style={:?}, flags={:?}",
                info.family, info.variant.style, info.flags
            );
            all_fonts.push(f);
        }
    }

    if let Some(f) = Font::new(Bytes::new(LATIN_MODERN_MATH), 0) {
        eprintln!(
            "Font: family={:?}, flags={:?}",
            f.info().family,
            f.info().flags
        );
        all_fonts.push(f);
    }

    eprintln!("Loaded {} fonts", all_fonts.len());

    let book = FontBook::from_fonts(&all_fonts);

    for (family, _) in book.families() {
        eprintln!("Book family: {:?}", family);
    }

    let v = FontVariant::new(FontStyle::Normal, FontWeight::REGULAR, Default::default());
    for f in &["liberation serif", "libertinus serif", "lora"] {
        eprintln!("select({:?}, Normal): {:?}", f, book.select(f, v));
    }

    let library = LazyHash::new(Library::default());
    let main_source = Source::detached("Hello $a + b$");
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
            std::fs::write("/tmp/out_reg.pdf", &pdf).unwrap();
            eprintln!("OK: wrote /tmp/out_reg.pdf ({} bytes)", pdf.len());
        }
        Err(diags) => {
            for d in &diags {
                eprintln!("ERR: {}", d.message);
            }
        }
    }
}
