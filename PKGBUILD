# Maintainer: Your Name <your.email@example.com>
pkgname=kmarkdownify
pkgver=1.0.0
pkgrel=1
pkgdesc="Plasma Dolphin service menu for converting PDF files to Markdown using OpenRouter API with Mistral OCR"
arch=('any')
url="https://github.com/profiluefter/KMarkdownify"
license=('MIT')
depends=('bash' 'curl' 'jq' 'coreutils' 'file')
optdepends=(
    'kdialog: for KDE dialog notifications'
    'libnotify: for fallback notifications'
)
source=("kmarkdownify.sh"
        "kmarkdownify.desktop")
sha256sums=('SKIP'
            'SKIP')

package() {
    # Install the shell script
    install -Dm755 "${srcdir}/kmarkdownify.sh" "${pkgdir}/usr/local/bin/kmarkdownify.sh"
    
    # Install the desktop service menu file
    install -Dm644 "${srcdir}/kmarkdownify.desktop" \
        "${pkgdir}/usr/share/kservices5/ServiceMenus/kmarkdownify.desktop"
    
    # Create directory for API key configuration
    install -dm755 "${pkgdir}/usr/share/doc/${pkgname}"
    
    # Create a README for the package
    cat > "${pkgdir}/usr/share/doc/${pkgname}/README" << 'EOF'
KMarkdownify - PDF to Markdown Converter
=========================================

Setup Instructions:
-------------------

1. Get an API key from OpenRouter:
   Visit https://openrouter.ai/keys and create an account to get your API key.

2. Configure your API key:
   mkdir -p ~/.config/kmarkdownify
   echo "YOUR_API_KEY_HERE" > ~/.config/kmarkdownify/api_key
   chmod 600 ~/.config/kmarkdownify/api_key

3. Restart Dolphin or refresh the service menus:
   kbuildsycoca5 --noincremental

Usage:
------

1. Right-click on any PDF file in Dolphin
2. Navigate to "KMarkdownify" → "Convert to Markdown"
3. Wait for the conversion to complete
4. The markdown file will be saved in the same directory with .md extension

Requirements:
-------------

- Active OpenRouter API key (free tier available)
- Internet connection for API calls
- PDF files for conversion

For more information, visit: https://github.com/profiluefter/KMarkdownify
EOF
}
