# GTerm - macOS SSH Terminal

Fast, secure, and lightweight SSH terminal for macOS built with Tauri 2.0, Rust, and Svelte.

## Features

- **Fast & Lightweight**: Built with Rust and Tauri 2.0, significantly smaller and faster than Electron-based alternatives
- **SSH Connection Management**: Save and manage multiple SSH hosts
- **Private Key Support**: Support for ed25519, RSA, and ECDSA keys with automatic type detection
- **Key Generation**: Built-in ed25519 key pair generator
- **Interactive Terminal**: Full xterm.js terminal with vim, nano, htop support
- **SFTP Browser**: Browse, upload, and download files via SFTP
- **Tab Management**: Multiple SSH sessions in tabs
- **Dark/Light Theme**: Beautiful UI with theme switching
- **Secure**: All credentials stored locally, private keys never saved in config
- **macOS Keychain**: Optional password storage in system keychain
- **Real-time I/O**: Zero lag terminal experience
- **Port Forwarding**: Local and dynamic port forwarding support (coming soon)

## Installation

### Download

Download the latest release from the [Releases page](https://github.com/g0shk3/GTerm/releases/latest):
- **GTerm_1.0.x_aarch64.dmg** - For Apple Silicon (M1/M2/M3)
- **GTerm_1.0.x_x64.dmg** - For Intel Macs

### macOS Security Note

Since GTerm is not notarized by Apple, macOS will block it by default. This is normal for apps distributed outside the Mac App Store.

**⚠️ IMPORTANT: Installation on a new computer requires removing quarantine TWICE**

macOS applies quarantine attributes to both the DMG file AND the app inside it. You must remove quarantine from both to avoid "damaged" or "Library missing" errors.

**Recommended Installation Steps:**

1. **Download the DMG file** from GitHub releases
2. **Remove quarantine from DMG first:**
   ```bash
   xattr -cr ~/Downloads/GTerm_*.dmg
   ```
3. **Mount the DMG** by double-clicking it
4. **Copy GTerm.app to Applications:**
   ```bash
   cp -R /Volumes/GTerm/GTerm.app /Applications/
   ```
5. **Remove quarantine from the app:**
   ```bash
   xattr -cr /Applications/GTerm.app
   ```
6. **Launch the app:**
   ```bash
   open /Applications/GTerm.app
   ```

**Why is this needed twice?**
- First `xattr -cr` removes quarantine from the DMG archive
- Second `xattr -cr` removes quarantine from the app inside
- Without both steps, you may see errors like:
  - `"GTerm.app" is damaged and can't be opened`
  - `Library not loaded: /opt/homebrew/*/libssl.3.dylib`
  - `dyld: Library not loaded`

**Why is this needed at all?**
macOS Gatekeeper blocks apps that aren't signed with an Apple Developer certificate. GTerm is open-source and safe, but requires these steps to bypass Gatekeeper.

**Alternative: System Settings Method**
1. Try to open GTerm normally
2. Go to **System Settings > Privacy & Security**
3. Scroll down and click **"Open Anyway"** next to the GTerm message
4. Click **"Open"** in the confirmation dialog

### Auto-Update

GTerm includes built-in auto-update functionality. Once installed, the app will automatically notify you of new versions and download updates in the background.

## Requirements

- macOS Sonoma (14.0) or later
- Apple Silicon (M1/M2/M3) or Intel processor
- SSH key pair for authentication

## Development

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js (via Homebrew)
brew install node

# Install Tauri CLI
cargo install tauri-cli
```

### Setup

```bash
# Clone the repository
git clone <repository-url>
cd GTerm

# Install dependencies
npm install

# Run in development mode
npm run tauri:dev
```

### Build

```bash
# Build for production
npm run tauri:build
```

The built app will be in `src-tauri/target/release/bundle/`.

## Usage

### Adding a Host

1. Click "New Connection" button
2. Fill in the host details:
   - Name: A friendly name for the host
   - Host: IP address or hostname
   - Port: SSH port (default: 22)
   - Username: SSH username
   - Private Key: Path to your SSH private key
   - Passphrase: Optional passphrase for the key

3. Click "Save" to save the host for later use

### Generating SSH Keys

1. In the "New Connection" dialog, click "Generate" button
2. A new ed25519 key pair will be created in `~/.ssh/gterm_ed25519`
3. The public key will be copied to your clipboard
4. Paste the public key to your server's `~/.ssh/authorized_keys`

### Connecting

1. Select a saved host from the list
2. Click "Connect"
3. A new tab will open with the SSH session

### SFTP Browser

1. While connected to a host, you can browse files via SFTP
2. Navigate directories by clicking on folders
3. Download files by clicking the "Download" button
4. Upload files using the "Upload File" button

## Security

- **Private keys are never stored in the config** - only the path to the key file
- Optional password storage uses macOS Keychain
- All connections are established locally - no data is sent to external servers
- Support for modern ed25519 keys with warnings for outdated RSA keys

## Architecture

- **Frontend**: Svelte + Tailwind CSS + xterm.js
- **Backend**: Rust with Tauri 2.0
- **SSH**: ssh2 crate for native SSH connections
- **Terminal**: xterm.js for terminal emulation
- **Storage**: tauri-plugin-store for local JSON storage
- **Keychain**: tauri-plugin-keyring for secure credential storage

## Keyboard Shortcuts

- `Ctrl+C` - Send interrupt signal
- `Ctrl+D` - Send EOF / exit
- `Ctrl+L` - Clear screen
- Arrow keys - Navigate command history
- `Tab` - Auto-completion (server-side)

## Troubleshooting

### Connection Issues

- Verify the SSH key has correct permissions (0600)
- Check if the key is in the correct format (OpenSSH)
- Test connection manually: `ssh -i /path/to/key user@host`

### RSA Key Warning

If you see a warning about RSA keys, consider generating a new ed25519 key:

```bash
ssh-keygen -t ed25519 -C "your_email@example.com"
```

Then copy the public key to your server:

```bash
ssh-copy-id -i ~/.ssh/id_ed25519.pub user@host
```

## License

MIT License - See LICENSE file for details

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Roadmap

- [x] SSH connection with private key authentication
- [x] Interactive terminal with xterm.js
- [x] SFTP file browser
- [x] Tab management
- [x] Theme switching
- [x] Key generation
- [ ] Port forwarding (local and dynamic)
- [ ] Command history with search
- [ ] Snippets/shortcuts
- [ ] Session recording
- [ ] Multi-pane terminal
- [ ] Plugin system

## Credits

Inspired by Termius, but built from scratch with performance and security in mind.
