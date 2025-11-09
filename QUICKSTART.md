# Quick Start Guide

Get up and running with KMarkdownify in 5 minutes!

## Prerequisites

- KDE Plasma Desktop with Dolphin file manager
- Internet connection
- OpenRouter API key (free tier available)

## Installation (Choose One)

### Option A: Arch Linux (Recommended for Arch users)

```bash
git clone https://github.com/profiluefter/KMarkdownify.git
cd KMarkdownify
makepkg -si
```

### Option B: Manual Installation (All Linux Distributions)

```bash
# 1. Install dependencies
sudo apt install bash curl jq coreutils file kdialog  # Debian/Ubuntu
# OR
sudo dnf install bash curl jq coreutils file kdialog  # Fedora
# OR
sudo pacman -S bash curl jq coreutils file kdialog    # Arch

# 2. Clone and install
git clone https://github.com/profiluefter/KMarkdownify.git
cd KMarkdownify
sudo install -Dm755 kmarkdownify.sh /usr/local/bin/kmarkdownify.sh
sudo install -Dm644 kmarkdownify.desktop /usr/share/kio/servicemenus/kmarkdownify.desktop

# 3. Refresh KDE menus
kbuildsycoca5 --noincremental
```

## Configuration

### Step 1: Get API Key

1. Go to https://openrouter.ai/keys
2. Sign up (free tier includes credits for testing)
3. Create an API key
4. Copy the key (starts with `sk-or-v1-...`)

### Step 2: Save API Key

```bash
mkdir -p ~/.config/kmarkdownify
echo "YOUR_API_KEY_HERE" > ~/.config/kmarkdownify/api_key
chmod 600 ~/.config/kmarkdownify/api_key
```

Replace `YOUR_API_KEY_HERE` with your actual key!

### Step 3: (Optional) Customize Configuration

For advanced features like custom metadata fields or different AI models:

```bash
# Copy example configuration
cp /usr/share/doc/kmarkdownify/config.example ~/.config/kmarkdownify/config

# Or if manually installed:
cp config.example ~/.config/kmarkdownify/config

# Edit to customize
nano ~/.config/kmarkdownify/config
```

**What you can customize:**
- AI model selection
- Metadata extraction (Title, Author, Course, Due Date, etc.)
- Temperature and token limits
- Custom conversion prompts

**Default behavior (without config file):**
- Uses Mistral Pixtral Large model
- Extracts metadata (Title, Author, Course, Due Date)
- Formats output with YAML frontmatter

## First Use

1. **Open Dolphin** file manager
2. **Find a PDF file** you want to convert
3. **Right-click** on the PDF
4. **Select:** `KMarkdownify` → `Convert to Markdown`
5. **Wait** for the notification (usually 10-30 seconds)
6. **Check** the same folder for `filename.md`

## Quick Test

Test from command line first:

```bash
# Download a sample PDF
curl -o /tmp/test.pdf "https://www.w3.org/WAI/ER/tests/xhtml/testfiles/resources/pdf/dummy.pdf"

# Convert it
/usr/local/bin/kmarkdownify.sh /tmp/test.pdf

# Check the result
cat /tmp/test.md
```

## Common Issues

### "Service menu not showing"
```bash
kbuildsycoca5 --noincremental
killall dolphin
```

### "API key not found"
```bash
# Make sure file exists and has correct permissions
ls -la ~/.config/kmarkdownify/api_key
cat ~/.config/kmarkdownify/api_key
```

### "Command not found: jq"
```bash
# Install missing dependency
sudo apt install jq  # or dnf/pacman depending on your distro
```

## Next Steps

- Read the full [README](README.md) for detailed documentation
- Check [INSTALL.md](INSTALL.md) for distribution-specific instructions
- See [docs/CONFIGURATION.md](docs/CONFIGURATION.md) for advanced customization
- Try converting different types of PDFs:
  - Technical documentation
  - Academic papers
  - Scanned documents
  - Forms and tables

## Tips

1. **Start small** - Test with simple PDFs first
2. **Check quality** - Review the Markdown output for accuracy
3. **Monitor costs** - Track usage in your OpenRouter dashboard
4. **Batch convert** - Use shell loops for multiple files
5. **Customize prompt** - Edit the script for domain-specific needs

## Need Help?

- 📖 Full documentation: [README.md](README.md)
- 🔧 Installation help: [INSTALL.md](INSTALL.md)
- ⚙️ Configuration: [docs/CONFIGURATION.md](docs/CONFIGURATION.md)
- 🐛 Report issues: https://github.com/profiluefter/KMarkdownify/issues

Happy converting! 🎉
