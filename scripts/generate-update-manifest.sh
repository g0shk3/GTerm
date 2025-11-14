#!/bin/bash

# Script to generate latest.json for Tauri updater
# This should be run after building the app and before creating a GitHub release

set -e

# Get version from tauri.conf.json
VERSION=$(node -p "require('./src-tauri/tauri.conf.json').version")
echo "Generating update manifest for version $VERSION"

# Platform-specific file names
DARWIN_APP="GTerm.app.tar.gz"
DARWIN_SIG="GTerm.app.tar.gz.sig"

# GitHub release URL base
RELEASE_URL="https://github.com/g0shk3/GTerm/releases/download/v${VERSION}"

# Check if signature file exists
if [ ! -f "src-tauri/target/release/bundle/macos/${DARWIN_SIG}" ]; then
    echo "Error: Signature file not found. Please sign the app first."
    echo "Run: npm run tauri signer sign src-tauri/target/release/bundle/macos/${DARWIN_APP} -- -k ~/.tauri/myapp.key"
    exit 1
fi

# Read the signature
SIGNATURE=$(cat "src-tauri/target/release/bundle/macos/${DARWIN_SIG}")

# Create latest.json
cat > latest.json << EOF
{
  "version": "v${VERSION}",
  "notes": "See the release notes on GitHub",
  "pub_date": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
  "platforms": {
    "darwin-x86_64": {
      "signature": "${SIGNATURE}",
      "url": "${RELEASE_URL}/${DARWIN_APP}"
    },
    "darwin-aarch64": {
      "signature": "${SIGNATURE}",
      "url": "${RELEASE_URL}/${DARWIN_APP}"
    }
  }
}
EOF

echo "✓ Generated latest.json"
cat latest.json
echo ""
echo "Next steps:"
echo "1. Create a GitHub release: gh release create v${VERSION}"
echo "2. Upload the app bundle: gh release upload v${VERSION} src-tauri/target/release/bundle/macos/${DARWIN_APP}"
echo "3. Upload the signature: gh release upload v${VERSION} src-tauri/target/release/bundle/macos/${DARWIN_SIG}"
echo "4. Upload the manifest: gh release upload v${VERSION} latest.json"
