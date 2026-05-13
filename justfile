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
