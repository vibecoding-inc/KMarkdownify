# Installation Guide

This guide provides detailed installation instructions for KMarkdownify on different Linux distributions.

## Table of Contents
- [Arch Linux Installation](#arch-linux-installation)
- [Manual Installation](#manual-installation)
- [Post-Installation Setup](#post-installation-setup)

## Arch Linux Installation

### Prerequisites
Ensure you have the base development tools:
```bash
sudo pacman -S base-devel git
```

### Installation Steps

1. **Clone the repository:**
```bash
git clone https://github.com/profiluefter/KMarkdownify.git
cd KMarkdownify
```

2. **Build and install the package:**
```bash
makepkg -si
```

This will:
- Download and verify dependencies
- Install the package to your system
- Set up the Dolphin service menu automatically

3. **Refresh KDE service menus:**
```bash
kbuildsycoca5 --noincremental
```

4. **Restart Dolphin** (optional but recommended):
```bash
killall dolphin
dolphin &
```

## Manual Installation

### For Debian/Ubuntu

1. **Install dependencies:**
```bash
sudo apt update
sudo apt install bash curl jq coreutils file kdialog libnotify-bin git
```

2. **Clone and install:**
```bash
git clone https://github.com/profiluefter/KMarkdownify.git
cd KMarkdownify

# Install script
sudo install -Dm755 kmarkdownify.sh /usr/local/bin/kmarkdownify.sh

# Install service menu (system-wide)
sudo install -Dm644 kmarkdownify.desktop /usr/share/kio/servicemenus/kmarkdownify.desktop

# Install prompt files
sudo mkdir -p /usr/local/share/kmarkdownify/prompts
sudo install -Dm644 prompts/default_prompt.txt /usr/local/share/kmarkdownify/prompts/default_prompt.txt
sudo install -Dm644 prompts/metadata_prompt.txt /usr/local/share/kmarkdownify/prompts/metadata_prompt.txt

# Install example configuration
sudo mkdir -p /usr/share/doc/kmarkdownify
sudo install -Dm644 config.example /usr/share/doc/kmarkdownify/config.example

# Or for user-only installation:
# mkdir -p ~/.local/share/kio/servicemenus
# cp kmarkdownify.desktop ~/.local/share/kio/servicemenus/
```

3. **Refresh service menus:**
```bash
kbuildsycoca5 --noincremental
```

### For Fedora

1. **Install dependencies:**
```bash
sudo dnf install bash curl jq coreutils file kdialog libnotify git
```

2. **Follow the same installation steps as Debian/Ubuntu above.**

### For openSUSE

1. **Install dependencies:**
```bash
sudo zypper install bash curl jq coreutils file kdialog libnotify-tools git
```

2. **Follow the same installation steps as Debian/Ubuntu above.**

## Post-Installation Setup

### 1. Obtain OpenRouter API Key

1. Visit [OpenRouter Keys](https://openrouter.ai/keys)
2. Sign up for a free account
3. Generate an API key
4. Copy the key for the next step

### 2. Configure API Key

Create the configuration directory and save your API key:

```bash
# Create config directory
mkdir -p ~/.config/kmarkdownify

# Save API key (replace YOUR_API_KEY with your actual key)
echo "YOUR_API_KEY" > ~/.config/kmarkdownify/api_key

# Secure the file
chmod 600 ~/.config/kmarkdownify/api_key
```

**Important:** Keep your API key private and never commit it to version control!

### 3. Verify Installation

Test the installation from command line:

```bash
# This should show the error about needing a PDF file
/usr/local/bin/kmarkdownify.sh

# Test with a PDF file
/usr/local/bin/kmarkdownify.sh /path/to/your/test.pdf
```

## Troubleshooting Installation

### Service menu doesn't appear in Dolphin

**Solution 1:** Refresh the KDE cache
```bash
kbuildsycoca5 --noincremental
killall dolphin
```

**Solution 2:** Check installation path
```bash
# For system-wide installation
ls -l /usr/share/kio/servicemenus/kmarkdownify.desktop

# For user-only installation
ls -l ~/.local/share/kio/servicemenus/kmarkdownify.desktop
```

**Solution 3:** Install for current user only
```bash
mkdir -p ~/.local/share/kio/servicemenus
cp kmarkdownify.desktop ~/.local/share/kio/servicemenus/
kbuildsycoca5 --noincremental
```

### Script not found or permission denied

```bash
# Check if script exists
ls -l /usr/local/bin/kmarkdownify.sh

# Make it executable if needed
sudo chmod +x /usr/local/bin/kmarkdownify.sh

# Check if /usr/local/bin is in your PATH
echo $PATH | grep /usr/local/bin
```

### Missing dependencies

Install any missing dependencies:

```bash
# Check which dependencies are missing
for cmd in curl jq base64 file; do
    command -v $cmd >/dev/null 2>&1 || echo "Missing: $cmd"
done

# Install missing packages based on your distribution
# Arch: sudo pacman -S curl jq coreutils file
# Ubuntu: sudo apt install curl jq coreutils file
# Fedora: sudo dnf install curl jq coreutils file
```

## Uninstallation

### Arch Linux (if installed via PKGBUILD)
```bash
sudo pacman -R kmarkdownify
```

### Manual Installation
```bash
# Remove script
sudo rm /usr/local/bin/kmarkdownify.sh

# Remove service menu (system-wide)
sudo rm /usr/share/kio/servicemenus/kmarkdownify.desktop
# or (user-only)
rm ~/.local/share/kio/servicemenus/kmarkdownify.desktop

# Remove prompt files
sudo rm -rf /usr/local/share/kmarkdownify

# Remove documentation
sudo rm -rf /usr/share/doc/kmarkdownify

# Remove configuration (optional)
rm -rf ~/.config/kmarkdownify

# Refresh service menus
kbuildsycoca5 --noincremental
```

## Next Steps

After installation, proceed to the main [README](README.md) for usage instructions and configuration options.
