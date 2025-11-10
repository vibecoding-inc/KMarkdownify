# Maintainer: Your Name <your.email@example.com>
pkgname=kmarkdownify
pkgver=3.0.0
pkgrel=1
pkgdesc="Plasma Dolphin service menu for converting PDF files to Markdown using OpenRouter API (Rust rewrite)"
arch=('x86_64' 'aarch64')
url="https://github.com/profiluefter/KMarkdownify"
license=('MIT')
depends=('file' 'dbus')
makedepends=('rust' 'cargo')
optdepends=(
    'kdialog: for KDE dialog confirmations'
    'libnotify: for desktop notifications'
)
source=("Cargo.toml"
        "Cargo.lock"
        "src/main.rs"
        "kmarkdownify.desktop"
        "config.example"
        "default_prompt.txt"
        "metadata_prompt.txt")
sha256sums=('SKIP'
            'SKIP'
            'SKIP'
            'SKIP'
            'SKIP'
            'SKIP'
            'SKIP')

build() {
    cd "${srcdir}"
    
    # Build the Rust binary
    cargo build --release --locked
}

package() {
    # Install the binary
    install -Dm755 "${srcdir}/target/release/kmarkdownify" "${pkgdir}/usr/local/bin/kmarkdownify"
    
    # Install the desktop service menu file
    install -Dm644 "${srcdir}/kmarkdownify.desktop" \
        "${pkgdir}/usr/share/kio/servicemenus/kmarkdownify.desktop"
    
    # Install prompt files
    install -Dm644 "${srcdir}/default_prompt.txt" \
        "${pkgdir}/usr/share/${pkgname}/prompts/default_prompt.txt"
    install -Dm644 "${srcdir}/metadata_prompt.txt" \
        "${pkgdir}/usr/share/${pkgname}/prompts/metadata_prompt.txt"
    
    # Install example configuration file
    install -Dm644 "${srcdir}/config.example" \
        "${pkgdir}/usr/share/doc/${pkgname}/config.example"
    
    # Create a README for the package
    cat > "${pkgdir}/usr/share/doc/${pkgname}/README" << 'EOF'
KMarkdownify - PDF to Markdown Converter (Rust v3.0)
=====================================================

Setup Instructions:
-------------------

1. Get an API key from OpenRouter:
   Visit https://openrouter.ai/keys and create an account to get your API key.

2. Configure your API key:
   mkdir -p ~/.config/kmarkdownify
   echo "YOUR_API_KEY_HERE" > ~/.config/kmarkdownify/api_key
   chmod 600 ~/.config/kmarkdownify/api_key

3. (Optional) Customize configuration:
   cp /usr/share/doc/kmarkdownify/config.example ~/.config/kmarkdownify/config
   # Edit the config file to customize model, temperature, metadata extraction, etc.

4. Restart Dolphin or refresh the service menus:
   kbuildsycoca5 --noincremental

Usage:
------

1. Right-click on any PDF file in Dolphin
2. Navigate to "KMarkdownify" → "Convert to Markdown"
3. Wait for the conversion to complete
4. The markdown file will be saved in the same directory with .md extension

What's New in v3.0:
-------------------

- Rewritten in Rust for improved safety, reliability, and performance
- No shell script injection vulnerabilities
- Better error handling and type safety
- Faster execution
- All features from v2.x maintained

Features:
---------

- Metadata extraction: Automatically extracts Title, Author, Course, Due Date
- Configurable models and prompts
- YAML frontmatter support
- Graceful handling of missing metadata (uses N/A)
- Memory safe implementation in Rust

Requirements:
-------------

- Active OpenRouter API key (free tier available)
- Internet connection for API calls
- PDF files for conversion

For more information, visit: https://github.com/profiluefter/KMarkdownify
EOF
}
