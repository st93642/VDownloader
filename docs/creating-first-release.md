# Creating the First Release (v0.1.0)

This guide explains how to create and publish the first release of VDownloader.

## Quick Start

To create the v0.1.0 release:

```bash
# 1. Merge this PR and ensure you're on the main branch
git checkout main
git pull origin main

# 2. Run the release script
./scripts/create-release.sh 0.1.0
```

The script will:
- Create the `v0.1.0` tag
- Push it to GitHub
- Trigger the automated build and release workflow

## What Happens Next

Once the tag is pushed:

1. **GitHub Actions Workflow Starts** (~10-15 minutes)
   - Builds Linux binary (Ubuntu)
   - Builds Windows binary (MSYS2/MinGW)
   - Builds macOS binary (Homebrew)

2. **Release is Created Automatically**
   - All binaries are attached to the release
   - Release notes are auto-generated
   - Release is published at: https://github.com/st93642/VDownloader/releases/tag/v0.1.0

3. **Monitor Progress**
   - Watch the workflow: https://github.com/st93642/VDownloader/actions
   - Check for any build failures
   - Verify all artifacts are uploaded

## Manual Alternative

If you prefer to create the tag manually:

```bash
# Create annotated tag
git tag -a v0.1.0 -m "Release v0.1.0 - Initial release

Features:
- Multi-platform video downloader (YouTube, TikTok, X, VK Video, Rutube, Instagram, Reddit, Dzen)
- GTK4-based native desktop application
- Integrated video search functionality
- Modern UI following GNOME HIG guidelines
- Async downloads with Tokio runtime
"

# Push the tag
git push origin v0.1.0
```

## Verification Steps

After the workflow completes:

1. **Check the release page**: https://github.com/st93642/VDownloader/releases/tag/v0.1.0
2. **Verify artifacts are present**:
   - ✓ vdownloader-linux
   - ✓ vdownloader-windows.exe
   - ✓ vdownloader-macos
3. **Test download links** for each platform
4. **Review auto-generated release notes**

## If Something Goes Wrong

### Workflow Fails

1. Check the workflow logs: https://github.com/st93642/VDownloader/actions
2. Identify which build failed (Linux/Windows/macOS)
3. Fix the issue in the code
4. Delete the tag and recreate it:
   ```bash
   git tag -d v0.1.0
   git push origin :refs/tags/v0.1.0
   ./scripts/create-release.sh 0.1.0
   ```

### Tag Push Fails

If you can't push the tag due to permissions:

1. Ask repository admin to push the tag, or
2. Create the tag through GitHub UI:
   - Go to: https://github.com/st93642/VDownloader/releases/new
   - Enter tag: `v0.1.0`
   - Target: `main` branch
   - Click "Publish release"

## Post-Release

After successful release:

- [ ] Announce the release (optional)
- [ ] Update any external documentation
- [ ] Plan features for v0.2.0
- [ ] Close related issues

## Need More Help?

See the comprehensive release documentation: [RELEASE.md](../RELEASE.md)
