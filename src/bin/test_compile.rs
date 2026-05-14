use chrono::{Datelike, Utc};
use std::path::PathBuf;
use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime};
use typst::layout::PagedDocument;
use typst::syntax::{FileId, Source};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt, World};

static LORA_REGULAR: &[u8] = include_bytes!("../../assets/fonts/Lora-Regular.ttf");
static LORA_ITALIC: &[u8] = include_bytes!("../../assets/fonts/Lora-Italic.ttf");
static LORA_BOLD: &[u8] = include_bytes!("../../assets/fonts/Lora-Bold.ttf");
static LORA_BOLD_ITALIC: &[u8] = include_bytes!("../../assets/fonts/Lora-BoldItalic.ttf");
static LATIN_MODERN_MATH: &[u8] = include_bytes!("../../assets/fonts/latinmodern-math.otf");
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
        self.fonts.get(index).cloned()
    }
    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        let d = Utc::now().date_naive();
        Datetime::from_ymd(d.year(), (d.month0() + 1) as u8, (d.day0() + 1) as u8)
    }
}

fn compile(markup: &str) -> Result<Vec<u8>, String> {
    let font_data = [
        (LORA_REGULAR, 0u32),
        (LORA_ITALIC, 0u32),
        (LORA_BOLD, 0u32),
        (LORA_BOLD_ITALIC, 0u32),
        (LATIN_MODERN_MATH, 0u32),
        (JETBRAINS_MONO, 0u32),
        (JETBRAINS_MONO_ITALIC, 0u32),
    ];

    let fonts: Vec<Font> = font_data
        .iter()
        .filter_map(|(data, index)| Font::new(Bytes::new(*data), *index))
        .collect();

    if fonts.len() != 7 {
        return Err("Failed to load fonts".to_string());
    }

    let library = LazyHash::new(Library::default());
    let book = LazyHash::new(FontBook::from_fonts(&fonts));
    let main_source = Source::detached(markup);

    let world = OrthosWorld {
        library,
        book,
        fonts,
        main_source,
    };
    let result = typst::compile::<PagedDocument>(&world);

    let document = result.output.map_err(|diags| {
        diags
            .into_iter()
            .map(|d| d.message.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    })?;

    typst_pdf::pdf(&document, &Default::default()).map_err(|e| {
        e.into_iter()
            .map(|d| d.message.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    })
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let markup = if args.len() > 1 { &args[1] } else { "Hello" };

    match compile(markup) {
        Ok(pdf) => {
            std::fs::write("/tmp/out_compile.pdf", &pdf).unwrap();
            println!("OK: {} bytes", pdf.len());
        }
        Err(e) => println!("ERR: {}", e),
    }
}
