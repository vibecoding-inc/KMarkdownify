# GitHub Copilot Instructions for KMarkdownify

## Project Overview

KMarkdownify is a KDE Plasma Dolphin service menu integration that converts PDF files to Markdown format using the OpenRouter API with Mistral's Pixtral Large OCR model. The project is designed to be lightweight, secure, and easy to install on Linux distributions, with special support for Arch Linux via PKGBUILD.

## Architecture

The project consists of three main components:

1. **kmarkdownify.sh** - A Bash script that handles PDF conversion via the OpenRouter API
2. **kmarkdownify.desktop** - A KDE service menu entry for Dolphin file manager integration
3. **PKGBUILD** - An Arch Linux package build script for easy installation

## Code Style and Standards

### Bash Scripting
- Use strict error handling: `set -euo pipefail`
- Quote all variable expansions to prevent word splitting
- Use functions for error handling and notifications with dual fallback (kdialog → notify-send)
- Validate all inputs before processing
- Use `jq` for JSON construction to prevent injection attacks
- Never use `eval` or command substitution with user input
- Disable pagers in commands (e.g., `git --no-pager`)

### Documentation
- Maintain comprehensive documentation in README.md, INSTALL.md, and QUICKSTART.md
- Keep IMPLEMENTATION_NOTES.md updated with technical decisions and architecture changes
- Include troubleshooting sections for common issues
- Provide examples for all major use cases

## Development Guidelines

### When Making Changes

1. **Script Changes (kmarkdownify.sh)**
   - Test with `bash -n kmarkdownify.sh` for syntax errors
   - Run `shellcheck kmarkdownify.sh` for best practices
   - Verify error handling paths work correctly
   - Test with various PDF files (small, large, corrupted)
   - Ensure backward compatibility with API key configuration

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

### Dependencies

**Required:**
- bash (4.0+)
- curl (7.x+)
- jq (1.5+)
- coreutils (base64)
- file (MIME type detection)

**Optional:**
- kdialog (KDE notifications)
- libnotify (fallback notifications)

## Security Considerations

### API Key Management
- API keys must be stored in `~/.config/kmarkdownify/api_key`
- Recommended permissions: 600 (user read/write only)
- Never log, display, or commit API keys
- Transmit only via HTTPS Authorization header

### Input Validation
- Always validate file existence and type before processing
- Use `file` command to verify PDF MIME type
- Properly quote all file paths
- Use `jq` for JSON construction, never string interpolation

### Network Security
- Use HTTPS-only communication
- Include proper HTTP headers (Referer, X-Title)
- Never include sensitive data in URLs or query parameters

## Testing

### Before Committing
1. Run syntax validation: `bash -n kmarkdownify.sh`
2. Run shellcheck: `shellcheck kmarkdownify.sh`
3. Test error cases:
   - No arguments provided
   - Non-existent file
   - Non-PDF file
   - Missing API key
   - Empty API key file
4. Test successful conversion with a sample PDF

### Manual Testing
```bash
# Test command line usage
./kmarkdownify.sh /path/to/test.pdf

# Verify output
cat /path/to/test.md

# Test service menu (requires KDE/Dolphin)
# Right-click on PDF in Dolphin → KMarkdownify → Convert to Markdown
```

## API Integration

### OpenRouter API Details
- **Endpoint:** `https://openrouter.ai/api/v1/chat/completions`
- **Model:** `mistralai/pixtral-large-latest`
- **Input Format:** Base64-encoded PDF in image_url field
- **Temperature:** 0.1 (low for consistent OCR)
- **Max Tokens:** 8000

### Request Structure
The script sends a POST request with:
- Authorization: Bearer token
- Content-Type: application/json
- HTTP-Referer: GitHub repository URL
- X-Title: KMarkdownify

### Error Handling
- Check for API error responses in JSON
- Extract error messages from `.error.message` or `.error`
- Provide user-friendly error notifications
- Exit gracefully with appropriate exit codes

## File Locations

### Installation Paths
- Script: `/usr/local/bin/kmarkdownify.sh` (mode 755)
- Service menu (KDE 5): `/usr/share/kservices5/ServiceMenus/kmarkdownify.desktop`
- Service menu (KDE 6): `~/.local/share/kio/servicemenus/kmarkdownify.desktop`
- Documentation: `/usr/share/doc/kmarkdownify/`

### Configuration
- API key: `~/.config/kmarkdownify/api_key`

## Common Tasks

### Adding New Features
1. Update the main script (kmarkdownify.sh)
2. Add documentation to README.md
3. Update IMPLEMENTATION_NOTES.md with technical details
4. Test thoroughly before committing
5. Update version in PKGBUILD if needed

### Fixing Bugs
1. Identify the root cause in kmarkdownify.sh
2. Add validation or error handling as needed
3. Test the fix with various scenarios
4. Update troubleshooting section in documentation

### Improving Documentation
1. Keep README.md user-focused and concise
2. Put technical details in IMPLEMENTATION_NOTES.md
3. Update QUICKSTART.md for getting started quickly
4. Keep INSTALL.md comprehensive for all distributions

## Distribution Support

### Primary Support
- Arch Linux (via PKGBUILD)

### Manual Installation Support
- Debian/Ubuntu
- Fedora
- openSUSE
- Any Linux distribution with KDE Plasma

## Future Enhancements to Consider

When suggesting new features, consider:
- Batch processing multiple PDFs
- Progress indicators for large files
- Custom output path selection
- Model selection options
- Quality/parameter adjustments
- Retry logic for API failures
- Caching to avoid re-processing
- GUI configuration tool
- Language detection and optimization
- Page selection for partial conversion

## References

- [KDE Service Menus Documentation](https://userbase.kde.org/Dolphin/File_Management#Service_Menus)
- [OpenRouter API Documentation](https://openrouter.ai/docs)
- [Mistral AI Documentation](https://docs.mistral.ai/)
- [Arch Linux PKGBUILD Manual](https://wiki.archlinux.org/title/PKGBUILD)
- [Bash Best Practices](https://google.github.io/styleguide/shellguide.html)

## Notes for AI Assistants

When working on this repository:
- This is a bash-based project, not a typical software application with unit tests
- Focus on shell scripting best practices and security
- Test changes manually with actual PDF files when possible
- Consider cross-distribution compatibility
- Maintain the simplicity and lightweight nature of the tool
- Always validate bash syntax and run shellcheck before committing
- Remember that this integrates with KDE Plasma, so consider KDE-specific behaviors
