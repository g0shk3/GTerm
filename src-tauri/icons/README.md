# Icons

To generate proper icons for the app, use the following tools:

## macOS Icons

You'll need to create the following icon files:
- `32x32.png` - 32x32 pixels
- `128x128.png` - 128x128 pixels
- `128x128@2x.png` - 256x256 pixels (for Retina)
- `icon.icns` - macOS icon bundle
- `icon.ico` - Windows icon (for cross-platform)

## Using Tauri Icon Generator

You can use Tauri's icon generator to create all required icons from a single source:

```bash
# Install cargo-tauri if not already installed
cargo install tauri-cli

# Generate icons from a 1024x1024 PNG source
cargo tauri icon /path/to/icon-1024x1024.png
```

## Icon Requirements

- Source image should be at least 1024x1024 pixels
- Use a square aspect ratio
- PNG format with transparency
- Simple, recognizable design that works at small sizes

## Temporary Solution

For development, you can use Tauri's default icons or create simple placeholder icons using any image editor.
