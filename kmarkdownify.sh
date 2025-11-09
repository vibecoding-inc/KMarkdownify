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

# A global variable to store the ID of the persistent notification.
# Do not touch this variable directly; use the helper functions.
_NOTIF_ID=""
_NOTIF_TITLE="" # Store the title for updates

# ---
# Displays the initial persistent loading notification.
#
# @param $1 {string} The title for the notification.
# @param $2 {string} The initial message/body.
# ---
notif_show_loading() {
    local title="$1"
    local message="$2"
    
    echo -e "${YELLOW}$message${NC}"
    
    # Store title for updates
    _NOTIF_TITLE="$title"

    # Send the notification and capture its ID
    # -t 0 = persistent
    # -i "process-working" = loading icon
    # -p = print the ID
    if command -v notify-send &> /dev/null; then
        _NOTIF_ID=$(notify-send "$title" "$message" -t 0 -i "process-working" -p 2>/dev/null || echo "")
    fi
    
    # Set up a trap to automatically call 'notif_error' if the script
    # exits unexpectedly (e.g., Ctrl-C or an 'exit 1')
    # This ensures the loading notification is always removed.
    trap 'notif_error "Script interrupted or failed." > /dev/null 2>&1' EXIT SIGHUP SIGINT SIGTERM
}

# ---
# Updates the text of the *existing* loading notification.
# Keeps the same title and loading icon.
#
# @param $1 {string} The new message/body to display.
# ---
notif_update() {
    local message="$1"
    
    echo -e "${YELLOW}$message${NC}"
    
    # Only run if the notification ID exists
    if [[ -n "$_NOTIF_ID" ]] && command -v notify-send &> /dev/null; then
        # Replace the notification with the new message
        # We keep -t 0 and the icon to show it's still loading.
        notify-send -r "$_NOTIF_ID" "$_NOTIF_TITLE" "$message" -t 0 -i "process-working" 2>/dev/null || true
    fi
}

# ---
# Finishes the notification using the "replace" method.
# It replaces the persistent notification with a new, temporary one.
#
# @param $1 {string} The *new* title for the final status.
# @param $2 {string} The *new* message for the final status.
# @param $3 {string} The icon to use (e.g., "dialog-ok-apply").
# @param $4 {number} Optional: Timeout in ms (default: 3000).
# ---
notif_finish() {
    local title="$1"
    local message="$2"
    local icon="$3"
    local timeout="${4:-3000}"

    # Disable the trap first, since we are exiting cleanly.
    # '0' is the signal for a clean exit.
    trap - 0 EXIT SIGHUP SIGINT SIGTERM

    # Only run if the notification ID exists
    if [[ -n "$_NOTIF_ID" ]] && command -v notify-send &> /dev/null; then
        # Replace the persistent notification with a new temporary one.
        notify-send -r "$_NOTIF_ID" "$title" "$message" -i "$icon" -t "$timeout" 2>/dev/null || true
        
        # Clear the ID
        _NOTIF_ID=""
        _NOTIF_TITLE=""
    elif command -v notify-send &> /dev/null; then
        # Fallback if the notification was never shown
        notify-send "$title" "$message" -i "$icon" -t "$timeout" 2>/dev/null || true
    fi
}

# ---
# Convenience function for a success message.
#
# @param $1 {string} The final success message.
# ---
notif_success() {
    local message="${1:-Task finished successfully.}"
    echo -e "${GREEN}$message${NC}"
    notif_finish "Task Complete" "$message" "dialog-ok-apply"
}

# ---
# Convenience function for an error message.
#
# @param $1 {string} The final error message.
# ---
notif_error() {
    local message="${1:-An error occurred.}"
    echo -e "${RED}Error: $message${NC}" >&2
    # Use a longer timeout for errors
    notif_finish "Task Failed" "$message" "dialog-error" 5000
}

# Function to show error and exit
error_exit() {
    notif_error "$1"
    exit 1
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
PDF_FILENAME=$(basename "$PDF_FILE")
PDF_DIR=$(dirname "$PDF_FILE")
OUTPUT_FILE="${PDF_DIR}/${PDF_BASENAME}.md"

# Check if output file already exists
if [ -f "$OUTPUT_FILE" ]; then
    if ! kdialog --yesno "Output file already exists: $OUTPUT_FILE\n\nDo you want to overwrite it?" --title "KMarkdownify" 2>/dev/null; then
        # User cancelled - show info and exit cleanly
        if command -v notify-send &> /dev/null; then
            notify-send "KMarkdownify" "Conversion cancelled by user." -i "dialog-information" 2>/dev/null || true
        fi
        echo -e "${GREEN}Conversion cancelled by user.${NC}"
        exit 0
    fi
fi

notif_show_loading "KMarkdownify" "Converting PDF to Markdown...\nThis may take a moment."

# Convert PDF to base64 and store in temporary file
# Use temp file to avoid storing large data in environment variables
notif_update "Encoding PDF file..."
PDF_BASE64_FILE=$(mktemp)
# Note: trap is already set by notif_show_loading, we just need to clean up temp files
trap 'notif_error "Script interrupted or failed." > /dev/null 2>&1; rm -f "$PDF_BASE64_FILE" "$TMPFILE"' EXIT INT TERM SIGHUP
base64 -w 0 "$PDF_FILE" > "$PDF_BASE64_FILE"

# Create JSON payload for API request
notif_update "Preparing API request..."

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

IMPORTANT: Transcribe the content EXACTLY as it appears in the document, including any typos, spelling errors, or grammatical mistakes. Do not correct or modify the actual text content.

You may improve the formatting by:
- Using appropriate Markdown syntax (headings, lists, tables, etc.)
- Adding code blocks for code snippets
- Using emphasis (bold, italic) for highlighted text
- Preserving document structure

Format your response as follows:
1. Start with YAML frontmatter containing the metadata (enclosed in --- delimiters)
2. Follow with the main content in Markdown format

Only return the formatted output without any explanations or additional commentary.

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
        PROMPT_TEXT="Please extract all text content from this PDF document and convert it to clean, well-formatted Markdown.

IMPORTANT: Transcribe the content EXACTLY as it appears in the document, including any typos, spelling errors, or grammatical mistakes. Do not correct or modify the actual text content.

You may improve the formatting by:
- Using appropriate Markdown syntax (headings, lists, tables, etc.)
- Adding code blocks for code snippets
- Using emphasis (bold, italic) for highlighted text
- Preserving document structure

Only return the Markdown text without any explanations or additional commentary."
    fi
fi

# Use a more robust method to build JSON payload that avoids argument list length limits
# We write the JSON payload to a temporary file to avoid both command-line and environment variable size limits
TMPFILE=$(mktemp)
# Update trap to include new temp file
trap 'notif_error "Script interrupted or failed." > /dev/null 2>&1; rm -f "$PDF_BASE64_FILE" "$TMPFILE"' EXIT INT TERM SIGHUP

jq -Rs \
    --arg model "$MODEL" \
    --arg prompt "$PROMPT_TEXT" \
    --arg temperature "$TEMPERATURE" \
    --argjson max_tokens "$MAX_TOKENS" \
    --arg filename "$PDF_FILENAME" \
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
                        "type": "file",
                        "file": {
                            "filename": $filename,
                            "file_data": ("data:application/pdf;base64," + .)
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
notif_update "Sending request to OpenRouter API..."
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
notif_update "Extracting markdown content..."
MARKDOWN_CONTENT=$(echo "$RESPONSE" | jq -r '.choices[0].message.content // empty')

if [ -z "$MARKDOWN_CONTENT" ]; then
    error_exit "Failed to extract content from API response. The response may be empty or malformed."
fi

# Save to output file
notif_update "Saving to file: $OUTPUT_FILE"
echo "$MARKDOWN_CONTENT" > "$OUTPUT_FILE"

# Show success notification (this will also clear the trap and close the loading notification)
notif_success "✓ Conversion complete!\n\nSaved to: $OUTPUT_FILE"

exit 0
