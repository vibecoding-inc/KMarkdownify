# KMarkdownify

A Plasma Dolphin service menu entry that converts PDF files to Markdown using OpenRouter API with Mistral OCR.

## Features

- 🐬 **Seamless Dolphin Integration** - Right-click context menu for PDF files
- 🤖 **AI-Powered OCR** - Uses Mistral's Pixtral Large model for accurate text extraction
- 📝 **Clean Markdown Output** - Preserves document structure, headings, lists, and formatting
- 📊 **Metadata Extraction** - Automatically extracts Title, Author, Course, Due Date from PDFs
- 🎯 **YAML Frontmatter** - Formats metadata as YAML frontmatter in markdown output
- ⚙️ **Highly Configurable** - Customize model, temperature, prompts, and metadata fields
- 🔔 **User-Friendly Notifications** - KDialog and fallback notification support
- ⚡ **Simple Setup** - Easy configuration and installation
- 🛡️ **Graceful Degradation** - Uses 'N/A' for metadata fields that cannot be found

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

# Install the service menu
sudo install -Dm644 kmarkdownify.desktop /usr/share/kio/servicemenus/kmarkdownify.desktop

# Install prompt files
sudo mkdir -p /usr/local/share/kmarkdownify/prompts
sudo install -Dm644 prompts/default_prompt.txt /usr/local/share/kmarkdownify/prompts/default_prompt.txt
sudo install -Dm644 prompts/metadata_prompt.txt /usr/local/share/kmarkdownify/prompts/metadata_prompt.txt

# Install example configuration
sudo mkdir -p /usr/share/doc/kmarkdownify
sudo install -Dm644 config.example /usr/share/doc/kmarkdownify/config.example
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

### Advanced Configuration (Optional)

KMarkdownify supports advanced configuration through a config file. This allows you to customize:
- AI model selection
- Temperature and token limits
- Metadata extraction behavior
- Custom prompts

**Setup advanced configuration:**

```bash
# Copy the example configuration file
cp /usr/share/doc/kmarkdownify/config.example ~/.config/kmarkdownify/config

# Or if you installed manually:
cp config.example ~/.config/kmarkdownify/config

# Edit the configuration file
nano ~/.config/kmarkdownify/config
```

**Configuration options:**

- `MODEL`: Choose the AI model (default: `mistralai/pixtral-large-latest`)
- `TEMPERATURE`: Control output consistency (0.0-1.0, default: 0.1)
- `MAX_TOKENS`: Maximum response length (default: 8000)
- `EXTRACT_METADATA`: Enable/disable metadata extraction (default: true)
- `METADATA_FIELDS`: Comma-separated list of fields to extract (default: "Title,Author,Course,Due Date")
- `SYSTEM_PROMPT_FILE`: Path to a custom prompt file (optional, see Custom Prompts section)
- `CUSTOM_PROMPT`: Override the default prompt with your own inline text (optional)

**Example: Disable metadata extraction**
```bash
echo "EXTRACT_METADATA=false" >> ~/.config/kmarkdownify/config
```

**Example: Add custom metadata fields**
```bash
echo "METADATA_FIELDS=Title,Author,Course,Due Date,Student ID,Professor" >> ~/.config/kmarkdownify/config
```

### Custom Prompts

KMarkdownify supports custom prompts in three ways:

1. **Using Built-in Prompt Files (Default)**
   - The tool includes two default prompt files:
     - `default_prompt.txt` - Simple text extraction without metadata
     - `metadata_prompt.txt` - Text extraction with metadata and YAML frontmatter
   - These are installed to `/usr/share/kmarkdownify/prompts/` (Arch) or `/usr/local/share/kmarkdownify/prompts/` (manual install)
   - The appropriate prompt is automatically selected based on `EXTRACT_METADATA` setting

2. **Using a Custom Prompt File**
   - Create your own prompt file and reference it in the config:
   ```bash
   # Create a custom prompt file
   cat > ~/.config/kmarkdownify/custom_prompt.txt << 'EOF'
   Please extract text from this PDF and format it as a technical documentation page.
   Focus on code blocks, API references, and preserve all formatting details.
   EOF
   
   # Configure KMarkdownify to use it
   echo "SYSTEM_PROMPT_FILE=custom_prompt.txt" >> ~/.config/kmarkdownify/config
   ```
   - You can use absolute paths or relative paths (relative to `~/.config/kmarkdownify/`)
   - Use `{METADATA_FIELDS}` placeholder in your prompt to inject the configured metadata fields

3. **Using Inline Custom Prompt**
   - Override the prompt directly in the config file:
   ```bash
   echo 'CUSTOM_PROMPT="Extract only headings and bullet points from this PDF."' >> ~/.config/kmarkdownify/config
   ```

**Priority Order:**
1. `CUSTOM_PROMPT` (inline in config) - highest priority
2. `SYSTEM_PROMPT_FILE` (custom prompt file)
3. Built-in prompt files based on `EXTRACT_METADATA` setting - default

### Metadata Extraction

When metadata extraction is enabled (default), KMarkdownify will:
1. Analyze the PDF for common metadata fields (Title, Author, Course, Due Date)
2. Format the metadata as YAML frontmatter at the beginning of the markdown file
3. Use "N/A" for any fields that cannot be found
4. Follow the metadata with the full document content

**Example output with metadata:**
```markdown
---
Title: Assignment 1 - Data Structures
Author: John Doe
Course: CS 101
Due Date: 2025-11-15
---

# Assignment 1: Data Structures

## Question 1
...
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
2. Loads configuration from `~/.config/kmarkdownify/config` (if present)
3. Determines the appropriate system prompt (custom file, inline custom, or built-in)
4. Encodes the PDF file to base64 format
5. Constructs a JSON API request using `jq` with stdin to avoid argument length limits
6. Sends the PDF to OpenRouter API using the configured AI model (default: Mistral's Pixtral Large)
7. The AI model performs OCR and extracts metadata (Title, Author, Course, Due Date) if enabled
8. Converts the text to Markdown format with optional YAML frontmatter containing the metadata
9. Saves the result as a `.md` file in the same directory
10. Shows a notification upon completion

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

You have several options to customize the conversion prompt:

1. **Edit the built-in prompt files** (requires admin access):
   - `/usr/share/kmarkdownify/prompts/default_prompt.txt`
   - `/usr/share/kmarkdownify/prompts/metadata_prompt.txt`

2. **Create a custom prompt file** (recommended):
   ```bash
   # Create your custom prompt
   nano ~/.config/kmarkdownify/my_prompt.txt
   
   # Configure KMarkdownify to use it
   echo "SYSTEM_PROMPT_FILE=my_prompt.txt" >> ~/.config/kmarkdownify/config
   ```

3. **Use inline custom prompt in config**:
   ```bash
   echo 'CUSTOM_PROMPT="Your custom instructions here..."' >> ~/.config/kmarkdownify/config
   ```

See the "Custom Prompts" section in Configuration for more details.

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