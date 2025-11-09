#!/bin/bash
#
# KMarkdownify - PDF to Markdown converter using OpenRouter API with Mistral OCR
# Usage: kmarkdownify.sh <pdf-file>
#

set -euo pipefail

# Default Configuration
API_ENDPOINT="https://openrouter.ai/api/v1/chat/completions"
CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/kmarkdownify"
CONFIG_FILE="${CONFIG_DIR}/config"
API_KEY_FILE="${CONFIG_DIR}/api_key"

# Determine installation prefix for prompt files
if [ -d "/usr/share/kmarkdownify/prompts" ]; then
    PROMPTS_DIR="/usr/share/kmarkdownify/prompts"
elif [ -d "/usr/local/share/kmarkdownify/prompts" ]; then
    PROMPTS_DIR="/usr/local/share/kmarkdownify/prompts"
else
    # Fallback to script's directory
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    PROMPTS_DIR="${SCRIPT_DIR}/prompts"
fi

# Default values (can be overridden by config file)
MODEL="mistralai/pixtral-large-latest"
TEMPERATURE="0.1"
MAX_TOKENS="8000"
EXTRACT_METADATA="true"
METADATA_FIELDS="Title,Author,Course,Due Date"
CUSTOM_PROMPT=""
SYSTEM_PROMPT_FILE=""

# Load configuration file if it exists
if [ -f "$CONFIG_FILE" ]; then
    # Source the config file, but only allow safe variable assignments
    while IFS='=' read -r key value; do
        # Skip comments and empty lines
        [[ "$key" =~ ^#.*$ ]] && continue
        [[ -z "$key" ]] && continue
        # Remove leading/trailing whitespace
        key=$(echo "$key" | xargs)
        value=$(echo "$value" | xargs)
        # Set variables based on key
        case "$key" in
            MODEL) MODEL="$value" ;;
            TEMPERATURE) TEMPERATURE="$value" ;;
            MAX_TOKENS) MAX_TOKENS="$value" ;;
            EXTRACT_METADATA) EXTRACT_METADATA="$value" ;;
            METADATA_FIELDS) METADATA_FIELDS="$value" ;;
            CUSTOM_PROMPT) CUSTOM_PROMPT="$value" ;;
            SYSTEM_PROMPT_FILE) SYSTEM_PROMPT_FILE="$value" ;;
        esac
    done < "$CONFIG_FILE"
fi

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

# Convert PDF to base64 and store in temporary file
# Use temp file to avoid storing large data in environment variables
echo "Encoding PDF file..."
PDF_BASE64_FILE=$(mktemp)
trap 'rm -f "$PDF_BASE64_FILE" "$TMPFILE"' EXIT
base64 -w 0 "$PDF_FILE" > "$PDF_BASE64_FILE"

# Create JSON payload for API request
echo "Preparing API request..."

# Build the prompt text based on configuration
if [ -n "$CUSTOM_PROMPT" ]; then
    # Use custom prompt if provided inline in config
    PROMPT_TEXT="$CUSTOM_PROMPT"
elif [ -n "$SYSTEM_PROMPT_FILE" ]; then
    # Use custom prompt file if specified
    # Handle both absolute and relative paths
    if [[ "$SYSTEM_PROMPT_FILE" = /* ]]; then
        PROMPT_FILE="$SYSTEM_PROMPT_FILE"
    else
        PROMPT_FILE="${CONFIG_DIR}/${SYSTEM_PROMPT_FILE}"
    fi
    
    if [ ! -f "$PROMPT_FILE" ]; then
        error_exit "System prompt file not found: $PROMPT_FILE"
    fi
    
    PROMPT_TEXT=$(cat "$PROMPT_FILE")
    # Replace {METADATA_FIELDS} placeholder if present
    PROMPT_TEXT="${PROMPT_TEXT//\{METADATA_FIELDS\}/$METADATA_FIELDS}"
elif [ "$EXTRACT_METADATA" = "true" ]; then
    # Use default metadata extraction prompt from installed file
    if [ -f "${PROMPTS_DIR}/metadata_prompt.txt" ]; then
        PROMPT_TEXT=$(cat "${PROMPTS_DIR}/metadata_prompt.txt")
        # Replace {METADATA_FIELDS} placeholder
        PROMPT_TEXT="${PROMPT_TEXT//\{METADATA_FIELDS\}/$METADATA_FIELDS}"
    else
        # Fallback to inline prompt if file not found
        PROMPT_TEXT="Please analyze this PDF document and extract both metadata and content.

First, extract the following metadata fields if present in the document: ${METADATA_FIELDS}. If a field cannot be found, use 'N/A'.

Then, extract all text content from the PDF and convert it to clean, well-formatted Markdown.

Format your response as follows:
1. Start with YAML frontmatter containing the metadata (enclosed in --- delimiters)
2. Follow with the main content in Markdown format

Preserve the document structure including headings, paragraphs, lists, tables, and emphasis. Only return the formatted output without any explanations or additional commentary.

Example format:
---
Title: Document Title or N/A
Author: Author Name or N/A
Course: Course Name or N/A
Due Date: Date or N/A
---

# Document Content Starts Here
..."
    fi
else
    # Use default simple extraction prompt from installed file
    if [ -f "${PROMPTS_DIR}/default_prompt.txt" ]; then
        PROMPT_TEXT=$(cat "${PROMPTS_DIR}/default_prompt.txt")
    else
        # Fallback to inline prompt if file not found
        PROMPT_TEXT="Please extract all text content from this PDF document and convert it to clean, well-formatted Markdown. Preserve the document structure including headings, paragraphs, lists, tables, and emphasis. Only return the Markdown text without any explanations or additional commentary."
    fi
fi

# Use a more robust method to build JSON payload that avoids argument list length limits
# We write the JSON payload to a temporary file to avoid both command-line and environment variable size limits
TMPFILE=$(mktemp)
trap 'rm -f "$PDF_BASE64_FILE" "$TMPFILE"' EXIT

jq -Rs \
    --arg model "$MODEL" \
    --arg prompt "$PROMPT_TEXT" \
    --arg temperature "$TEMPERATURE" \
    --argjson max_tokens "$MAX_TOKENS" \
    '{
        "model": $model,
        "messages": [
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": $prompt
                    },
                    {
                        "type": "image_url",
                        "image_url": {
                            "url": ("data:application/pdf;base64," + .)
                        }
                    }
                ]
            }
        ],
        "temperature": ($temperature | tonumber),
        "max_tokens": $max_tokens
    }' < "$PDF_BASE64_FILE" > "$TMPFILE"

# Make API request
# Use temporary file to avoid argument list length limits with large payloads
echo "Sending request to OpenRouter API..."
RESPONSE=$(curl -s -X POST "$API_ENDPOINT" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $API_KEY" \
    -H "HTTP-Referer: https://github.com/profiluefter/KMarkdownify" \
    -H "X-Title: KMarkdownify" \
    --data-binary @"$TMPFILE")

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
