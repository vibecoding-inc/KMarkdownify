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
    
    # Create a comprehensive README for the package
    cat > "${pkgdir}/usr/share/doc/${pkgname}/README" << 'EOF'
KMarkdownify - PDF to Markdown Converter & Assignment Solver
=============================================================

VERSION: 3.0.0
PROJECT: https://github.com/profiluefter/KMarkdownify
LICENSE: MIT

OVERVIEW
--------
KMarkdownify converts PDF files to Markdown format or automatically solves 
assignments within PDF files using AI-powered OCR and language models via 
the OpenRouter API.

FEATURES
--------
- Convert PDFs to clean, well-formatted Markdown
- Solve assignments and problems in PDFs with detailed step-by-step solutions
- Extract metadata (Title, Author, Course, Due Date) with YAML frontmatter
- Seamless integration with KDE Plasma's Dolphin file manager
- Desktop notifications for conversion progress
- Configurable AI models, prompts, and parameters
- Memory-safe Rust implementation

INSTALLATION (Arch Linux)
--------------------------
This package was installed using the PKGBUILD. All files are in place.

QUICK START GUIDE
-----------------

1. GET AN API KEY
   Visit https://openrouter.ai/keys
   Create a free account and generate an API key

2. CONFIGURE THE API KEY
   Run these commands:
   
   mkdir -p ~/.config/kmarkdownify
   echo "YOUR_API_KEY_HERE" > ~/.config/kmarkdownify/api_key
   chmod 600 ~/.config/kmarkdownify/api_key

3. REFRESH KDE SERVICE MENUS
   
   kbuildsycoca5 --noincremental

4. USE IT!
   Right-click any PDF in Dolphin → KMarkdownify → Choose an action

USAGE
-----

There are two ways to use KMarkdownify:

A. FROM DOLPHIN (Recommended):
   1. Right-click on a PDF file in Dolphin
   2. Navigate to "KMarkdownify" submenu
   3. Choose an action:
      - "Convert to Markdown" - Extract text and convert to Markdown
      - "Solve Assignments" - Solve all problems in the PDF
   4. Wait for the notification
   5. Find the output file in the same directory

B. FROM COMMAND LINE:
   
   Convert mode (creates document.md):
   $ kmarkdownify document.pdf
   
   Solve mode (creates document_solved.md):
   $ kmarkdownify --solve homework.pdf
   
   See man page for details:
   $ man kmarkdownify

OUTPUT FILES
------------
- Convert mode: Creates <filename>.md in the same directory
- Solve mode: Creates <filename>_solved.md in the same directory

CONFIGURATION (Optional)
------------------------

Basic configuration is just the API key. For advanced options:

1. Copy the example config:
   cp /usr/share/doc/kmarkdownify/config.example ~/.config/kmarkdownify/config

2. Edit the file to customize:
   nano ~/.config/kmarkdownify/config

Available options:
- MODEL: AI model to use (default: mistralai/pixtral-large-latest)
- TEMPERATURE: Model temperature 0.0-1.0 (default: 0.1)
- MAX_TOKENS: Maximum response length (default: 8000)
- EXTRACT_METADATA: Extract metadata true/false (default: true)
- METADATA_FIELDS: Fields to extract (default: "Title,Author,Course,Due Date")
- SYSTEM_PROMPT_FILE: Path to custom prompt file
- CUSTOM_PROMPT: Inline custom prompt text

CUSTOM PROMPTS
--------------

Built-in prompts are located at:
- /usr/share/kmarkdownify/prompts/default_prompt.txt
- /usr/share/kmarkdownify/prompts/metadata_prompt.txt
- /usr/share/kmarkdownify/prompts/solve_prompt.txt

To create a custom prompt:
1. Create a file: ~/.config/kmarkdownify/my_prompt.txt
2. Add to config: SYSTEM_PROMPT_FILE=my_prompt.txt
3. Use {METADATA_FIELDS} placeholder to inject configured fields

REQUIREMENTS
------------

Runtime dependencies (installed automatically):
- file: MIME type detection
- dbus: D-Bus communication

Optional dependencies (recommended):
- kdialog: KDE-style dialog confirmations
- libnotify: Desktop notifications

External requirements:
- OpenRouter API key (get from https://openrouter.ai/keys)
- Internet connection for API calls
- Valid PDF files to convert

API COSTS
---------

OpenRouter offers a free tier with credits for testing.
The Mistral Pixtral Large model is cost-effective for most use cases.

Check current pricing: https://openrouter.ai/models

Typical costs:
- Small PDF (1-5 pages): $0.01-0.05
- Medium PDF (10-20 pages): $0.10-0.20
- Large PDF (50+ pages): $0.50+

SOLVE MODE DETAILS
------------------

The --solve flag uses AI to:
1. Extract all questions/problems from the PDF
2. Provide detailed step-by-step solutions
3. Show all work and reasoning
4. Include final answers
5. Format with proper Markdown/LaTeX for math

Output includes:
- Original question text (exactly as written)
- Complete solution with explanations
- Mathematical notation using LaTeX ($ and $$)
- Code blocks for programming problems
- YAML frontmatter with metadata

EXAMPLE OUTPUT (Solve Mode):
---
Title: Homework 1 - Calculus
Author: John Doe
Course: MATH 101
Due Date: 2025-12-01
---

# Question 1

Find the derivative of f(x) = x^2 + 3x + 2

## Solution

To find the derivative, we apply the power rule to each term:

1. Derivative of x^2: 2x
2. Derivative of 3x: 3
3. Derivative of 2: 0

Therefore: $f'(x) = 2x + 3$

**Answer:** $f'(x) = 2x + 3$

TROUBLESHOOTING
---------------

Issue: Service menu not appearing in Dolphin
Solution: Run "kbuildsycoca5 --noincremental" and restart Dolphin

Issue: "API key not found" error
Solution: Verify ~/.config/kmarkdownify/api_key exists and contains your key

Issue: "File is not a PDF" error
Solution: Ensure you're selecting a valid PDF file

Issue: API errors
Solution: 
  - Check your API key is valid at https://openrouter.ai
  - Verify you have available credits
  - Check your internet connection

Issue: Empty or incomplete output
Solution:
  - Try increasing MAX_TOKENS in config
  - Check if the PDF is readable (not scanned at low quality)
  - Verify the model supports the PDF size

GETTING HELP
------------

- Read the man page: man kmarkdownify
- Visit: https://github.com/profiluefter/KMarkdownify
- Report bugs: https://github.com/profiluefter/KMarkdownify/issues

FILES AND LOCATIONS
-------------------

Installed files:
- Binary: /usr/local/bin/kmarkdownify
- Service menu: /usr/share/kio/servicemenus/kmarkdownify.desktop
- Man page: /usr/share/man/man1/kmarkdownify.1
- Prompts: /usr/share/kmarkdownify/prompts/
- Documentation: /usr/share/doc/kmarkdownify/

User configuration:
- API key: ~/.config/kmarkdownify/api_key
- Config: ~/.config/kmarkdownify/config
- Custom prompts: ~/.config/kmarkdownify/*.txt

WHAT'S NEW IN VERSION 3.0
--------------------------

- Complete rewrite in Rust for safety and performance
- Added --solve mode to automatically solve assignments
- No shell script injection vulnerabilities
- Better error handling with Result types
- Type-safe API request/response handling
- Improved notification system
- Faster execution with compiled binary
- Man page documentation
- All features from v2.x maintained

SECURITY
--------

- API keys stored locally with recommended 600 permissions
- All API communication over HTTPS
- Memory-safe Rust implementation prevents common vulnerabilities
- No shell injection risks
- Credentials never logged or displayed

COPYRIGHT
---------

Copyright (c) 2025 KMarkdownify Contributors
Licensed under the MIT License
EOF
}
