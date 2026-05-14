use std::path::PathBuf;

use chrono::{DateTime, Datelike, Duration, Utc};
use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime};
use typst::layout::PagedDocument;
use typst::syntax::{FileId, Source};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt, World};

use crate::font::lora;

struct OrthosWorld {
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    fonts: Vec<Font>,
    main_source: Source,
    now: DateTime<Utc>,
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

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        let path = id.vpath().as_rooted_path();
        if path.starts_with("https:") || path.starts_with("http:") {
            Err(FileError::NotFound(PathBuf::from(path)))
        } else {
            match std::fs::read(path) {
                Ok(data) => Ok(Bytes::new(data)),
                Err(_) => Err(FileError::NotFound(PathBuf::from(path))),
            }
        }
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }

    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        let mut now = self.now;
        if let Some(offset) = offset {
            now += Duration::hours(offset);
        }
        let date = now.date_naive();
        Datetime::from_ymd(
            date.year(),
            (date.month0() + 1) as u8,
            (date.day0() + 1) as u8,
        )
    }
}

pub fn compile(markup: &str) -> Result<Vec<u8>, String> {
    let font_data = [
        (lora::LORA_REGULAR, 0u32),
        (lora::LORA_ITALIC, 0u32),
        (lora::LORA_BOLD, 0u32),
        (lora::LORA_BOLD_ITALIC, 0u32),
        (lora::LATIN_MODERN_MATH, 0u32),
        (lora::JETBRAINS_MONO, 0u32),
        (lora::JETBRAINS_MONO_ITALIC, 0u32),
    ];

    let fonts: Vec<Font> = font_data
        .iter()
        .filter_map(|(data, index)| Font::new(Bytes::new(*data), *index))
        .collect();

    if fonts.len() != 7 {
        return Err("Failed to load fonts".to_string());
    }

    let book = LazyHash::new(FontBook::from_fonts(&fonts));
    let library = LazyHash::new(Library::default());
    let main_source = Source::detached(markup);

    let world = OrthosWorld {
        library,
        book,
        fonts,
        main_source,
        now: Utc::now(),
    };

    let result = typst::compile::<PagedDocument>(&world);

    let document = result.output.map_err(|diags| {
        diags
            .into_iter()
            .map(|d| d.message.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    })?;

    let pdf = typst_pdf::pdf(&document, &Default::default()).map_err(|diags| {
        diags
            .into_iter()
            .map(|d| d.message.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    })?;

    Ok(pdf)
}
