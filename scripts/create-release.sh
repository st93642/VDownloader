#!/bin/bash
#*****************************************************************************#
#                                                                             #
#  create-release.sh - Release creation script         TTTTTTTT SSSSSSS II   #
#                                                         TT    SS      II   #
#  By: st93642@students.tsi.lv                            TT    SSSSSSS II   #
#                                                         TT         SS II   #
#  Created: Dec 07 2025 st93642                           TT    SSSSSSS II   #
#                                                                             #
#   Transport and Telecommunication Institute - Riga, Latvia                  #
#                       https://tsi.lv                                        #
#*****************************************************************************#

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}=========================================${NC}"
echo -e "${BLUE}VDownloader Release Creation Script${NC}"
echo -e "${BLUE}=========================================${NC}"
echo ""

# Check if version argument is provided
if [ -z "$1" ]; then
    echo -e "${RED}Error: Version number required${NC}"
    echo -e "Usage: $0 <version>"
    echo -e "Example: $0 0.1.0"
    echo ""
    echo -e "${YELLOW}Current version in Cargo.toml:${NC}"
    grep '^version = ' Cargo.toml | head -1
    exit 1
fi

VERSION=$1
TAG="v${VERSION}"

# Validate version format (semantic versioning)
if ! [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9\-]+(\.[a-zA-Z0-9\-]+)*)?$ ]]; then
    echo -e "${RED}Error: Invalid version format${NC}"
    echo -e "Version must follow semantic versioning (e.g., 0.1.0, 1.0.0, 1.0.0-beta.1)"
    exit 1
fi

echo -e "${BLUE}Creating release for version: ${VERSION}${NC}"
echo ""

# Check if tag already exists
if git rev-parse "$TAG" >/dev/null 2>&1; then
    echo -e "${RED}Error: Tag $TAG already exists${NC}"
    echo -e "To create a new release, either:"
    echo -e "  1. Delete the existing tag: git tag -d $TAG && git push origin :refs/tags/$TAG"
    echo -e "  2. Use a different version number"
    exit 1
fi

# Check for uncommitted changes
if ! git diff-index --quiet HEAD --; then
    echo -e "${YELLOW}Warning: You have uncommitted changes${NC}"
    echo -e "Please commit or stash your changes before creating a release"
    git status --short
    exit 1
fi

# Verify Cargo.toml version matches
CARGO_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
if [ "$CARGO_VERSION" != "$VERSION" ]; then
    echo -e "${YELLOW}Warning: Cargo.toml version ($CARGO_VERSION) doesn't match release version ($VERSION)${NC}"
    read -p "Do you want to continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo -e "${RED}Release cancelled${NC}"
        exit 1
    fi
fi

# Get current branch
CURRENT_BRANCH=$(git branch --show-current)
echo -e "${BLUE}Current branch: ${CURRENT_BRANCH}${NC}"

# Confirm release creation
echo ""
echo -e "${YELLOW}Ready to create release:${NC}"
echo -e "  Version: ${VERSION}"
echo -e "  Tag: ${TAG}"
echo -e "  Branch: ${CURRENT_BRANCH}"
echo ""
read -p "Continue? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${RED}Release cancelled${NC}"
    exit 1
fi

# Create annotated tag
echo ""
echo -e "${BLUE}Creating annotated tag ${TAG}...${NC}"
git tag -a "$TAG" -m "Release ${TAG}

Features and changes in this release:
- Multi-platform video downloader
- GTK4-based desktop application
- Support for multiple video platforms
- Integrated search functionality
"

echo -e "${GREEN}✓ Tag created successfully${NC}"

# Push tag to origin
echo ""
echo -e "${BLUE}Pushing tag to origin...${NC}"
if git push origin "$TAG"; then
    echo -e "${GREEN}✓ Tag pushed successfully${NC}"
else
    echo -e "${RED}✗ Failed to push tag${NC}"
    echo -e "You can manually push the tag later with: git push origin $TAG"
    exit 1
fi

echo ""
echo -e "${GREEN}=========================================${NC}"
echo -e "${GREEN}Release Created Successfully!${NC}"
echo -e "${GREEN}=========================================${NC}"
echo ""
echo -e "${BLUE}Next steps:${NC}"
echo -e "  1. GitHub Actions will automatically build binaries for all platforms"
echo -e "  2. A GitHub release will be created with the built artifacts"
echo -e "  3. Monitor the workflow at: https://github.com/st93642/VDownloader/actions"
echo ""
echo -e "${BLUE}Release URL (will be available after workflow completes):${NC}"
echo -e "  https://github.com/st93642/VDownloader/releases/tag/${TAG}"
echo ""
