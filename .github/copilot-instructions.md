# GitHub Copilot Instructions for KMarkdownify

## Project Overview

KMarkdownify is a KDE Plasma Dolphin service menu integration that converts PDF files to Markdown format using the OpenRouter API with various AI models. The project is implemented in Rust, designed to be lightweight, secure, and easy to install on Linux distributions, with special support for Arch Linux via PKGBUILD.

## Architecture

The project consists of three main components:

1. **src/main.rs** - Rust application that handles PDF conversion via OpenRouter API with streaming support
2. **kmarkdownify.desktop** - A KDE service menu entry for Dolphin file manager integration
3. **PKGBUILD** - An Arch Linux package build script for easy installation

## Code Style and Standards

### Rust Development
- Follow Rust idioms and best practices
- Use `anyhow` for error handling with context
- Use `tokio` for async runtime
- Implement proper error handling with `Result` types
- Use structured logging with `println!` and `eprintln!`
- Run `cargo fmt` before committing
- Run `cargo clippy` to catch common mistakes
- Use `serde` for JSON serialization/deserialization
- Validate all inputs before processing
- Never log, display, or commit API keys

### Documentation
- Maintain comprehensive documentation in README.md and config.example
- Keep man page (kmarkdownify.1) updated with current features
- Include troubleshooting sections for common issues
- Provide examples for all major use cases

## Development Guidelines

### When Making Changes

1. **Rust Code Changes (src/main.rs)**
   - Run `cargo build` to check for compilation errors
   - Run `cargo fmt` to format code
   - Run `cargo clippy` for linting suggestions
   - Test with various PDF files (small, large, corrupted)
   - Ensure backward compatibility with configuration files
   - Test both streaming and error handling paths

2. **Service Menu Changes (kmarkdownify.desktop)**
   - Validate desktop file format with `desktop-file-validate`
   - Test in both KDE 5 and KDE 6/Plasma 6 environments if possible
   - Verify MIME type filters work correctly
   - Test right-click menu appearance in Dolphin

3. **Package Changes (PKGBUILD)**
   - Follow Arch Linux packaging guidelines
   - Update version numbers and checksums
   - Test installation: `makepkg -si`
   - Verify all files are installed to correct locations
   - Ensure Rust toolchain is available during build

### Dependencies

**Build Dependencies:**
- rust (1.70+)
- cargo

**Runtime Dependencies:**
- file (MIME type detection)
- poppler-utils (optional, for accurate page counting)

**Rust Crate Dependencies:**
- reqwest (HTTP client with streaming)
- serde & serde_json (JSON handling)
- base64 (PDF encoding)
- anyhow (error handling)
- notify-rust (desktop notifications)
- dirs (config directory detection)
- tokio (async runtime)
- eventsource-stream (SSE streaming)
- futures-util (async utilities)

## Security Considerations

### API Key Management
- API keys must be stored in `~/.config/kmarkdownify/api_key`
- Recommended permissions: 600 (user read/write only)
- Never log, display, or commit API keys
- Transmit only via HTTPS Authorization header

### Input Validation
- Always validate file existence and type before processing
- Use `file` command to verify PDF MIME type
- Properly handle all paths with Rust's PathBuf
- Use `serde_json` for safe JSON construction

### Network Security
- Use HTTPS-only communication
- Include proper HTTP headers (Referer, X-Title)
- Never include sensitive data in URLs or query parameters
- Handle streaming errors gracefully

## Testing

### Before Committing
1. Run build: `cargo build`
2. Run formatter: `cargo fmt`
3. Run linter: `cargo clippy`
4. Test error cases:
   - No arguments provided
   - Non-existent file
   - Non-PDF file
   - Missing API key
   - Empty API key file
5. Test successful conversion with a sample PDF
6. Test streaming with large PDFs

### Manual Testing
```bash
# Build in release mode
cargo build --release

# Test command line usage
./target/release/kmarkdownify /path/to/test.pdf

# Test solve mode
./target/release/kmarkdownify --solve /path/to/homework.pdf

# Verify output
cat /path/to/test.md

# Test service menu (requires KDE/Dolphin)
# Right-click on PDF in Dolphin → KMarkdownify → Convert to Markdown
```

## API Integration

### OpenRouter API Details
- **Endpoint:** `https://openrouter.ai/api/v1/chat/completions`
- **Streaming:** Enabled with `stream: true` parameter
- **Default Model:** `mistralai/pixtral-large-latest`
- **Input Format:** Base64-encoded PDF in file field
- **Temperature:** 0.1 (low for consistent OCR)
- **Max Tokens:** Calculated as pages × MAX_TOKENS_PER_PAGE

### Streaming Implementation
The application uses Server-Sent Events (SSE) streaming:
- Writes content to file in real-time as it's received
- Updates notifications with progress (word count, reasoning)
- Handles partial failures gracefully
- Preserves partial output on errors

### Model Selection
- **Convert Mode:** Uses `OCR_MODEL` or falls back to `MODEL`
  - Optimized for fast, accurate document conversion
  - Example: `google/gemini-2.0-flash-lite`
  
- **Solve Mode:** Uses `REASONING_MODEL` or falls back to `MODEL`
  - Optimized for problem-solving and reasoning
  - Example: `openai/gpt-4o-mini`, `deepseek/deepseek-r1`

### Request Structure
The application sends a POST request with:
- Authorization: Bearer token
- Content-Type: application/json
- HTTP-Referer: GitHub repository URL
- X-Title: KMarkdownify
- stream: true (for streaming responses)

### Response Handling
- Parse SSE stream events
- Extract content and reasoning_content from deltas
- Update notification with reasoning headings when available
- Write content incrementally to output file
- Handle [DONE] event to close stream

### Error Handling
- Check for API error responses in JSON
- Extract error messages from `.error.message`
- Provide user-friendly error notifications
- Preserve partial output on stream errors
- Exit gracefully with appropriate exit codes

## File Locations

### Installation Paths
- Binary: `/usr/bin/kmarkdownify` (mode 755)
- Service menu (system-wide): `/usr/share/kio/servicemenus/kmarkdownify.desktop`
- Service menu (user-only): `~/.local/share/kio/servicemenus/kmarkdownify.desktop`
- Documentation: `/usr/share/doc/kmarkdownify/`
- Prompt files: `/usr/share/kmarkdownify/prompts/`
- Man page: `/usr/share/man/man1/kmarkdownify.1.gz`

### Configuration
- API key: `~/.config/kmarkdownify/api_key`
- Config file: `~/.config/kmarkdownify/config`

## Common Tasks

### Adding New Features
1. Update the main Rust code (src/main.rs)
2. Add documentation to README.md
3. Update config.example with new options
4. Update man page (kmarkdownify.1)
5. Test thoroughly before committing
6. Update version in PKGBUILD and Cargo.toml if needed

### Fixing Bugs
1. Identify the root cause in src/main.rs
2. Add validation or error handling as needed
3. Test the fix with various scenarios
4. Update troubleshooting section in documentation

### Improving Documentation
1. Keep README.md user-focused and concise
2. Keep config.example comprehensive with examples
3. Update man page for detailed reference
4. Include examples for all features

## Distribution Support

### Primary Support
- Arch Linux (via PKGBUILD)

### Manual Installation Support
- Any Linux distribution with:
  - Rust toolchain (for building)
  - KDE Plasma with Dolphin
  - Standard utilities (file, curl)

## Future Enhancements to Consider

When suggesting new features, consider:
- Batch processing multiple PDFs
- Enhanced progress indicators
- Custom output path selection
- Additional model provider support
- Quality/parameter adjustments per mode
- Advanced retry logic for API failures
- Caching to avoid re-processing
- GUI configuration tool
- Language detection and optimization
- Page selection for partial conversion

## References

- [KDE Service Menus Documentation](https://userbase.kde.org/Dolphin/File_Management#Service_Menus)
- [OpenRouter API Documentation](https://openrouter.ai/docs)
- [OpenRouter Streaming Documentation](https://openrouter.ai/docs/api-reference/streaming)
- [Arch Linux PKGBUILD Manual](https://wiki.archlinux.org/title/PKGBUILD)
- [Rust Best Practices](https://rust-lang.github.io/api-guidelines/)

## Notes for AI Assistants

When working on this repository:
- This is a Rust project using async/await with tokio
- Focus on Rust best practices and memory safety
- Test changes manually with actual PDF files when possible
- Consider cross-distribution compatibility
- Maintain the simplicity and lightweight nature of the tool
- Always run `cargo fmt` and `cargo clippy` before committing
- Remember that this integrates with KDE Plasma, so consider KDE-specific behaviors
- Streaming is now the primary mode of operation
- Support both OCR and reasoning model configurations
- Handle reasoning tokens appropriately for compatible models
