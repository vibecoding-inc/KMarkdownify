# Maintainer: Your Name <your.email@example.com>
pkgname=kmarkdownify
pkgver=3.0.0
pkgrel=1
pkgdesc="Plasma Dolphin service menu for converting PDF files to Markdown or solving assignments using OpenRouter API"
arch=('x86_64' 'aarch64')
url="https://github.com/profiluefter/KMarkdownify"
license=('MIT')
depends=('file' 'dbus')
makedepends=('rust' 'cargo')
optdepends=(
    'kdialog: for KDE dialog confirmations'
    'libnotify: for desktop notifications'
)

# For local builds, we'll use the current directory
# For AUR/releases, this should be changed to download from a release tarball
_is_local_build=true

if [ "$_is_local_build" = true ]; then
    source=()
    sha256sums=()
else
    source=("${pkgname}-${pkgver}.tar.gz::https://github.com/profiluefter/KMarkdownify/archive/refs/tags/v${pkgver}.tar.gz")
    sha256sums=('SKIP')
fi

build() {
    if [ "$_is_local_build" = true ]; then
        cd "${startdir}"
    else
        cd "${srcdir}/KMarkdownify-${pkgver}"
    fi
    
    # Build the Rust binary
    cargo build --release --locked
}

package() {
    if [ "$_is_local_build" = true ]; then
        cd "${startdir}"
    else
        cd "${srcdir}/KMarkdownify-${pkgver}"
    fi
    
    # Install the binary
    install -Dm755 "target/release/kmarkdownify" "${pkgdir}/usr/local/bin/kmarkdownify"
    
    # Install the desktop service menu file
    install -Dm644 "kmarkdownify.desktop" \
        "${pkgdir}/usr/share/kio/servicemenus/kmarkdownify.desktop"
    
    # Install prompt files
    install -Dm644 "default_prompt.txt" \
        "${pkgdir}/usr/share/${pkgname}/prompts/default_prompt.txt"
    install -Dm644 "metadata_prompt.txt" \
        "${pkgdir}/usr/share/${pkgname}/prompts/metadata_prompt.txt"
    install -Dm644 "solve_prompt.txt" \
        "${pkgdir}/usr/share/${pkgname}/prompts/solve_prompt.txt"
    
    # Install man page
    install -Dm644 "kmarkdownify.1" \
        "${pkgdir}/usr/share/man/man1/kmarkdownify.1"
    
    # Install example configuration file
    install -Dm644 "config.example" \
        "${pkgdir}/usr/share/doc/${pkgname}/config.example"
    
    # Install LICENSE if it exists
    if [ -f "LICENSE" ]; then
        install -Dm644 "LICENSE" \
            "${pkgdir}/usr/share/licenses/${pkgname}/LICENSE"
    fi
}
