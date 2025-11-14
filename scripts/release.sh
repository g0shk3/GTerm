#!/bin/bash

export PATH="/opt/homebrew/bin:$PATH"

# GTerm Auto-Update Release Script
# Usage: ./scripts/release.sh <version>
# Example: ./scripts/release.sh 1.0.4

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper functions
print_step() {
    echo -e "${BLUE}==>${NC} $1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

# Check if version is provided
if [ -z "$1" ]; then
    print_error "Version not specified!"
    echo "Usage: ./scripts/release.sh <version>"
    echo "Example: ./scripts/release.sh 1.0.4"
    exit 1
fi

VERSION=$1
TAG="v${VERSION}"

print_step "Starting release process for version ${VERSION}"
echo ""

# Confirm with user
echo -e "${YELLOW}This will:${NC}"
echo "  1. Update version in package.json, tauri.conf.json, and Cargo.toml"
echo "  2. Build the application"
echo "  3. Create and sign the update package"
echo "  4. Generate latest.json"
echo "  5. Create GitHub release and upload files"
echo ""
read -p "Continue? (y/n) " -n 1 -r
echo ""
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    print_error "Release cancelled"
    exit 1
fi

# Step 1: Update versions
print_step "Step 1/7: Updating version in all files..."

# Update package.json
sed -i.bak "s/\"version\": \".*\"/\"version\": \"${VERSION}\"/" package.json
print_success "Updated package.json"

# Update tauri.conf.json
sed -i.bak "s/\"version\": \".*\"/\"version\": \"${VERSION}\"/" src-tauri/tauri.conf.json
print_success "Updated src-tauri/tauri.conf.json"

# Update Cargo.toml
sed -i.bak "s/^version = \".*\"/version = \"${VERSION}\"/" src-tauri/Cargo.toml
print_success "Updated src-tauri/Cargo.toml"

# Remove backup files
rm -f package.json.bak src-tauri/tauri.conf.json.bak src-tauri/Cargo.toml.bak

echo ""

# Step 2: Commit version changes
print_step "Step 2/7: Committing version changes..."
git add package.json src-tauri/tauri.conf.json src-tauri/Cargo.toml
git commit -m "Bump version to ${VERSION}" || print_warning "No changes to commit (version already updated)"
print_success "Version changes committed"
echo ""

# Step 3: Build the application
print_step "Step 3/7: Building the application (this takes 5-10 minutes)..."
npm run tauri:build
print_success "Build completed"
echo ""

# Step 4: Create tar.gz package
print_step "Step 4/7: Creating update package..."
cd src-tauri/target/release/bundle/macos

if [ ! -f "GTerm.app.tar.gz" ]; then
    # Use COPYFILE_DISABLE=1 to prevent macOS metadata files (._*) from being included
    COPYFILE_DISABLE=1 tar -czf GTerm.app.tar.gz GTerm.app
    print_success "Created GTerm.app.tar.gz"
else
    print_warning "GTerm.app.tar.gz already exists, skipping..."
fi

cd - > /dev/null
echo ""

# Step 5: Sign the package
print_step "Step 5/7: Signing the package..."
cd src-tauri/target/release/bundle/macos

if [ ! -f "GTerm.app.tar.gz.sig" ]; then
    # Sign with password Admin123! using expect for automation
    expect << 'EOF'
spawn tauri signer sign GTerm.app.tar.gz --private-key-path /Users/g0shk3/.tauri/gterm.key
expect "Password:"
send "Admin123!\r"
expect eof
EOF
    print_success "Package signed successfully"
else
    print_warning "GTerm.app.tar.gz.sig already exists, using existing signature"
fi

cd - > /dev/null
echo ""

# Step 6: Generate latest.json
print_step "Step 6/7: Generating latest.json..."

SIGNATURE=$(cat src-tauri/target/release/bundle/macos/GTerm.app.tar.gz.sig)
PUB_DATE=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

cat > src-tauri/target/release/bundle/macos/latest.json << EOF
{
  "version": "${VERSION}",
  "notes": "See release notes on GitHub: https://github.com/g0shk3/GTerm/releases/tag/${TAG}",
  "pub_date": "${PUB_DATE}",
  "platforms": {
    "darwin-aarch64": {
      "signature": "${SIGNATURE}",
      "url": "https://github.com/g0shk3/GTerm/releases/download/${TAG}/GTerm.app.tar.gz"
    }
  }
}
EOF

print_success "Generated latest.json"
echo ""

# Step 7: Create GitHub Release
print_step "Step 7/7: Creating GitHub release and uploading files..."

# Push commits and create tag
git push
git tag ${TAG}
git push origin ${TAG}
print_success "Git tag ${TAG} created and pushed"

# Create GitHub release
gh release create ${TAG} \
  --title "Release ${TAG}" \
  --notes "## What's New in ${TAG}

Update notes will be added here.

## Installation

Download \`GTerm_${VERSION}_aarch64.dmg\` below and drag GTerm.app to your Applications folder.

## Auto-Update

If you already have GTerm installed, it will automatically notify you about this update on next launch.

---

**Full Changelog**: https://github.com/g0shk3/GTerm/compare/v1.0.3...${TAG}" \
  || print_warning "Release already exists, skipping creation..."

print_success "GitHub release created"

# Upload files
print_step "Uploading release files..."

gh release upload ${TAG} \
  src-tauri/target/release/bundle/macos/GTerm.app.tar.gz \
  src-tauri/target/release/bundle/macos/GTerm.app.tar.gz.sig \
  src-tauri/target/release/bundle/macos/latest.json \
  src-tauri/target/release/bundle/dmg/GTerm_${VERSION}_aarch64.dmg \
  --clobber

print_success "All files uploaded successfully"
echo ""

# Final summary
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}🎉 Release ${TAG} completed successfully!${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo "📦 Release URL: https://github.com/g0shk3/GTerm/releases/tag/${TAG}"
echo ""
echo "✅ Uploaded files:"
echo "   - GTerm_${VERSION}_aarch64.dmg (for new installations)"
echo "   - GTerm.app.tar.gz (for auto-update)"
echo "   - GTerm.app.tar.gz.sig (signature)"
echo "   - latest.json (update manifest)"
echo ""
echo "🚀 Users with older versions will now see an update notification!"
echo ""
