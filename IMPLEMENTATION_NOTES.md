# Implementation Notes

This document provides technical details about the KMarkdownify implementation.

## Problem Statement

Create a Plasma Dolphin Service Menu entry for a program that converts PDF files to Markdown files. The service menu calls a shell script which sends the PDF file to the OpenRouter API using Mistral OCR for text extraction. Include an Arch Linux PKGBUILD for installation.

## Solution Architecture

### 1. Service Menu Integration (`kmarkdownify.desktop`)

**Purpose**: Provides right-click context menu integration in KDE Dolphin file manager.

**Key Design Decisions**:
- Uses KDE service menu format
- Filters by MIME type `application/pdf` to only show for PDF files
- Uses `%f` placeholder for single file selection
- Placed in `/usr/share/kio/servicemenus/` for system-wide availability
- Alternative location: `~/.local/share/kio/servicemenus/` for per-user installation

**Technical Details**:
```ini
[Desktop Entry]
Type=Service
ServiceTypes=KonqPopupMenu/Plugin
MimeType=application/pdf;
Actions=ConvertToMarkdown;
X-KDE-Submenu=KMarkdownify
```

### 2. Conversion Script (`kmarkdownify.sh`)

**Purpose**: Handles the actual PDF to Markdown conversion via OpenRouter API.

**Architecture**:
```
User selects PDF → Script validates → Encode to Base64 → 
API Request → Parse Response → Save Markdown → Notify User
```

**Key Components**:

#### Error Handling
- `set -euo pipefail`: Strict error handling mode
- Custom error functions with dual notification (kdialog + notify-send)
- Graceful degradation (kdialog falls back to notify-send)

#### Input Validation
1. Argument count check
2. File existence verification
3. PDF type validation using `file` command
4. Dependency checks (curl, jq, base64)
5. API key validation

#### API Integration
- **Endpoint**: `https://openrouter.ai/api/v1/chat/completions`
- **Model**: `mistralai/pixtral-large-latest` (optimized for OCR)
- **Authentication**: Bearer token via Authorization header
- **Input Format**: Base64-encoded PDF in image_url field
- **Temperature**: 0.1 (low for consistent OCR results)
- **Max Tokens**: 8000 (approximately 6000 words)

#### JSON Construction
Uses `jq -Rs` with stdin and writes to a temporary file to avoid all size limits:
```bash
TMPFILE=$(mktemp)
trap 'rm -f "$TMPFILE"' EXIT

printf '%s' "$PDF_BASE64" | jq -Rs \
    --arg model "$MODEL" \
    --arg prompt "$PROMPT_TEXT" \
    '{ ... }' > "$TMPFILE"
```

**Key Changes (v2.1.0)**:
- Prior versions passed the base64-encoded PDF as a command-line argument (`--arg pdf_data`)
- This caused "Argument list too long" errors for large PDFs due to system ARG_MAX limits
- Now uses a temporary file approach to eliminate all size restrictions:
  1. PDF base64 data is piped to `jq -Rs` (raw input, slurp mode) via stdin
  2. jq constructs the JSON payload and writes it directly to a temporary file
  3. curl reads the JSON payload from the temporary file using `--data-binary @"$TMPFILE"`
- This approach avoids command-line length limits, environment variable size limits, and pipe buffer issues
- The temporary file is automatically cleaned up on script exit using a trap
- Prevents JSON injection attacks and properly escapes special characters

#### Response Processing
1. Check for API error responses
2. Extract markdown content using `jq`
3. Validate content is not empty
4. Write to output file (same directory, .md extension)

### 3. Package Management (`PKGBUILD`)

**Purpose**: Provides standardized installation for Arch Linux users.

**Installation Paths**:
- Script: `/usr/local/bin/kmarkdownify.sh` (mode 755)
- Desktop file: `/usr/share/kio/servicemenus/kmarkdownify.desktop` (mode 644)
- Prompt files: `/usr/share/kmarkdownify/prompts/` (Arch) or `/usr/local/share/kmarkdownify/prompts/` (manual)
- Documentation: `/usr/share/doc/kmarkdownify/`

**Installed Files (v2.1.0+)**:
- `default_prompt.txt` - Simple text extraction without metadata
- `metadata_prompt.txt` - Text extraction with metadata and YAML frontmatter
- `config.example` - Example configuration file

**Dependencies**:
- Required: bash, curl, jq, coreutils, file
- Optional: kdialog (KDE notifications), libnotify (fallback notifications)

**Build Process**:
```bash
makepkg -si  # Build and install with dependencies
```

## Security Considerations

### API Key Management
- Stored in user's home: `~/.config/kmarkdownify/api_key`
- Recommended permissions: 600 (user read/write only)
- Never logged or displayed
- Transmitted only via HTTPS Authorization header

### Input Sanitization
- All file paths properly quoted
- File type validated before processing
- JSON constructed via jq (no string interpolation)
- No eval or command substitution of user input

### Network Security
- HTTPS-only communication
- No sensitive data in URL parameters
- Proper HTTP headers (Referer, X-Title)
- API key in Authorization header, not URL

## API Usage

### Request Structure
```json
{
  "model": "mistralai/pixtral-large-latest",
  "messages": [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Please extract all text content..."
        },
        {
          "type": "image_url",
          "image_url": {
            "url": "data:application/pdf;base64,..."
          }
        }
      ]
    }
  ],
  "temperature": 0.1,
  "max_tokens": 8000
}
```

### Response Structure
```json
{
  "choices": [
    {
      "message": {
        "content": "# Markdown Content Here"
      }
    }
  ]
}
```

## Testing Performed

1. **Syntax Validation**: `bash -n kmarkdownify.sh` ✅
2. **Style Check**: `shellcheck kmarkdownify.sh` ✅
3. **Error Handling Tests**:
   - No arguments provided ✅
   - Non-existent file ✅
   - Non-PDF file type ✅
   - Missing API key ✅
4. **Dependency Checks**: curl, jq, base64, file ✅

## Future Enhancements

Potential improvements for future versions:

1. ~~**Metadata Extraction**: Extract Title, Author, Course, Due Date~~ ✅ Implemented in v2.0.0
2. ~~**Configurable Settings**: Allow model, temperature, and prompt customization~~ ✅ Implemented in v2.0.0
3. **Batch Processing**: Process multiple PDFs at once
4. **Progress Bar**: For large files
5. **Custom Output Path**: Allow user to specify output location
6. **Retry Logic**: Automatic retry on API failures
7. **Caching**: Cache conversions to avoid re-processing
8. **GUI Configuration**: Settings dialog for API key and preferences
9. **Language Detection**: Optimize prompt based on detected language
10. **Page Selection**: Convert only specific pages

## Version 2.0.0 Features (Implemented)

### Configuration System
The script now supports a flexible configuration file at `~/.config/kmarkdownify/config`:
- **MODEL**: Choose different AI models
- **TEMPERATURE**: Control output consistency (0.0-1.0)
- **MAX_TOKENS**: Set maximum response length
- **EXTRACT_METADATA**: Enable/disable metadata extraction
- **METADATA_FIELDS**: Customize which metadata fields to extract
- **SYSTEM_PROMPT_FILE**: Path to custom prompt file (v2.1.0+)
- **CUSTOM_PROMPT**: Override default prompts with inline custom instructions

### Prompt System (v2.1.0+)
The script supports three ways to specify prompts, with priority order:

1. **CUSTOM_PROMPT** (highest priority) - Inline prompt in config file
2. **SYSTEM_PROMPT_FILE** - Path to custom prompt file
   - Can be absolute path or relative to `~/.config/kmarkdownify/`
   - Supports `{METADATA_FIELDS}` placeholder for field substitution
3. **Built-in prompt files** (default)
   - `default_prompt.txt` - Used when EXTRACT_METADATA=false
   - `metadata_prompt.txt` - Used when EXTRACT_METADATA=true
   - Located in `/usr/share/kmarkdownify/prompts/` or `/usr/local/share/kmarkdownify/prompts/`
   - Fallback to inline prompts if files not found

**Prompt File Discovery**:
The script automatically searches for prompt files in:
1. `/usr/share/kmarkdownify/prompts/` (Arch Linux package installation)
2. `/usr/local/share/kmarkdownify/prompts/` (Manual installation)
3. `{SCRIPT_DIR}/prompts/` (Development/local usage)
4. Falls back to inline prompts if no files found

### Metadata Extraction
When enabled (default), the script:
1. Extracts specified metadata fields from PDFs
2. Formats them as YAML frontmatter
3. Uses "N/A" for fields that cannot be found
4. Supports customizable field lists

Default fields: Title, Author, Course, Due Date

Example output:
```markdown
---
Title: Assignment 1
Author: John Doe
Course: CS 101
Due Date: 2025-11-15
---

# Document content...
```

### Prompt Modes
Three operational modes based on configuration:
1. **Custom Inline Prompt Mode**: When CUSTOM_PROMPT is set (highest priority)
2. **Custom File Prompt Mode**: When SYSTEM_PROMPT_FILE is set
3. **Built-in Prompt Mode**: Uses default or metadata prompt files based on EXTRACT_METADATA
   - **Metadata Extraction Mode**: When EXTRACT_METADATA=true (default)
   - **Simple Extraction Mode**: When EXTRACT_METADATA=false

### Backward Compatibility
- Works without config file (uses sensible defaults)
- Existing installations continue to work
- Optional configuration enhances capabilities

## Compatibility

**Tested Environments**:
- KDE Plasma 5
- Dolphin file manager
- Bash 4.0+
- curl 7.x+
- jq 1.5+

**Distribution Support**:
- Arch Linux (via PKGBUILD)
- Debian/Ubuntu (manual installation)
- Fedora (manual installation)
- openSUSE (manual installation)
- Any Linux distribution with KDE Plasma

## Troubleshooting Guide

### Service Menu Not Appearing
1. Run `kbuildsycoca5 --noincremental`
2. Restart Dolphin: `killall dolphin`
3. Check file location: `/usr/share/kio/servicemenus/kmarkdownify.desktop`
4. Try user location: `~/.local/share/kio/servicemenus/`

### Script Errors
1. Check dependencies: `for cmd in curl jq base64 file; do which $cmd; done`
2. Verify API key: `cat ~/.config/kmarkdownify/api_key`
3. Test script: `/usr/local/bin/kmarkdownify.sh /path/to/test.pdf`
4. Check permissions: `ls -l /usr/local/bin/kmarkdownify.sh`

### API Issues
1. Verify API key is valid at https://openrouter.ai/
2. Check account balance/credits
3. Test with smaller PDF first
4. Review OpenRouter API status

## References

- [KDE Service Menus Documentation](https://userbase.kde.org/Dolphin/File_Management#Service_Menus)
- [OpenRouter API Documentation](https://openrouter.ai/docs)
- [Mistral AI Documentation](https://docs.mistral.ai/)
- [Arch Linux PKGBUILD Manual](https://wiki.archlinux.org/title/PKGBUILD)

## Credits

- **OpenRouter**: API routing and management
- **Mistral AI**: Pixtral Large OCR model
- **KDE Project**: Dolphin file manager and service menu framework
- **jq**: JSON processing
- **shellcheck**: Bash script analysis

---

Implementation completed: 2025-11-09
Version: 2.1.0

**Changelog for v2.1.0:**
- **CRITICAL FIX**: Resolved "Argument list too long" error for large PDFs in both jq and curl
- Changed jq JSON construction to use stdin instead of command-line arguments
- Changed curl to receive JSON payload from a temporary file using `--data-binary @"$TMPFILE"`
- Temporary file approach eliminates command-line, environment variable, and pipe buffer size limits
- Extracted system prompts into separate files (default_prompt.txt, metadata_prompt.txt)
- Moved prompt files to repository root for proper PKGBUILD source handling
- Added SYSTEM_PROMPT_FILE configuration option for custom prompt files
- Implemented prompt file discovery system with fallback locations
- Added support for {METADATA_FIELDS} placeholder in custom prompts
- Updated PKGBUILD to properly handle prompt directory structure
- Improved documentation with prompt customization guide

**Changelog for v2.0.0:**
- Added metadata extraction with YAML frontmatter support
- Implemented configuration file system
- Added customizable model selection
- Added configurable temperature and max_tokens
- Added support for custom prompts
- Added customizable metadata fields
- Maintained backward compatibility
