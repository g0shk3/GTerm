# GTerm - Keyboard Shortcuts Quick Reference

## Currently Implemented Shortcuts

| Shortcut | Action | Component | Line |
|----------|--------|-----------|------|
| **Cmd+W** | Close active tab | App.svelte | 76 |
| **Cmd+Q** | Close application | App.svelte | 83 |
| **Cmd+Shift+D** | Duplicate current session | App.svelte | 90 |
| **Escape** | Close HostManager modal | HostManager.svelte | 139 |
| **Enter** | Navigate SFTP file/directory | SFTP.svelte | 173 |
| **Click** | Toggle sidebar | App.svelte | 138 |
| **Click** | Toggle theme (dark/light) | App.svelte | 144 |

---

## How Shortcuts Work

### Main Keyboard Handler (App.svelte)
- **Type**: Document-level DOM event listener
- **Trigger**: `keydown` event
- **Scope**: Global (all components)
- **Limitation**: Does NOT work when terminal has focus (xterm.js captures input)

### Sub-component Handlers
- **HostManager**: Escape to close modal
- **SFTP**: Enter to select file
- **Local**: Component-level event bindings

---

## Where to Add New Shortcuts

### Option 1: Simple App-Level Shortcut
Edit `/Users/g0shk3/Developer/GTerm/src/App.svelte` lines 74-98

Example:
```javascript
// Cmd+T - New connection
if (e.metaKey && e.key === 't') {
  e.preventDefault();
  showHostManager = true;
  return;
}
```

### Option 2: Component-Specific Shortcut
Use Svelte's `on:keydown` binding in the component

Example:
```svelte
<input on:keydown={(e) => {
  if (e.metaKey && e.key === 's') {
    e.preventDefault();
    handleSave();
  }
}} />
```

### Option 3: Native Tauri Menu (Future)
Would require:
1. Edit `src-tauri/tauri.conf.json` - add menu section
2. Edit `src-tauri/src/main.rs` - add menu builder
3. Add Rust dependencies

---

## Potential Conflicts

### Terminal Capture
- **Issue**: xterm.js captures ALL keyboard input when terminal has focus
- **Solution**: Add handlers in Terminal.svelte or App.svelte BEFORE terminal gets focus
- **Current**: App handlers won't work when typing in terminal

### macOS Standard Shortcuts
- **Cmd+W**: Standard "Close" - already implemented correctly
- **Cmd+Q**: Standard "Quit" - already implemented correctly
- **Cmd+Tab**: System app switcher - should NOT override

---

## Common Shortcut Patterns in macOS Apps

| Shortcut | Use Case | Status |
|----------|----------|--------|
| Cmd+N | New window/document | Not used |
| Cmd+T | New tab | Not implemented |
| Cmd+W | Close tab/window | Implemented |
| Cmd+Q | Quit app | Implemented |
| Cmd+Tab | Switch app | System default |
| Cmd+1,2,3... | Jump to tab | Not implemented |
| Cmd+Opt+→ | Next tab | Not implemented |
| Cmd+Opt+← | Previous tab | Not implemented |
| Cmd+F | Find | Not implemented |
| Cmd+K | Command palette | Not implemented |
| Cmd+, | Preferences | Not implemented |
| Cmd+/ | Toggle sidebar | Not implemented |
| Cmd+Shift+D | Duplicate | Implemented |

---

## Development Checklist for Adding Shortcuts

- [ ] Identify which component needs the shortcut
- [ ] Check for conflicts with macOS system shortcuts
- [ ] Check for conflicts with xterm.js input capture
- [ ] Choose between global (App.svelte) or local handler
- [ ] Add preventDefault() to avoid default behavior
- [ ] Test with terminal focused and unfocused
- [ ] Test with multiple tabs open
- [ ] Update documentation
- [ ] Test on physical macOS hardware

---

## Testing Shortcuts

### Manual Testing
1. Open GTerm
2. Try shortcut Cmd+W - should close tab
3. Try shortcut Cmd+Q - should close app
4. Try shortcut Cmd+Shift+D - should duplicate session
5. Click in terminal, try Cmd+W - may NOT work (xterm.js capture)

### Debugging
- Add console.log() to keyboardHandler
- Check browser DevTools (F12)
- Look for event.preventDefault() calls
- Check if other listeners consuming event

---

## File Structure for Shortcuts

```
src/
├── App.svelte                    <- Main global shortcuts
├── lib/
│   ├── components/
│   │   ├── Terminal.svelte      <- Terminal-specific shortcuts
│   │   ├── SFTP.svelte          <- SFTP-specific shortcuts
│   │   ├── Sidebar.svelte       <- Sidebar navigation
│   │   └── HostManager.svelte   <- Modal shortcuts
│   └── stores/
│       ├── tabs.js              <- Tab state (use for Cmd+1-9)
│       └── hosts.js             <- Host state (use for search)
```

---

## Future Enhancement Recommendations

### Phase 1: Easy (2-3 hours)
```javascript
// Add to App.svelte keyboardHandler
Cmd+T        // New connection dialog
Cmd+1-9      // Jump to specific tab
Cmd+Tab      // Next tab
Cmd+Shift+Tab // Previous tab
Cmd+/        // Toggle sidebar
```

### Phase 2: Medium (4-6 hours)
```javascript
// Add Tauri menu system
// Create native menu bar with accelerators
// Allows system shortcuts to work outside app window
```

### Phase 3: Advanced (8-12 hours)
```javascript
// Add split pane support
Cmd+D        // Split vertical
Cmd+Shift+E  // Split horizontal
Arrow keys   // Navigate panes

// Add command palette
Cmd+K        // Open palette
Type to search // Filter commands
```

---

## Technical Details

### Event Properties Used
- `e.metaKey` - Command key on macOS
- `e.shiftKey` - Shift key
- `e.key` - The actual key pressed (case-insensitive)
- `e.preventDefault()` - Stop default browser behavior

### Important Notes
- macOS uses `metaKey` NOT `ctrlKey`
- Always use `.toLowerCase()` for case-insensitive comparison
- Use `e.preventDefault()` to avoid browser defaults
- Terminal input (xterm.js) captures keydown events before App handler sees them

---

## Resources

- Tauri Menu Docs: https://tauri.app/docs/
- Tauri Global Shortcut: https://tauri.app/docs/
- xterm.js: https://xtermjs.org/
- macOS Keyboard Shortcut Guidelines: https://developer.apple.com/design/human-interface-guidelines/macos/user-interaction/keyboard/

---

## Getting Help

If you want to add a shortcut:
1. Check `keyboard_shortcuts_analysis.md` for detailed analysis
2. Check `code_reference.md` for code examples
3. Look at existing shortcuts in App.svelte
4. Test in browser DevTools console first
5. Make sure it doesn't conflict with system shortcuts

