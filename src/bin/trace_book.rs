use chrono::{Datelike, Utc};
use std::path::PathBuf;
use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime};
use typst::layout::PagedDocument;
use typst::syntax::{FileId, Source};
use typst::text::{Font, FontBook, FontStyle, FontVariant, FontWeight};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt, World};

static LORA_REGULAR: &[u8] = include_bytes!("../../assets/fonts/Lora-Regular.ttf");
static LORA_ITALIC: &[u8] = include_bytes!("../../assets/fonts/Lora-Italic.ttf");
static LORA_BOLD: &[u8] = include_bytes!("../../assets/fonts/Lora-Bold.ttf");
static LORA_BOLD_ITALIC: &[u8] = include_bytes!("../../assets/fonts/Lora-BoldItalic.ttf");
static JETBRAINS_MONO: &[u8] = include_bytes!("../../assets/fonts/JetBrainsMono[wght].ttf");
static JETBRAINS_MONO_ITALIC: &[u8] =
    include_bytes!("../../assets/fonts/JetBrainsMono-Italic[wght].ttf");

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
        let f = self.fonts.get(index).cloned();
        eprintln!("TRACE font({}): {}", index, f.is_some());
        f
    }
    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        let d = Utc::now().date_naive();
        Datetime::from_ymd(d.year(), (d.month0() + 1) as u8, (d.day0() + 1) as u8)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let markup = if args.len() > 1 { &args[1] } else { "Hello" };

    let font_data = [
        (LORA_REGULAR, 0u32),
        (LORA_ITALIC, 0u32),
        (LORA_BOLD, 0u32),
        (LORA_BOLD_ITALIC, 0u32),
        (JETBRAINS_MONO, 0u32),
        (JETBRAINS_MONO_ITALIC, 0u32),
    ];
    let mut fonts: Vec<Font> = font_data
        .iter()
        .filter_map(|(data, index)| Font::new(Bytes::new(*data), *index))
        .collect();

    if let Ok(data) = std::fs::read("/usr/share/fonts/noto/NotoSansMath-Regular.ttf")
        && let Some(f) = Font::new(Bytes::new(data), 0)
    {
        fonts.push(f);
    }

    let book = FontBook::from_fonts(&fonts);
    let library = LazyHash::new(Library::default());
    let main_source = Source::detached(markup);

    // Manual check what select returns for all possible families
    eprintln!("=== Pre-compile font check ===");
    let v = FontVariant::new(FontStyle::Normal, FontWeight::REGULAR, Default::default());
    for family in &[
        "libertinus serif",
        "lora",
        "noto sans math",
        "new computer modern math",
    ] {
        let result = book.select(family, v);
        eprintln!("select({:?}, Normal/400): {:?}", family, result);
    }
    eprintln!("=== End pre-check ===");

    let world = OrthosWorld {
        library,
        book: LazyHash::new(book),
        fonts,
        main_source,
    };

    let result = typst::compile::<PagedDocument>(&world);
    match result.output {
        Ok(doc) => {
            let pdf = typst_pdf::pdf(&doc, &Default::default()).unwrap();
            std::fs::write("/tmp/out_trace.pdf", &pdf).unwrap();
            eprintln!("OK: wrote /tmp/out_trace.pdf ({} bytes)", pdf.len());
        }
        Err(diags) => {
            for d in &diags {
                eprintln!("ERR: {}", d.message);
            }
        }
    }
}
