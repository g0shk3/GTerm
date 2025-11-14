#!/bin/bash

# GTerm Release Preparation Script (without gh CLI)
# Usage: ./scripts/prepare-release.sh <version>
# Example: ./scripts/prepare-release.sh 1.0.4

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

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

# Check version argument
if [ -z "$1" ]; then
    print_error "Version not specified!"
    echo "Usage: ./scripts/prepare-release.sh <version>"
    echo "Example: ./scripts/prepare-release.sh 1.0.4"
    exit 1
fi

VERSION=$1
TAG="v${VERSION}"

print_step "Preparing release for version ${VERSION}"
echo ""

# Step 1: Update versions
print_step "Step 1/6: Updating version in all files..."

sed -i.bak "s/\"version\": \".*\"/\"version\": \"${VERSION}\"/" package.json
print_success "Updated package.json"

sed -i.bak "s/\"version\": \".*\"/\"version\": \"${VERSION}\"/" src-tauri/tauri.conf.json
print_success "Updated src-tauri/tauri.conf.json"

sed -i.bak "s/^version = \".*\"/version = \"${VERSION}\"/" src-tauri/Cargo.toml
print_success "Updated src-tauri/Cargo.toml"

rm -f package.json.bak src-tauri/tauri.conf.json.bak src-tauri/Cargo.toml.bak
echo ""

# Step 2: Build
print_step "Step 2/6: Building the application (5-10 minutes)..."
npm run tauri:build
print_success "Build completed"
echo ""

# Step 3: Create tar.gz
print_step "Step 3/6: Creating update package..."
cd src-tauri/target/release/bundle/macos

if [ -f "GTerm.app.tar.gz" ]; then
    rm -f GTerm.app.tar.gz GTerm.app.tar.gz.sig
fi

tar -czf GTerm.app.tar.gz GTerm.app
print_success "Created GTerm.app.tar.gz"

cd - > /dev/null
echo ""

# Step 4: Sign the package
print_step "Step 4/6: Signing the package..."
cd src-tauri/target/release/bundle/macos

printf '\n' | tauri signer sign GTerm.app.tar.gz --private-key-path ~/.tauri/gterm.key || true
print_success "Package signed"

cd - > /dev/null
echo ""

# Step 5: Generate latest.json
print_step "Step 5/6: Generating latest.json..."

SIGNATURE=$(cat src-tauri/target/release/bundle/macos/GTerm.app.tar.gz.sig)
PUB_DATE=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

cat > src-tauri/target/release/bundle/macos/latest.json << EOF
{
  "version": "${VERSION}",
  "notes": "See release notes: https://github.com/g0shk3/GTerm/releases/tag/${TAG}",
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
cat src-tauri/target/release/bundle/macos/latest.json
echo ""

# Step 6: Create release directory
print_step "Step 6/6: Creating release directory..."

RELEASE_DIR="release-${VERSION}"
rm -rf "${RELEASE_DIR}"
mkdir -p "${RELEASE_DIR}"

cp src-tauri/target/release/bundle/macos/GTerm.app.tar.gz "${RELEASE_DIR}/"
cp src-tauri/target/release/bundle/macos/GTerm.app.tar.gz.sig "${RELEASE_DIR}/"
cp src-tauri/target/release/bundle/macos/latest.json "${RELEASE_DIR}/"
cp src-tauri/target/release/bundle/dmg/GTerm_${VERSION}_aarch64.dmg "${RELEASE_DIR}/"

print_success "Release files copied to ${RELEASE_DIR}/"
echo ""

# Final summary
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}🎉 Release ${TAG} prepared successfully!${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo "📁 All files are in: ${RELEASE_DIR}/"
echo ""
ls -lh "${RELEASE_DIR}/"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo "1. Go to: https://github.com/g0shk3/GTerm/releases/new"
echo "2. Create tag: ${TAG}"
echo "3. Upload these files:"
echo "   - ${RELEASE_DIR}/GTerm_${VERSION}_aarch64.dmg"
echo "   - ${RELEASE_DIR}/GTerm.app.tar.gz"
echo "   - ${RELEASE_DIR}/GTerm.app.tar.gz.sig"
echo "   - ${RELEASE_DIR}/latest.json"
echo "4. Publish the release"
echo ""
echo "🚀 After publishing, restart GTerm v1.0.3 to test auto-update!"
echo ""
