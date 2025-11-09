#!/bin/bash
#
# KMarkdownify - PDF to Markdown converter using OpenRouter API with Mistral OCR
# Usage: kmarkdownify.sh <pdf-file>
#

set -euo pipefail

# Configuration
API_ENDPOINT="https://openrouter.ai/api/v1/chat/completions"
MODEL="mistralai/pixtral-large-latest"
API_KEY_FILE="${HOME}/.config/kmarkdownify/api_key"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to show error and exit
error_exit() {
    echo -e "${RED}Error: $1${NC}" >&2
    kdialog --error "$1" --title "KMarkdownify Error" 2>/dev/null || notify-send "KMarkdownify Error" "$1"
    exit 1
}

# Function to show info message
info_message() {
    echo -e "${GREEN}$1${NC}"
    kdialog --passivepopup "$1" 5 --title "KMarkdownify" 2>/dev/null || notify-send "KMarkdownify" "$1"
}

# Function to show warning message
warning_message() {
    echo -e "${YELLOW}$1${NC}"
}

# Check if file argument is provided
if [ $# -eq 0 ]; then
    error_exit "No PDF file provided. Usage: kmarkdownify.sh <pdf-file>"
fi

PDF_FILE="$1"

# Check if file exists
if [ ! -f "$PDF_FILE" ]; then
    error_exit "File not found: $PDF_FILE"
fi

# Check if file is a PDF
if ! file "$PDF_FILE" | grep -q "PDF"; then
    error_exit "File is not a PDF: $PDF_FILE"
fi

# Check for required commands
for cmd in curl base64 jq; do
    if ! command -v "$cmd" &> /dev/null; then
        error_exit "Required command not found: $cmd. Please install it first."
    fi
done

# Check for API key
if [ ! -f "$API_KEY_FILE" ]; then
    error_exit "API key not found. Please create file: $API_KEY_FILE\n\nYou can get an API key from: https://openrouter.ai/keys"
fi

API_KEY=$(tr -d '[:space:]' < "$API_KEY_FILE")

if [ -z "$API_KEY" ]; then
    error_exit "API key file is empty: $API_KEY_FILE"
fi

# Generate output filename
PDF_BASENAME=$(basename "$PDF_FILE" .pdf)
PDF_DIR=$(dirname "$PDF_FILE")
OUTPUT_FILE="${PDF_DIR}/${PDF_BASENAME}.md"

# Check if output file already exists
if [ -f "$OUTPUT_FILE" ]; then
    if ! kdialog --yesno "Output file already exists: $OUTPUT_FILE\n\nDo you want to overwrite it?" --title "KMarkdownify" 2>/dev/null; then
        info_message "Conversion cancelled by user."
        exit 0
    fi
fi

info_message "Converting PDF to Markdown...\nThis may take a moment."

# Convert PDF to base64
echo "Encoding PDF file..."
PDF_BASE64=$(base64 -w 0 "$PDF_FILE")

# Create JSON payload for API request
echo "Preparing API request..."
JSON_PAYLOAD=$(jq -n \
    --arg model "$MODEL" \
    --arg pdf_data "data:application/pdf;base64,$PDF_BASE64" \
    '{
        "model": $model,
        "messages": [
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Please extract all text content from this PDF document and convert it to clean, well-formatted Markdown. Preserve the document structure including headings, paragraphs, lists, tables, and emphasis. Only return the Markdown text without any explanations or additional commentary."
                    },
                    {
                        "type": "image_url",
                        "image_url": {
                            "url": $pdf_data
                        }
                    }
                ]
            }
        ],
        "temperature": 0.1,
        "max_tokens": 8000
    }'
)

# Make API request
echo "Sending request to OpenRouter API..."
RESPONSE=$(curl -s -X POST "$API_ENDPOINT" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $API_KEY" \
    -H "HTTP-Referer: https://github.com/profiluefter/KMarkdownify" \
    -H "X-Title: KMarkdownify" \
    -d "$JSON_PAYLOAD")

# Check for API errors
if echo "$RESPONSE" | jq -e '.error' > /dev/null 2>&1; then
    ERROR_MSG=$(echo "$RESPONSE" | jq -r '.error.message // .error')
    error_exit "API Error: $ERROR_MSG"
fi

# Extract markdown content from response
echo "Extracting markdown content..."
MARKDOWN_CONTENT=$(echo "$RESPONSE" | jq -r '.choices[0].message.content // empty')

if [ -z "$MARKDOWN_CONTENT" ]; then
    error_exit "Failed to extract content from API response. The response may be empty or malformed."
fi

# Save to output file
echo "Saving to file: $OUTPUT_FILE"
echo "$MARKDOWN_CONTENT" > "$OUTPUT_FILE"

# Success message
info_message "✓ Conversion complete!\n\nSaved to: $OUTPUT_FILE"

echo -e "${GREEN}Conversion successful!${NC}"
echo "Output saved to: $OUTPUT_FILE"

exit 0
