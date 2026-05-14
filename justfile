build:
    cargo build --release

check:
    cargo check

test:
    cargo test

lint:
    cargo clippy -- -D warnings

fmt:
    cargo fmt --check

fmt-fix:
    cargo fmt

install:
    cargo install --path . --root ~/.local

fetch-fonts:
    mkdir -p assets/fonts
    curl -L -o assets/fonts/Lora-Variable.ttf \
        "https://github.com/google/fonts/raw/main/ofl/lora/Lora%5Bwght%5D.ttf"
    curl -L -o assets/fonts/Lora-Italic-Variable.ttf \
        "https://github.com/google/fonts/raw/main/ofl/lora/Lora-Italic%5Bwght%5D.ttf"
    python3 -c "
import math, os
from fontTools.ttLib import TTFont
from fontTools.varLib.instancer import instantiateVariableFont
src = 'assets/fonts/Lora-Variable.ttf'
src_i = 'assets/fonts/Lora-Italic-Variable.ttf'
for s, w, o in [(src, 400, 'Lora-Regular.ttf'), (src, 700, 'Lora-Bold.ttf'), (src_i, 400, 'Lora-Italic.ttf'), (src_i, 700, 'Lora-BoldItalic.ttf')]:
    f = TTFont(s)
    f = instantiateVariableFont(f, {'wght': w}, inplace=False)
    f.save('assets/fonts/' + o)
    f.close()
    print(f'Extracted {o}')
# Increase italic angle for better visual slant
for fn in ['Lora-Italic.ttf', 'Lora-BoldItalic.ttf']:
    path = os.path.join('assets/fonts', fn)
    f = TTFont(path)
    glyf = f['glyf']
    add_rad = math.tan(math.radians(12))
    for gname in glyf.keys():
        g = glyf[gname]
        if hasattr(g, 'coordinates') and g.numberOfContours > 0:
            for i in range(len(g.coordinates)):
                g.coordinates[i] = (g.coordinates[i][0] + g.coordinates[i][1] * add_rad, g.coordinates[i][1])
    f['post'].italicAngle = 12
    f.save(path)
    f.close()
    print(f'Increased italic angle for {fn}')
"
    curl -L -o assets/fonts/latinmodern-math.otf \
        "https://mirrors.ctan.org/fonts/lm-math.zip"
    unzip -o assets/fonts/lm-math.zip -d /tmp/lm-math-extract 2>/dev/null || true
    cp /tmp/lm-math-extract/lm-math/opentype/latinmodern-math.otf assets/fonts/ 2>/dev/null || true
    rm -rf /tmp/lm-math-extract assets/fonts/lm-math.zip
    curl -L -o /tmp/JetBrainsMono.zip \
        "https://github.com/JetBrains/JetBrainsMono/releases/download/v2.304/JetBrainsMono-2.304.zip"
    unzip -o /tmp/JetBrainsMono.zip "fonts/variable/JetBrainsMono\[wght\].ttf" \
        "fonts/variable/JetBrainsMono-Italic\[wght\].ttf" -d /tmp/
    cp /tmp/fonts/variable/JetBrainsMono\[wght\].ttf assets/fonts/
    cp /tmp/fonts/variable/JetBrainsMono-Italic\[wght\].ttf assets/fonts/
    rm -rf /tmp/JetBrainsMono.zip /tmp/fonts
    rm -f assets/fonts/Lora-Variable.ttf assets/fonts/Lora-Italic-Variable.ttf
