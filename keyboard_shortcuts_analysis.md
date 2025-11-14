# GTerm - Keyboard Shortcuts & Keyboard Handling Analysis

## Overview
GTerm is a Tauri-based macOS SSH terminal application written with Svelte frontend and Rust backend. Currently, keyboard shortcut handling is minimal and purely implemented at the frontend level with no Tauri menu system integration.

---

## 1. Current Keyboard Shortcuts Implementation

### Location: `/Users/g0shk3/Developer/GTerm/src/App.svelte` (Lines 74-98)

Currently implemented shortcuts (via `keyboardHandler` function):

```javascript
// Cmd+W - Close current tab
if (e.metaKey && e.key === 'w') {
  e.preventDefault();
  handleCloseTab($activeTabId);
  return;
}

// Cmd+Q - Close application
if (e.metaKey && e.key === 'q') {
  e.preventDefault();
  window.close();
  return;
}

// Cmd+Shift+D - Duplicate session
if (e.metaKey && e.shiftKey && (e.key === 'D' || e.key === 'd')) {
  e.preventDefault();
  const currentTab = $tabs.find(t => t.id === $activeTabId);
  if (currentTab) {
    createTab(currentTab.host);
  }
  return;
}
```

### Current Shortcuts Summary:
| Shortcut | Action | Status |
|----------|--------|--------|
| Cmd+W | Close active tab | ✓ Working |
| Cmd+Q | Close application | ✓ Working |
| Cmd+Shift+D | Duplicate session | ✓ Working |

### Registration Method:
- **Mechanism**: Global document-level `keydown` event listener
- **Location**: `onMount` hook in App.svelte (line 28)
- **Lifecycle**: Registered once, removed on component destroy
- **Event Handling**: Using event.preventDefault() for standard macOS shortcuts

---

## 2. Other Keyboard Event Handlers Found

### HostManager.svelte (Line 139)
Modal Escape key handling:
```javascript
on:keydown={(e) => e.key === 'Escape' && handleClose()}
```
- **Purpose**: Close SSH connections modal with Escape key

### SFTP.svelte (Line 173)
Enter key for file navigation:
```javascript
on:keydown={(e) => e.key === 'Enter' && handleFileClick(file)}
```
- **Purpose**: Navigate directories/open files with Enter key

---

## 3. Tauri Menu System & Accelerators

### Current Status: **NOT IMPLEMENTED**

The Tauri framework (v2.0) supports native menu system with keyboard accelerators, but GTerm does not use it.

### Tauri Configuration:
- **File**: `/Users/g0shk3/Developer/GTerm/src-tauri/tauri.conf.json`
- **Current Setup**: Minimal - no menu configuration
- **Capabilities**: `/Users/g0shk3/Developer/GTerm/src-tauri/capabilities/main.json`
  - Includes `core:menu:*` permissions but not used
  - No menu-specific capabilities defined

### Why Menu System Not Used:
1. No `menu` builder in main.rs
2. No menu configuration in tauri.conf.json
3. Current implementation relies purely on DOM event handlers

---

## 4. Global Shortcut Handling

### Current Status: **NOT IMPLEMENTED**

Tauri provides `tauri-plugin-global-shortcut` for system-level shortcuts, but GTerm doesn't use it.

### Missing Integration:
- No dependencies in Cargo.toml for global shortcuts plugin
- No system-level hotkey registration
- Shortcuts only work when app window is focused

---

## 5. Split View / Split Pane Functionality

### Current Status: **NOT IMPLEMENTED**

Searched for:
- "split", "pane", "column", "divide" - No results
- Split pane UI components - Not found
- Draggable dividers - Not implemented

### Current Architecture:
- **Tab-based interface only** - Multiple tabs, one view visible
- **Sidebar**: Collapsible right sidebar for connection management
- **No pane splitting**: Cannot view multiple connections simultaneously

### What Exists:
- Tab switching via button clicks (line 113, App.svelte)
- Tab switching via keyboard: Currently NOT mapped to shortcuts
- Sidebar toggle button (line 138)
- Tab closing (line 124)

---

## 6. Search / Command Palette / Profile Switching

### Current Status: **NOT IMPLEMENTED**

No evidence of:
- Search functionality
- Command palette
- Quick profile/host switcher
- Keyboard-triggered connection switcher

### Current Navigation:
- Click tabs to switch between connections
- Click sidebar hosts to connect
- No search or filtering of hosts
- All saved hosts visible in sidebar

### Sidebar Features:
- Host list display (Sidebar.svelte, lines 39-74)
- Host filtering: None (shows all hosts)
- Host search: Not implemented
- Quick connect: Click-based only

---

## 7. Project Structure Reference

### Frontend Files:
```
src/
├── App.svelte                    # Main app, keyboard handler here
├── lib/
│   ├── components/
│   │   ├── Terminal.svelte      # Terminal UI (xterm.js integration)
│   │   ├── SFTP.svelte          # File transfer UI
│   │   ├── Sidebar.svelte       # Connection list sidebar
│   │   └── HostManager.svelte   # Modal for managing SSH profiles
│   └── stores/
│       ├── tabs.js             # Tab state management
│       ├── hosts.js            # Host/connection storage
│       └── theme.js            # Theme state
```

### Backend Files:
```
src-tauri/
├── src/
│   ├── main.rs                 # Tauri app entry, commands exposed
│   └── ssh/
│       ├── mod.rs              # SSH connection management
│       ├── connection.rs        # SSH connection implementation
│       ├── sftp.rs            # SFTP file operations
│       └── keygen.rs           # SSH key generation
├── tauri.conf.json             # Tauri configuration (no menu defined)
├── capabilities/
│   └── main.json              # Permissions manifest
└── Cargo.toml                  # Rust dependencies
```

### Dependencies Status:
```toml
# Keyboard-related dependencies: NONE
# Menu system dependencies: NONE
# Global shortcut dependencies: NONE
# Available in Tauri 2.0 but not added
```

---

## 8. State Management

### Tab Management (tabs.js):
```javascript
export const tabs = writable([]);           // All open tabs
export const activeTabId = writable(null);  // Current tab ID

export function createTab(host)
export function closeTab(id)
export function renameTab(id, newTitle)
export function updateTabConnection(id, connected)
```

### Host Management (hosts.js):
```javascript
export async function getHosts()           // Load all saved hosts
export async function saveHost(host)       // Save/update host
export async function deleteHost(id)       // Delete host
export const hostsStore = writable([])     // Reactive host store
```

### Theme Management (theme.js):
- Dark/light theme toggle (button in header, line 144)

---

## 9. Tauri Commands Exposed

From `/Users/g0shk3/Developer/GTerm/src-tauri/src/main.rs`:

```rust
ssh_connect()              // Initiate SSH connection
ssh_send_input()           // Send data to terminal
ssh_disconnect()           // Close SSH connection
ssh_resize()               // Resize remote terminal
get_home_dir()            // Get user home directory
get_private_key_type()    // Check SSH key type
generate_keypair()        // Generate ed25519 key
sftp_list_directory()     // List remote files
sftp_download()           // Download file
sftp_upload()             // Upload file
```

---

## 10. Current Modal/Dialog Handling

### HostManager Modal:
- **Trigger**: Click "Connect to Server" or "Manage Connections"
- **Close**: Escape key or click outside (line 139)
- **Features**: Add/Edit/Delete SSH hosts
- **No keyboard shortcuts**: for form actions

### SFTP Component:
- **File Navigation**: Arrow keys - NOT implemented
- **Enter**: Select file/directory (line 173)
- **Keyboard shortcuts**: Only Enter key handled

### Terminal Component:
- **Focus**: Terminal captures all input (xterm.js)
- **No shortcuts**: Keyboard input forwarded to SSH session
- **Window resize**: Tracked and sent to remote terminal (line 103)

---

## 11. Missing Features Analysis

### High Priority for Implementation:
1. **Tab Navigation Shortcuts**
   - Cmd+Number (1-9): Jump to tab
   - Cmd+Tab: Next tab
   - Cmd+Shift+Tab: Previous tab
   - Cmd+T: New connection (currently requires clicking button)

2. **Profile/Host Switching**
   - Cmd+K: Quick host switcher / command palette
   - Search/filter hosts by typing
   - Keyboard navigation of sidebar

3. **Split View Shortcuts**
   - Cmd+D: Split vertically
   - Cmd+Shift+E: Split horizontally
   - Cmd+W: Close pane
   - Arrow keys: Navigate between panes

4. **Editor Features**
   - Cmd+F: Find/Search
   - Cmd+,: Settings/Preferences
   - Cmd+/: Toggle sidebar
   - Already implemented: Cmd+,: (not used)

5. **Accessibility**
   - F1: Help/Keyboard shortcuts reference
   - Cmd+Shift+?: Show all shortcuts

### Terminal-Level Improvements:
- Currently xterm.js captures all input
- Terminal-specific shortcuts might conflict with app shortcuts
- Need careful scoping of keyboard handler

---

## 12. Technology Stack

### Frontend:
- **Framework**: Svelte (not React/Vue)
- **Terminal**: xterm.js with FitAddon, WebLinksAddon
- **UI Framework**: TailwindCSS
- **State**: Svelte stores (writable, get)
- **File Dialogs**: @tauri-apps/plugin-dialog
- **API**: @tauri-apps/api/core (invoke, listen, emit)

### Backend:
- **Runtime**: Tauri 2.0
- **Language**: Rust
- **Async**: Tokio runtime
- **SSH**: ssh2 crate (v0.9)
- **Storage**: tauri-plugin-store (JSON file-based)
- **Plugins Used**:
  - tauri-plugin-dialog
  - tauri-plugin-store
  - tauri-plugin-shell

### Build Config:
- **Frontend Build**: Vite (npm run dev/build)
- **Frontend Dev**: http://localhost:5173
- **Bundle**: dmg + app format for macOS

---

## 13. Recommendations for Enhancement

### Phase 1: Immediate (No architecture changes)
1. Add tab navigation shortcuts (Cmd+1-9, Cmd+Tab)
2. Add modal keyboard shortcuts (focus management)
3. Add sidebar keyboard navigation (arrow keys)
4. Document existing shortcuts

### Phase 2: Medium-term (Minor refactoring)
1. Migrate to Tauri menu system for native menu bar
2. Add global shortcut plugin for system-level hotkeys
3. Implement command palette (Cmd+K)
4. Add keyboard shortcut settings/customization

### Phase 3: Long-term (Architecture changes)
1. Implement split pane functionality
2. Add advanced search/filtering
3. Add terminal-specific command shortcuts
4. Create keyboard shortcut conflict resolution system

---

## 14. Files to Modify for Enhancements

1. **App.svelte** - Main keyboard handler location
2. **main.rs** - Tauri builder configuration
3. **tauri.conf.json** - Menu configuration
4. **Cargo.toml** - Add new dependencies
5. **Terminal.svelte** - Terminal-specific keyboard handling
6. **Sidebar.svelte** - Keyboard navigation
7. **HostManager.svelte** - Modal keyboard handling

---

## Summary

**Current State**: Minimal keyboard shortcut system, 3 shortcuts only (Cmd+W, Cmd+Q, Cmd+Shift+D)

**Architecture**: 
- Frontend-only event handling
- No Tauri menu system
- No global shortcuts
- No split panes
- No search/command palette

**Extensibility**: 
- Well-structured Svelte components
- Clear separation of concerns
- Good use of stores for state
- Ready for enhancement with proper planning

**Tech Debt**:
- Keyboard handling scattered across components
- No centralized shortcut registry
- No conflict detection
- No keyboard remapping capability
