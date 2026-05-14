use typst::foundations::Bytes;
use typst::text::{Font, FontBook, FontStyle, FontVariant, FontWeight};

static LORA_REGULAR: &[u8] = include_bytes!("../../assets/fonts/Lora-Regular.ttf");
static LORA_ITALIC: &[u8] = include_bytes!("../../assets/fonts/Lora-Italic.ttf");
static LORA_BOLD: &[u8] = include_bytes!("../../assets/fonts/Lora-Bold.ttf");
static LORA_BOLD_ITALIC: &[u8] = include_bytes!("../../assets/fonts/Lora-BoldItalic.ttf");
static LATIN_MODERN_MATH: &[u8] = include_bytes!("../../assets/fonts/latinmodern-math.otf");
static JETBRAINS_MONO: &[u8] = include_bytes!("../../assets/fonts/JetBrainsMono[wght].ttf");
static JETBRAINS_MONO_ITALIC: &[u8] =
    include_bytes!("../../assets/fonts/JetBrainsMono-Italic[wght].ttf");

fn main() {
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
        .filter_map(|(data, index)| {
            let f = Font::new(Bytes::new(*data), *index);
            if let Some(ref font) = f {
                let info = font.info();
                println!(
                    "Font: family={:?}, style={:?}, weight={:?}, stretch={:?}, flags={:?}",
                    info.family,
                    info.variant.style,
                    info.variant.weight,
                    info.variant.stretch,
                    info.flags
                );
            }
            f
        })
        .collect();

    let book = FontBook::from_fonts(&fonts);
    println!("\nFamilies in book:");
    for (family, infos) in book.families() {
        println!("  Family: {:?}", family);
        for info in infos {
            println!(
                "    -> style={:?}, weight={:?}, stretch={:?}",
                info.variant.style, info.variant.weight, info.variant.stretch
            );
        }
    }

    // Test exact family name
    for family_name in &["Lora", "lora", "Lora Variable"] {
        let v = FontVariant::new(FontStyle::Normal, FontWeight::REGULAR, Default::default());
        println!(
            "\nSelect({:?}, Normal/400/Default): {:?}",
            family_name,
            book.select(family_name, v)
        );
    }

    // Also test with select_family
    println!("\nselect_family results:");
    for family_name in &["Lora", "lora"] {
        let ids: Vec<_> = book.select_family(family_name).collect();
        println!("  {:?}: {:?}", family_name, ids);
    }
}
