# Release Process Documentation

This document describes how to create and publish releases for VDownloader.

## Overview

VDownloader uses an automated CI/CD pipeline via GitHub Actions to build and publish releases. When a version tag is pushed to the repository, the workflow automatically:

1. Builds binaries for Linux, Windows, and macOS
2. Creates a GitHub release
3. Attaches the built binaries to the release

## Prerequisites

Before creating a release, ensure:

- All changes are committed and pushed to the main branch
- The version number in `Cargo.toml` matches the release version
- All tests pass (`cargo test`)
- Code is properly formatted (`cargo fmt`)
- No linter warnings (`cargo clippy`)

## Creating a Release

### Method 1: Using the Release Script (Recommended)

The easiest way to create a release is using the provided script:

```bash
# Make sure you're on the main branch
git checkout main
git pull origin main

# Run the release script with the version number
./scripts/create-release.sh 0.1.0
```

The script will:
- Validate the version format
- Check for uncommitted changes
- Verify the Cargo.toml version
- Create an annotated git tag
- Push the tag to GitHub
- Provide next steps and monitoring links

### Method 2: Manual Release Creation

If you prefer to create the release manually:

1. **Update the version** (if needed):
   ```bash
   # Edit Cargo.toml and update the version field
   version = "0.1.0"
   ```

2. **Commit version changes** (if any):
   ```bash
   git add Cargo.toml
   git commit -m "Bump version to 0.1.0"
   git push origin main
   ```

3. **Create and push the tag**:
   ```bash
   # Create an annotated tag (replace 0.1.0 with your version)
   git tag -a v0.1.0 -m "Release v0.1.0 - Initial release

   Features:
   - Multi-platform video downloader
   - GTK4-based desktop application
   - Support for multiple video platforms
   - Integrated search functionality
   "

   # Push the tag to GitHub
   git push origin v0.1.0
   ```

4. **Monitor the workflow**:
   - Go to: https://github.com/st93642/VDownloader/actions
   - Watch the "Build Multi-Platform" workflow run
   - Wait for all jobs to complete (Linux, Windows, macOS builds)

5. **Verify the release**:
   - Go to: https://github.com/st93642/VDownloader/releases
   - Your release should appear with attached binaries

## Release Workflow Details

### Build Process

The GitHub Actions workflow (`.github/workflows/build.yml`) performs the following:

#### Linux Build (`build-linux`)
- Runs on: `ubuntu-latest`
- Installs: GTK4 development libraries
- Output: `vdownloader-linux` (native binary)

#### Windows Build (`build-windows`)
- Runs on: `windows-latest`
- Uses: MSYS2 with UCRT64 environment
- Installs: MinGW GTK4 and toolchain
- Target: `x86_64-pc-windows-gnu`
- Output: `vdownloader-windows.exe`

#### macOS Build (`build-macos`)
- Runs on: `macos-latest`
- Installs: GTK4 via Homebrew
- Output: 
  - `vdownloader-macos` (native binary)
  - `VDownloader.app` (macOS application bundle)

#### Release Creation (`create-release`)
- Triggers: Only when a tag starting with `v` is pushed
- Depends on: All platform builds completing successfully
- Uses: `softprops/action-gh-release@v1`
- Attaches: All built binaries to the GitHub release
- Auto-generates: Release notes from commits since last release

### Workflow Triggers

The workflow runs on:
- Push to `main` or `master` branches (builds only, no release)
- Push of tags matching `v*` (builds + creates release)
- Pull requests to `main` or `master` (builds only)
- Manual trigger via `workflow_dispatch`

## Version Numbering

VDownloader follows [Semantic Versioning](https://semver.org/):

- **MAJOR.MINOR.PATCH** (e.g., 1.2.3)
  - **MAJOR**: Breaking changes
  - **MINOR**: New features (backward compatible)
  - **PATCH**: Bug fixes (backward compatible)

Examples:
- `0.1.0` - Initial release
- `0.2.0` - New features added
- `0.2.1` - Bug fix release
- `1.0.0` - First stable release
- `1.1.0-beta.1` - Beta pre-release

## Troubleshooting

### Tag Already Exists

If you get an error that the tag already exists:

```bash
# Delete local tag
git tag -d v0.1.0

# Delete remote tag
git push origin :refs/tags/v0.1.0

# Create the tag again
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

### Workflow Fails

If the GitHub Actions workflow fails:

1. Check the workflow logs at: https://github.com/st93642/VDownloader/actions
2. Identify which job failed (Linux, Windows, or macOS)
3. Review the error messages in the logs
4. Fix the issue in the code
5. Delete the tag and create a new one after fixing

Common issues:
- **Build failures**: Usually dependency or compilation errors
- **Upload failures**: Network issues or artifact size limits
- **Release failures**: GitHub API rate limits or permissions issues

### Manual Release Creation

If the automated workflow fails, you can create a release manually:

1. Go to: https://github.com/st93642/VDownloader/releases/new
2. Select the tag (or create a new one)
3. Fill in the release title and description
4. Upload the built binaries manually from successful workflow runs
5. Publish the release

## Post-Release Checklist

After a successful release:

- [ ] Verify all binaries are attached to the release
- [ ] Test download links for each platform
- [ ] Update README.md if installation instructions changed
- [ ] Announce the release (if applicable)
- [ ] Close related issues and pull requests
- [ ] Plan next release milestones

## Release Artifacts

Each release includes the following artifacts:

| Platform | File Name | Description |
|----------|-----------|-------------|
| Linux | `vdownloader-linux` | Native Linux binary (requires GTK4) |
| Windows | `vdownloader-windows.exe` | Windows executable (requires GTK4 runtime) |
| macOS | `vdownloader-macos` | Native macOS binary (requires GTK4) |

**Note:** The macOS application bundle (`VDownloader.app`) is built during the CI process and available as a workflow artifact, but is not automatically attached to GitHub releases. Users should use the `vdownloader-macos` binary from the release page. The app bundle can be downloaded from the workflow artifacts if needed.

## Runtime Dependencies

All binaries require:
- **GTK4**: Must be installed on the target system
- **yt-dlp**: Must be available in system PATH

Installation guides for dependencies are in the main README.md.

## Security Notes

- Tags are immutable once pushed and should not be force-updated
- Release binaries are built in GitHub's secure CI environment
- Always verify checksums when distributing binaries (future enhancement)
- Keep dependencies updated to address security vulnerabilities

## Future Enhancements

Planned improvements to the release process:
- [ ] Add checksums (SHA256) for all binaries
- [ ] Code signing for Windows and macOS binaries
- [ ] Automated changelog generation from commit messages
- [ ] Pre-release and beta release support
- [ ] Automated version bumping in Cargo.toml
- [ ] Release notes templates
- [ ] Automated testing of release binaries

## Additional Resources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [GitHub Releases Documentation](https://docs.github.com/en/repositories/releasing-projects-on-github)
- [Semantic Versioning](https://semver.org/)
- [Cargo.toml Version Field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-version-field)
