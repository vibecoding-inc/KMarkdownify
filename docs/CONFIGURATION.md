# KMarkdownify Configuration Guide

## Overview

KMarkdownify supports two configuration files:
1. **API Key** (`~/.config/kmarkdownify/api_key`) - Required
2. **Configuration** (`~/.config/kmarkdownify/config`) - Optional

## API Key Configuration

Your API key should be stored in: `~/.config/kmarkdownify/api_key`

Example:
```
sk-or-v1-abc123def456...
```

**Security Notes:**
- Never share your API key
- Never commit your API key to version control
- Set proper permissions: `chmod 600 ~/.config/kmarkdownify/api_key`

## Configuration File Format

The configuration file uses a simple `KEY=VALUE` format. Create it at `~/.config/kmarkdownify/config`.

### Available Configuration Options

#### MODEL
The OpenRouter AI model to use for PDF conversion.

**Default:** `mistralai/pixtral-large-latest`

**Example:**
```bash
MODEL=mistralai/pixtral-large-latest
```

**Alternative Models:**
- `anthropic/claude-3-opus` - Very accurate, higher cost
- `openai/gpt-4-vision-preview` - Good OCR, moderate cost
- `google/gemini-pro-vision` - Google's vision model

**Note:** Different models have different capabilities and costs. Pixtral Large is optimized for OCR tasks.

#### TEMPERATURE
Controls the randomness of the AI's output. Lower values produce more consistent results.

**Default:** `0.1`
**Range:** `0.0` - `1.0`

**Example:**
```bash
TEMPERATURE=0.1
```

**Guidelines:**
- `0.0-0.2`: Very consistent, literal extraction (recommended for OCR)
- `0.3-0.7`: Balanced (default for most conversions)
- `0.8-1.0`: More creative interpretation (use cautiously)

#### MAX_TOKENS
Maximum length of the AI's response in tokens. Approximately 1 token = 0.75 words.

**Default:** `8000` (approximately 6000 words)

**Example:**
```bash
MAX_TOKENS=8000
```

**Guidelines:**
- Increase for very long documents
- Check your model's token limits
- Higher values may increase API costs

#### EXTRACT_METADATA
Enable or disable automatic metadata extraction from PDFs.

**Default:** `true`
**Values:** `true` or `false`

**Example:**
```bash
EXTRACT_METADATA=true
```

When enabled, KMarkdownify will:
- Extract document metadata (Title, Author, Course, Due Date, etc.)
- Format it as YAML frontmatter
- Use "N/A" for fields that cannot be found

**Example output:**
```markdown
---
Title: Assignment 1
Author: John Doe
Course: CS 101
Due Date: 2025-11-15
---

# Document content starts here...
```

#### METADATA_FIELDS
Comma-separated list of metadata fields to extract from the PDF.

**Default:** `Title,Author,Course,Due Date`

**Example:**
```bash
METADATA_FIELDS="Title,Author,Course,Due Date,Student ID,Professor"
```

**Common Fields:**
- Title
- Author
- Course
- Due Date
- Student ID
- Professor
- University
- Department
- Assignment Number
- Date Submitted

#### CUSTOM_PROMPT
Override the default conversion prompt with your own custom instructions.

**Default:** (empty - uses built-in prompts)

**Example:**
```bash
CUSTOM_PROMPT="Extract all text from this technical PDF and format as Markdown. Preserve code blocks, API references, and technical notation. Use ``` for code and maintain proper heading hierarchy."
```

**When to use:**
- Domain-specific requirements (technical docs, academic papers, forms)
- Custom formatting preferences
- Specialized metadata fields

## Complete Configuration Example

Here's a complete example configuration file:

```bash
# ~/.config/kmarkdownify/config

# Use Claude for higher accuracy
MODEL=anthropic/claude-3-opus

# Lower temperature for very consistent output
TEMPERATURE=0.05

# Increase token limit for long documents
MAX_TOKENS=12000

# Enable metadata extraction
EXTRACT_METADATA=true

# Extract additional fields for academic use
METADATA_FIELDS="Title,Author,Course,Due Date,Student ID,Professor,University"

# Don't use custom prompt - use the default
# CUSTOM_PROMPT=""
```

## Configuration Examples by Use Case


### Example Configurations by Use Case

#### For Technical Documentation
```bash
# ~/.config/kmarkdownify/config
MODEL=mistralai/pixtral-large-latest
TEMPERATURE=0.1
MAX_TOKENS=10000
EXTRACT_METADATA=false
CUSTOM_PROMPT="Please extract all text from this PDF technical document and convert it to well-formatted Markdown. Preserve code blocks, technical terms, API references, and maintain proper formatting for tables and lists. Use appropriate Markdown syntax for code (```), headings (#), and technical notation."
```

#### For Academic Papers
```bash
# ~/.config/kmarkdownify/config
MODEL=anthropic/claude-3-opus
TEMPERATURE=0.1
MAX_TOKENS=12000
EXTRACT_METADATA=true
METADATA_FIELDS="Title,Author,Journal,Publication Date,DOI,Abstract"
CUSTOM_PROMPT="Please extract all text from this academic PDF and convert it to Markdown. Preserve the abstract, sections, subsections, citations, references, figures, and tables. Maintain the academic structure and formatting conventions. Extract metadata for: Title, Author, Journal, Publication Date, DOI, Abstract."
```

#### For University Assignments (Default Use Case)
```bash
# ~/.config/kmarkdownify/config
MODEL=mistralai/pixtral-large-latest
TEMPERATURE=0.1
MAX_TOKENS=8000
EXTRACT_METADATA=true
METADATA_FIELDS="Title,Author,Course,Due Date,Student ID,Professor"
# No custom prompt needed - uses intelligent default
```

#### For Books/Long Documents
```bash
# ~/.config/kmarkdownify/config
MODEL=mistralai/pixtral-large-latest
TEMPERATURE=0.15
MAX_TOKENS=16000
EXTRACT_METADATA=true
METADATA_FIELDS="Title,Author,Publisher,Publication Year,ISBN"
CUSTOM_PROMPT="Please extract all text from this PDF book/document and convert it to clean Markdown. Preserve chapter structure, headings, paragraphs, quotes, and lists. Maintain readability and proper hierarchy."
```

#### For Forms/Structured Data
```bash
# ~/.config/kmarkdownify/config
MODEL=mistralai/pixtral-large-latest
TEMPERATURE=0.05
MAX_TOKENS=8000
EXTRACT_METADATA=false
CUSTOM_PROMPT="Please extract all text and structured data from this PDF form and convert it to Markdown tables where appropriate. Preserve form fields, labels, and the overall structure. Format data fields as key-value pairs or tables."
```

## Batch Processing

To process multiple PDFs at once, create a simple wrapper script:

**batch-convert.sh:**
```bash
#!/bin/bash
for pdf in "$@"; do
    echo "Converting: $pdf"
    /usr/local/bin/kmarkdownify.sh "$pdf"
done
```

Usage:
```bash
chmod +x batch-convert.sh
./batch-convert.sh *.pdf
```

## Integration with Other Tools

### Using with Obsidian

```bash
# Convert PDFs directly to your Obsidian vault
OUTPUT_DIR="$HOME/Documents/ObsidianVault/PDFs"
/usr/local/bin/kmarkdownify.sh input.pdf
mv input.md "$OUTPUT_DIR/"
```

### Using with Git

Add a pre-commit hook to convert PDFs in your repository:

```bash
# .git/hooks/pre-commit
#!/bin/bash
for pdf in $(git diff --cached --name-only --diff-filter=A | grep '.pdf$'); do
    /usr/local/bin/kmarkdownify.sh "$pdf"
    md_file="${pdf%.pdf}.md"
    git add "$md_file"
done
```

## Cost Management

### Monitor Your Usage

- Check OpenRouter dashboard regularly
- Set up billing alerts
- Use the free tier for testing

### Optimize Costs

1. Start with smaller documents to test
2. Use appropriate models for the task
3. Don't re-convert files unnecessarily
4. Consider batch processing during off-peak times

## Support

For more help:
- Check the main [README](../README.md)
- Open an issue on [GitHub](https://github.com/profiluefter/KMarkdownify/issues)
- Review OpenRouter documentation
