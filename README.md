# KMarkdownify

A Plasma Dolphin service menu entry that converts PDF files to Markdown using OpenRouter API with Mistral OCR.

## Features

- 🐬 **Seamless Dolphin Integration** - Right-click context menu for PDF files
- 🤖 **AI-Powered OCR** - Uses Mistral's Pixtral Large model for accurate text extraction
- 📝 **Clean Markdown Output** - Preserves document structure, headings, lists, and formatting
- 🔔 **User-Friendly Notifications** - KDialog and fallback notification support
- ⚡ **Simple Setup** - Easy configuration and installation

## Requirements

- Arch Linux (or any Linux distribution with manual installation)
- Bash
- curl
- jq
- coreutils
- file
- KDE Plasma Desktop (for Dolphin integration)
- OpenRouter API key (free tier available)

### Optional Dependencies

- `kdialog` - For KDE-style dialog notifications
- `libnotify` - For fallback notifications

## Installation

### Arch Linux (using PKGBUILD)

1. Clone this repository:
```bash
git clone https://github.com/profiluefter/KMarkdownify.git
cd KMarkdownify
```

2. Build and install the package:
```bash
makepkg -si
```

3. Configure your API key (see Configuration section below)

### Manual Installation (Other Distributions)

1. Clone this repository:
```bash
git clone https://github.com/profiluefter/KMarkdownify.git
cd KMarkdownify
```

2. Install required dependencies:
```bash
# For Debian/Ubuntu
sudo apt install bash curl jq coreutils file kdialog libnotify-bin

# For Fedora
sudo dnf install bash curl jq coreutils file kdialog libnotify

# For Arch
sudo pacman -S bash curl jq coreutils file kdialog libnotify
```

3. Install the files:
```bash
# Install the shell script
sudo install -Dm755 kmarkdownify.sh /usr/local/bin/kmarkdownify.sh

# Install the service menu (KDE 5)
sudo install -Dm644 kmarkdownify.desktop /usr/share/kservices5/ServiceMenus/kmarkdownify.desktop

# For KDE 6/Plasma 6, use this path instead:
# sudo install -Dm644 kmarkdownify.desktop ~/.local/share/kio/servicemenus/kmarkdownify.desktop
```

4. Refresh KDE service menus:
```bash
kbuildsycoca5 --noincremental
```

## Configuration

### Setting up OpenRouter API Key

1. **Get an API Key:**
   - Visit [OpenRouter](https://openrouter.ai/keys)
   - Create an account (free tier available)
   - Generate an API key

2. **Configure the API Key:**
```bash
# Create the configuration directory
mkdir -p ~/.config/kmarkdownify

# Save your API key
echo "YOUR_API_KEY_HERE" > ~/.config/kmarkdownify/api_key

# Secure the file
chmod 600 ~/.config/kmarkdownify/api_key
```

## Usage

1. **Open Dolphin File Manager**
2. **Navigate to a PDF file**
3. **Right-click on the PDF file**
4. **Select "KMarkdownify" → "Convert to Markdown"**
5. **Wait for the conversion** (a notification will appear)
6. **Find your Markdown file** in the same directory with `.md` extension

### Command Line Usage

You can also run the script directly from the command line:

```bash
/usr/local/bin/kmarkdownify.sh /path/to/your/file.pdf
```

The converted Markdown file will be saved as `/path/to/your/file.md`

## How It Works

1. The script receives the PDF file path from Dolphin's context menu
2. Encodes the PDF file to base64 format
3. Sends the PDF to OpenRouter API using Mistral's Pixtral Large model
4. The AI model extracts and converts the text to Markdown format
5. Saves the result as a `.md` file in the same directory
6. Shows a notification upon completion

## API Costs

OpenRouter offers a free tier with credits for testing. The Mistral Pixtral Large model is relatively cost-effective:
- Check current pricing at [OpenRouter Pricing](https://openrouter.ai/models)
- Typical PDF conversion costs vary based on document size and complexity

## Troubleshooting

### Service menu not appearing
- Make sure you installed the `.desktop` file in the correct location
- Run `kbuildsycoca5 --noincremental` to refresh the cache
- Restart Dolphin

### API key errors
- Verify your API key is correct and saved in `~/.config/kmarkdownify/api_key`
- Check that the file has no extra whitespace or newlines
- Ensure you have credits available in your OpenRouter account

### Missing dependencies
- Install all required dependencies: `curl`, `jq`, `base64`, `file`
- For notifications, install `kdialog` or `libnotify`

### Permission errors
- Ensure the script is executable: `chmod +x /usr/local/bin/kmarkdownify.sh`
- Check that you can write to the directory containing the PDF

## Development

### Testing the Script

```bash
# Test with a sample PDF
./kmarkdownify.sh /path/to/test.pdf

# Check the output
cat /path/to/test.md
```

### Modifying the Conversion Prompt

Edit the `kmarkdownify.sh` file and modify the prompt text in the JSON payload section to customize how the AI extracts and formats the content.

## License

MIT License - See LICENSE file for details

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Credits

- Uses [OpenRouter](https://openrouter.ai/) for API routing
- Powered by [Mistral AI's Pixtral Large model](https://docs.mistral.ai/)
- Built for KDE Plasma and Dolphin file manager

## Support

For issues, questions, or suggestions, please open an issue on GitHub:
https://github.com/profiluefter/KMarkdownify/issues