# GitHub Actions Workflows

This directory contains the CI/CD workflows for KMarkdownify.

## Workflows

### 🔍 CI (`ci.yml`)

**Triggers:** Push/PR to main/master branches

Validates the project files and checks for common issues:

- Validates the desktop service menu file format
- Checks PKGBUILD dependencies are correctly listed
- Verifies shell script syntax
- Checks for best practices (error handling, variable usage, etc.)

### 🐚 ShellCheck (`shellcheck.yml`)

**Triggers:** Push/PR to main/master (when `.sh` files change)

Runs ShellCheck static analysis on all shell scripts to catch:

- Syntax errors
- Semantic problems
- Common pitfalls
- Portability issues

### 📦 PKGBUILD Validation (`pkgbuild.yml`)

**Triggers:** Push/PR to main/master (when PKGBUILD changes)

Validates the Arch Linux package:

- Checks PKGBUILD syntax
- Runs namcap for PKGBUILD linting
- Verifies all required fields are present
- Tests package preparation (dry-run)
- Confirms all source files exist

### 📝 Markdown Lint (`markdown-lint.yml`)

**Triggers:** Push/PR to main/master (when `.md` files change)

Ensures documentation quality:

- Lints all markdown files for consistency
- Checks for broken links (informational)
- Enforces markdown style guide

### 🚀 Release (`release.yml`)

**Triggers:**

- Push to version tags (`v*.*.*`)
- Manual workflow dispatch

Creates releases with Arch Linux packages:

- Builds the Arch package (`.pkg.tar.zst`)
- Runs namcap on the built package
- Creates GitHub release with release notes
- Uploads package as release artifact

## Best Practices Implemented

### Security

- ✅ Minimal permissions (`contents: read` by default)
- ✅ Actions pinned to specific versions
- ✅ No hardcoded secrets in workflows

### Efficiency

- ✅ Path filters to run only when relevant files change
- ✅ Parallel job execution where possible
- ✅ Artifact caching with appropriate retention

### Quality

- ✅ Multiple validation layers (syntax, linting, testing)
- ✅ Clear job and step names
- ✅ Informational checks with `continue-on-error`
- ✅ Comprehensive error messages

## Usage

### Running Workflows Manually

All workflows support manual triggering via `workflow_dispatch`:

1. Go to the **Actions** tab
2. Select the workflow you want to run
3. Click **Run workflow**
4. Select the branch and any required inputs

### Creating a Release

To create a new release:

#### Option 1: Git Tag (Recommended)

```bash
git tag v1.0.0
git push origin v1.0.0
```

#### Option 2: Manual Workflow

1. Go to Actions → Release workflow
2. Click "Run workflow"
3. Enter the version (e.g., `1.0.0`)
4. Click "Run workflow"

The workflow will:

1. Build the Arch package
2. Create a GitHub release
3. Upload the package artifact

### Local Testing

Before pushing, you can test locally:

```bash
# Test ShellCheck
shellcheck kmarkdownify.sh

# Test PKGBUILD
namcap PKGBUILD
makepkg --nobuild --nodeps

# Test desktop file
desktop-file-validate kmarkdownify.desktop

# Test markdown
npx markdownlint-cli2 "**/*.md"
```

## Monitoring

### Status Badges

Add these to your README to show workflow status:

```markdown
![CI](https://github.com/profiluefter/KMarkdownify/workflows/CI/badge.svg)
![ShellCheck](https://github.com/profiluefter/KMarkdownify/workflows/ShellCheck/badge.svg)
```

### Notifications

Failed workflows will:

- Send email notifications to commit authors
- Show in the GitHub Actions tab
- Block PR merges if configured as required checks

## Maintenance

### Updating Actions

Periodically update action versions:

```bash
# Check for updates
gh api repos/actions/checkout/releases/latest

# Update in workflow files
# Use Dependabot to automate this (see .github/dependabot.yml)
```

### Adding New Checks

To add a new validation:

1. Create a new workflow file in `.github/workflows/`
2. Use appropriate triggers and permissions
3. Add clear step names and error messages
4. Test with `workflow_dispatch` before committing
5. Update this README

## Troubleshooting

### Workflow Fails on First Run

- Check repository permissions
- Verify required secrets are set
- Review workflow logs for specific errors

### PKGBUILD Workflow Issues

- Ensure PKGBUILD syntax is valid
- Check that all source files are committed
- Verify checksums if not using `SKIP`

### Release Workflow Issues

- Ensure tag follows `v*.*.*` format
- Check that GITHUB_TOKEN has sufficient permissions
- Verify no existing release with the same tag

## Contributing

When adding or modifying workflows:

1. Test locally first
2. Use `workflow_dispatch` for initial testing
3. Document changes in this README
4. Follow existing naming conventions
5. Pin actions to specific versions (not `@latest`)
