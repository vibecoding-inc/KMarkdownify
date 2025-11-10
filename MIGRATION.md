# Migration Guide: v2.x to v3.0

This guide helps users migrate from KMarkdownify v2.x (bash script) to v3.0 (Rust binary).

## Overview

Version 3.0 is a complete rewrite in Rust that maintains full backward compatibility with v2.x configuration and functionality. Your existing configuration files will continue to work without any changes.

## What's Changed

### For End Users

**Good News**: Nothing changes for your workflow!

- ✅ Configuration files remain compatible
- ✅ API key location unchanged
- ✅ Same config file format
- ✅ Same Dolphin integration
- ✅ All features work identically
- ✅ Same prompt files supported

### For System Administrators

**Installation Changes**:

1. **Binary Location**: Still `/usr/local/bin/`, but now installs `kmarkdownify` (binary) instead of `kmarkdownify.sh` (script)

2. **Dependencies Changed**:
   - **Removed**: bash, curl, jq
   - **Added**: rust, cargo (build time only)
   - **Runtime**: Only needs `file` and `dbus`

3. **PKGBUILD Changes**:
   - Architecture changed from `any` to `x86_64 aarch64`
   - Added build step for Rust compilation
   - Binary is now compiled, not just copied

## Migration Steps

### Arch Linux (PKGBUILD)

If you installed via PKGBUILD:

```bash
# 1. Remove old version
sudo pacman -R kmarkdownify

# 2. Update repository
cd KMarkdownify
git pull

# 3. Build and install new version
makepkg -si

# 4. Refresh KDE service menus
kbuildsycoca5 --noincremental

# 5. Restart Dolphin (optional)
killall dolphin
```

**That's it!** Your configuration and API key are preserved.

### Manual Installation

If you installed manually:

```bash
# 1. Navigate to repository
cd KMarkdownify
git pull

# 2. Install Rust if not already installed
# For Arch:
sudo pacman -S rust

# For Debian/Ubuntu:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 3. Build the binary
cargo build --release

# 4. Replace old script with new binary
sudo rm /usr/local/bin/kmarkdownify.sh  # Remove old script
sudo install -Dm755 target/release/kmarkdownify /usr/local/bin/kmarkdownify

# 5. Update service menu (if needed)
sudo install -Dm644 kmarkdownify.desktop /usr/share/kio/servicemenus/kmarkdownify.desktop

# 6. Refresh KDE service menus
kbuildsycoca5 --noincremental

# 7. Restart Dolphin (optional)
killall dolphin
```

## Verification

After migration, verify the installation:

```bash
# 1. Check binary exists and is executable
ls -lh /usr/local/bin/kmarkdownify
# Should show: -rwxr-xr-x ... /usr/local/bin/kmarkdownify

# 2. Check version (try running with no args)
/usr/local/bin/kmarkdownify
# Should show: Error: No PDF file provided. Usage: kmarkdownify <pdf-file>

# 3. Check configuration still exists
ls -la ~/.config/kmarkdownify/
# Should show: api_key and possibly config

# 4. Test with a PDF file
/usr/local/bin/kmarkdownify /path/to/test.pdf
```

## Configuration Compatibility

All v2.x configuration options are fully supported in v3.0:

| Config Option | v2.x Support | v3.0 Support | Notes |
|---------------|--------------|--------------|-------|
| `MODEL` | ✅ | ✅ | Same format |
| `TEMPERATURE` | ✅ | ✅ | Same format |
| `MAX_TOKENS` | ✅ | ✅ | Same format |
| `EXTRACT_METADATA` | ✅ | ✅ | Same format |
| `METADATA_FIELDS` | ✅ | ✅ | Same format |
| `CUSTOM_PROMPT` | ✅ | ✅ | Same format |
| `SYSTEM_PROMPT_FILE` | ✅ | ✅ | Same format |

**Example config file** (works in both v2.x and v3.0):
```bash
# ~/.config/kmarkdownify/config
MODEL=mistralai/pixtral-large-latest
TEMPERATURE=0.1
MAX_TOKENS=8000
EXTRACT_METADATA=true
METADATA_FIELDS=Title,Author,Course,Due Date
```

## Troubleshooting

### "Command not found" after upgrade

**Problem**: Shell can't find the new binary.

**Solution**: Make sure the binary is installed at the correct location:
```bash
sudo install -Dm755 target/release/kmarkdownify /usr/local/bin/kmarkdownify
```

### Service menu shows old command

**Problem**: Desktop file still references old script.

**Solution**: Update the desktop file:
```bash
sudo install -Dm644 kmarkdownify.desktop /usr/share/kio/servicemenus/kmarkdownify.desktop
kbuildsycoca5 --noincremental
```

### "Required command not found: curl"

**Problem**: Error message from old bash script still running.

**Solution**: Make sure old script is completely removed:
```bash
sudo rm /usr/local/bin/kmarkdownify.sh
which kmarkdownify  # Should show /usr/local/bin/kmarkdownify
```

### Binary won't run on my architecture

**Problem**: Binary compiled for wrong architecture.

**Solution**: Rebuild for your architecture:
```bash
cargo clean
cargo build --release
sudo install -Dm755 target/release/kmarkdownify /usr/local/bin/kmarkdownify
```

### Missing Rust toolchain

**Problem**: Don't have Rust installed for building.

**Solution**: Install Rust:
```bash
# Official method (all distributions):
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Or via package manager:
# Arch: sudo pacman -S rust
# Debian/Ubuntu: sudo apt install cargo rustc
# Fedora: sudo dnf install rust cargo
```

## Rollback to v2.x

If you need to rollback for any reason:

```bash
# 1. Remove v3.0 binary
sudo rm /usr/local/bin/kmarkdownify

# 2. Checkout v2.x version
git checkout v2.1.0  # or appropriate v2.x tag

# 3. Install old script
sudo install -Dm755 kmarkdownify.sh /usr/local/bin/kmarkdownify.sh

# 4. Update desktop file
sudo install -Dm644 kmarkdownify.desktop /usr/share/kio/servicemenus/kmarkdownify.desktop

# 5. Refresh service menus
kbuildsycoca5 --noincremental
```

Your configuration files (`~/.config/kmarkdownify/`) will work with v2.x without modification.

## Benefits of Upgrading

**Why upgrade to v3.0?**

1. **Security**: Memory-safe implementation eliminates buffer overflows, use-after-free, and shell injection vulnerabilities
2. **Performance**: Compiled binary is faster than interpreted shell script
3. **Reliability**: Type-safe code catches many bugs at compile time
4. **Maintainability**: Easier to maintain and extend in the future
5. **Fewer Dependencies**: No longer requires bash, curl, or jq at runtime
6. **Better Error Messages**: More detailed and helpful error reporting

## Support

If you encounter issues during migration:

1. Check this migration guide
2. Review [TROUBLESHOOTING.md](TROUBLESHOOTING.md) if it exists
3. Check [README.md](README.md) for updated installation instructions
4. Open an issue on GitHub: https://github.com/profiluefter/KMarkdownify/issues

## Summary

**For most users**, migration is seamless:
- Run `makepkg -si` (Arch) or rebuild and reinstall manually
- Your configuration continues to work
- No changes to workflow needed

**For advanced users**, you get:
- Improved security
- Better performance
- More reliable operation
- Same functionality and features

Enjoy the new Rust-powered KMarkdownify v3.0! 🦀
