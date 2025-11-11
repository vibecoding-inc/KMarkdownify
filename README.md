# KMarkdownify

Convert PDF files to Markdown or automatically solve assignments using AI-powered OCR.

## Features

- 📄 **Convert PDFs to Markdown** - Extract and format text content with proper structure
- 🎓 **Solve Assignments** - Automatically solve problems in PDFs with detailed solutions  
- 🤖 **AI-Powered** - Uses Mistral's Pixtral Large OCR model via OpenRouter API
- 🐬 **Dolphin Integration** - Right-click context menu for seamless workflow
- 📊 **Metadata Extraction** - Auto-extracts Title, Author, Course, Due Date to YAML frontmatter
- ⚙️ **Highly Configurable** - Customize models, prompts, temperature, and more
- 🦀 **Rust Implementation** - Memory-safe, fast, and reliable
- 🔔 **Desktop Notifications** - Real-time progress updates

## Installation (Arch Linux)

```bash
# Clone the repository
git clone https://github.com/profiluefter/KMarkdownify.git
cd KMarkdownify

# Build and install using PKGBUILD
makepkg -si
```

## Setup

1. **Get an API key** from [OpenRouter](https://openrouter.ai/keys) (free tier available)

2. **Configure the API key:**
```bash
mkdir -p ~/.config/kmarkdownify
echo "YOUR_API_KEY_HERE" > ~/.config/kmarkdownify/api_key
chmod 600 ~/.config/kmarkdownify/api_key
```

3. **Refresh KDE service menus:**
```bash
kbuildsycoca5 --noincremental
```

## Usage

### From Dolphin (Recommended)

1. Right-click any PDF file
2. Navigate to **KMarkdownify** submenu
3. Choose:
   - **Convert to Markdown** - Creates `filename.md`
   - **Solve Assignments** - Creates `filename_solved.md` with solutions

### From Command Line

```bash
# Convert PDF to Markdown
kmarkdownify document.pdf

# Solve assignments in PDF
kmarkdownify --solve homework.pdf
```

### View Documentation

```bash
man kmarkdownify
```

## Configuration (Optional)

```bash
# Copy example config
cp /usr/share/doc/kmarkdownify/config.example ~/.config/kmarkdownify/config

# Edit configuration
nano ~/.config/kmarkdownify/config
```

Available options:
- `MODEL` - AI model selection
- `TEMPERATURE` - Output consistency (0.0-1.0)
- `MAX_TOKENS_PER_PAGE` - Response length limit per page
- `TIMEOUT_PER_PAGE` - API timeout in seconds per page (default: 60)
- `EXTRACT_METADATA` - Enable/disable metadata extraction
- `METADATA_FIELDS` - Fields to extract
- `SYSTEM_PROMPT_FILE` - Custom prompt file path
- `CUSTOM_PROMPT` - Inline custom prompt

## Example Output

### Convert Mode
```markdown
---
Title: Research Paper
Author: John Doe
Course: CS 101
Due Date: 2025-12-01
---

# Introduction

The study of algorithms...
```

### Solve Mode
```markdown
---
Title: Homework 1
Author: Jane Smith
Course: MATH 201
Due Date: 2025-12-05
---

# Question 1

Find the derivative of f(x) = x² + 3x + 2

## Solution

Apply the power rule to each term:
1. d/dx(x²) = 2x
2. d/dx(3x) = 3
3. d/dx(2) = 0

**Answer:** f'(x) = 2x + 3
```

## Requirements

- Arch Linux (or compatible)
- Rust toolchain (for building)
- `file` command
- KDE Plasma with Dolphin
- OpenRouter API key
- Internet connection

**Optional:**
- `kdialog` - KDE dialogs
- `libnotify` - Desktop notifications

## API Costs

OpenRouter offers a free tier. Mistral Pixtral Large pricing:
- Small PDFs (1-5 pages): ~$0.01-0.05
- Medium PDFs (10-20 pages): ~$0.10-0.20

Check current pricing at [OpenRouter Models](https://openrouter.ai/models)

## Documentation

All documentation is in the man page:
```bash
man kmarkdownify
```

Or view the inline help in the PKGBUILD README:
```bash
cat /usr/share/doc/kmarkdownify/README
```

## License

MIT License - See LICENSE file

## Contributing

Issues and pull requests welcome at [GitHub](https://github.com/profiluefter/KMarkdownify)

## Credits

- [OpenRouter](https://openrouter.ai/) - API routing
- [Mistral AI](https://mistral.ai/) - Pixtral Large OCR model
- Built for KDE Plasma and Dolphin
