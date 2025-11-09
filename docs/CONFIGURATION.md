# KMarkdownify Configuration Examples

This directory contains example configurations and tips for using KMarkdownify.

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

## Customizing the Conversion Prompt

If you want to customize how the PDF is converted to Markdown, you can modify the prompt in `kmarkdownify.sh`.

Find this section in the script:
```bash
"text": "Please extract all text content from this PDF document..."
```

### Example Custom Prompts

**For Technical Documentation:**
```
"Please extract all text from this PDF technical document and convert it to well-formatted Markdown. Preserve code blocks, technical terms, API references, and maintain proper formatting for tables and lists. Use appropriate Markdown syntax for code (```), headings (#), and technical notation."
```

**For Academic Papers:**
```
"Please extract all text from this academic PDF and convert it to Markdown. Preserve the abstract, sections, subsections, citations, references, figures, and tables. Maintain the academic structure and formatting conventions."
```

**For Books/Long Documents:**
```
"Please extract all text from this PDF book/document and convert it to clean Markdown. Preserve chapter structure, headings, paragraphs, quotes, and lists. Maintain readability and proper hierarchy."
```

**For Forms/Structured Data:**
```
"Please extract all text and structured data from this PDF form and convert it to Markdown tables where appropriate. Preserve form fields, labels, and the overall structure."
```

## Model Configuration

The default model is `mistralai/pixtral-large-latest`. You can change this in the script:

```bash
MODEL="mistralai/pixtral-large-latest"
```

### Alternative Models on OpenRouter

Other models you might try (check OpenRouter for availability and pricing):
- `anthropic/claude-3-opus` - Very accurate, higher cost
- `openai/gpt-4-vision-preview` - Good OCR, moderate cost
- `google/gemini-pro-vision` - Google's vision model

**Note:** Different models have different capabilities and costs. Pixtral Large is optimized for OCR tasks.

## API Request Parameters

You can modify these parameters in the script's JSON payload:

```bash
"temperature": 0.1,     # Lower = more deterministic, higher = more creative
"max_tokens": 8000      # Maximum response length
```

### Temperature Guidelines
- `0.0-0.2`: Very consistent, literal extraction (recommended for OCR)
- `0.3-0.7`: Balanced (default for most conversions)
- `0.8-1.0`: More creative interpretation (use cautiously)

### Max Tokens
- Adjust based on document size
- 8000 tokens ≈ 6000 words
- Increase for very long documents
- Check your model's token limits

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
