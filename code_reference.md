# GTerm - Keyboard & Shortcut Code Reference

## 1. Main Keyboard Handler (App.svelte)

**File**: `/Users/g0shk3/Developer/GTerm/src/App.svelte`

**Registration** (lines 26-37):
```javascript
onMount(async () => {
  // ... other initialization code ...
  
  // Register keyboard shortcuts (only once)
  if (!keyboardShortcutsRegistered) {
    document.addEventListener('keydown', keyboardHandler);
    keyboardShortcutsRegistered = true;
  }

  return () => {
    if (keyboardShortcutsRegistered) {
      document.removeEventListener('keydown', keyboardHandler);
      keyboardShortcutsRegistered = false;
    }
  };
});
```

**Handler Implementation** (lines 74-98):
```javascript
const keyboardHandler = (e) => {
  // Cmd+W - затвори текущия таб
  if (e.metaKey && e.key === 'w') {
    e.preventDefault();
    handleCloseTab($activeTabId);
    return;
  }

  // Cmd+Q - затвори приложението
  if (e.metaKey && e.key === 'q') {
    e.preventDefault();
    window.close();
    return;
  }

  // Cmd+Shift+D - дупликат на сесията
  if (e.metaKey && e.shiftKey && (e.key === 'D' || e.key === 'd')) {
    e.preventDefault();
    const currentTab = $tabs.find(t => t.id === $activeTabId);
    if (currentTab) {
      createTab(currentTab.host);
    }
    return;
  }
};
```

---

## 2. Tab Management Store (tabs.js)

**File**: `/Users/g0shk3/Developer/GTerm/src/lib/stores/tabs.js`

```javascript
import { writable, get } from 'svelte/store';

export const tabs = writable([]);
export const activeTabId = writable(null);

let tabCounter = 0;

export function createTab(host) {
  const id = `tab-${++tabCounter}`;
  const newTab = {
    id,
    sessionId: `session-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
    title: host.name || host.host,
    host,
    type: 'terminal', // 'terminal' or 'sftp'
    connected: false,
  };

  tabs.update(t => [...t, newTab]);
  activeTabId.set(id);

  return newTab;
}

export function closeTab(id) {
  tabs.update(t => {
    const filtered = t.filter(tab => tab.id !== id);
    return filtered;
  });

  activeTabId.update(current => {
    if (current === id) {
      const currentTabs = get(tabs);
      if (currentTabs.length > 0) {
        return currentTabs[currentTabs.length - 1].id;
      }
      return null;
    }
    return current;
  });
}

export function renameTab(id, newTitle) {
  tabs.update(t => {
    const tab = t.find(tab => tab.id === id);
    if (tab) {
      tab.title = newTitle;
    }
    return t;
  });
}

export function updateTabConnection(id, connected) {
  tabs.update(t => {
    const tab = t.find(tab => tab.id === id);
    if (tab) {
      tab.connected = connected;
    }
    return t;
  });
}
```

---

## 3. Host Management Store (hosts.js)

**File**: `/Users/g0shk3/Developer/GTerm/src/lib/stores/hosts.js`

```javascript
import { writable } from 'svelte/store';
import { load } from '@tauri-apps/plugin-store';

let store = null;

async function initStore() {
  if (!store) {
    store = await load('hosts.json', { autoSave: true });
  }
  return store;
}

export async function getHosts() {
  const s = await initStore();
  const hosts = await s.get('hosts');
  return hosts || [];
}

export async function saveHost(host) {
  const s = await initStore();
  const hosts = await getHosts();
  const existingIndex = hosts.findIndex(h => h.id === host.id);

  if (existingIndex >= 0) {
    hosts[existingIndex] = host;
  } else {
    hosts.push(host);
  }

  await s.set('hosts', hosts);
  await s.save();
  return hosts;
}

export async function deleteHost(id) {
  const s = await initStore();
  const hosts = await getHosts();
  const filtered = hosts.filter(h => h.id !== id);
  await s.set('hosts', filtered);
  await s.save();
  return filtered;
}

export const hostsStore = writable([]);

export async function loadHosts() {
  const hosts = await getHosts();
  hostsStore.set(hosts);
  return hosts;
}

export async function addAndReloadHost(host) {
  const hosts = await saveHost(host);
  hostsStore.set(hosts);
  return hosts;
}

export async function removeAndReloadHost(id) {
  const hosts = await deleteHost(id);
  hostsStore.set(hosts);
  return hosts;
}
```

---

## 4. HostManager Modal Keyboard Handling

**File**: `/Users/g0shk3/Developer/GTerm/src/lib/components/HostManager.svelte`

```svelte
<!-- Modal overlay with Escape key handler (line 139) -->
<div 
  class="modal-overlay" 
  on:click={handleClose} 
  on:keydown={(e) => e.key === 'Escape' && handleClose()} 
  role="dialog" 
  tabindex="-1"
>
  <div 
    class="modal-content" 
    on:click|stopPropagation 
    on:keydown|stopPropagation 
    role="document"
  >
    <!-- Modal content -->
  </div>
</div>
```

---

## 5. SFTP File Navigation Keyboard Handling

**File**: `/Users/g0shk3/Developer/GTerm/src/lib/components/SFTP.svelte`

```svelte
<!-- File row with Enter key handler (line 173) -->
<tr
  class="file-row"
  class:selected={selectedFile?.path === file.path}
  on:click={() => handleFileClick(file)}
  on:keydown={(e) => e.key === 'Enter' && handleFileClick(file)}
  role="button"
  tabindex="0"
>
  <td class="file-name">
    <span class="file-icon">
      {file.is_dir ? '📁' : '📄'}
    </span>
    {file.name}
  </td>
  <!-- ... other cells ... -->
</tr>
```

---

## 6. Tauri Configuration (No Menu Setup)

**File**: `/Users/g0shk3/Developer/GTerm/src-tauri/tauri.conf.json`

```json
{
  "productName": "GTerm",
  "version": "1.0.0",
  "identifier": "com.gterm.app",
  "build": {
    "beforeDevCommand": "npm run dev",
    "devUrl": "http://localhost:5173",
    "beforeBuildCommand": "npm run build",
    "frontendDist": "../dist"
  },
  "bundle": {
    "active": true,
    "targets": ["dmg", "app"],
    "icon": [],
    "macOS": {
      "minimumSystemVersion": "11.0",
      "entitlements": null,
      "exceptionDomain": null
    }
  },
  "app": {
    "windows": [
      {
        "label": "main",
        "title": "GTerm",
        "width": 1200,
        "height": 800,
        "resizable": true,
        "fullscreen": false,
        "minWidth": 800,
        "minHeight": 600,
        "decorations": true,
        "transparent": false,
        "skipTaskbar": false,
        "center": true
      }
    ],
    "security": {
      "csp": null,
      "assetProtocol": {
        "enable": true,
        "scope": {
          "allow": ["$APPDATA/**", "$HOME/.ssh/**"],
          "deny": []
        }
      },
      "capabilities": ["main-capability"]
    }
  }
}
```

**Note**: No `menu` configuration present - this is where native menu shortcuts would be defined.

---

## 7. Tauri Capabilities Configuration

**File**: `/Users/g0shk3/Developer/GTerm/src-tauri/capabilities/main.json`

```json
{
  "identifier": "main-capability",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "core:window:default",
    // ... window permissions ...
    "core:event:default",
    "core:event:allow-listen",
    "core:event:allow-unlisten",
    "core:event:allow-emit",
    "core:event:allow-emit-to",
    // ... other permissions ...
    // NOTE: "core:menu:*" permissions are present but NOT USED
  ]
}
```

---

## 8. Tauri Main.rs (Backend - No Menu Setup)

**File**: `/Users/g0shk3/Developer/GTerm/src-tauri/src/main.rs` (partial)

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            connections: Arc::new(Mutex::new(HashMap::new())),
        })
        .invoke_handler(tauri::generate_handler![
            ssh_connect,
            ssh_send_input,
            ssh_disconnect,
            ssh_resize,
            get_home_dir,
            get_private_key_type,
            generate_keypair,
            sftp_list_directory,
            sftp_download,
            sftp_upload,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Note**: No `.menu()` or `.plugin(tauri_plugin_global_shortcut::init())` calls.

---

## 9. Rust Dependencies (Cargo.toml)

**File**: `/Users/g0shk3/Developer/GTerm/src-tauri/Cargo.toml`

```toml
[package]
name = "gterm"
version = "1.0.0"
description = "Fast and secure macOS SSH Terminal"
authors = ["GTerm Team"]
edition = "2021"

[build-dependencies]
tauri-build = { version = "2.0", features = [] }

[dependencies]
tauri = { version = "2.0", features = ["protocol-asset"] }
tauri-plugin-dialog = "2.0"
tauri-plugin-store = "2.0"
tauri-plugin-shell = "2.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.35", features = ["full"] }
ssh2 = "0.9"
anyhow = "1.0"
thiserror = "1.0"
base64 = "0.21"
dirs = "5.0"
keyring = "2.3"
async-trait = "0.1"
futures = "0.3"
chrono = "0.4"

# NOTE: Missing for keyboard enhancements:
# - tauri-plugin-global-shortcut (for system-level shortcuts)
# - No custom keyboard mapping library
```

---

## 10. Sidebar Component Structure

**File**: `/Users/g0shk3/Developer/GTerm/src/lib/components/Sidebar.svelte`

```svelte
<script>
  import { createEventDispatcher } from 'svelte';
  import { hostsStore } from '../stores/hosts';

  export let isOpen = true;

  const dispatch = createEventDispatcher();

  function selectHost(host) {
    dispatch('connect', host);
  }

  function editHost(event, host) {
    event.stopPropagation();
    dispatch('edit', host);
  }

  function toggleSidebar() {
    isOpen = !isOpen;
  }
</script>

<aside class="sidebar" class:open={isOpen}>
  <div class="sidebar-header">
    <button on:click={toggleSidebar} class="toggle-btn" title="Toggle sidebar">
      {#if isOpen}
        →
      {:else}
        ←
      {/if}
    </button>
    {#if isOpen}
      <h3 class="sidebar-title">Connections</h3>
    {/if}
  </div>

  {#if isOpen}
    <div class="sidebar-content">
      <div class="hosts-list">
        {#if $hostsStore.length === 0}
          <div class="empty-state">
            <p>No saved connections</p>
          </div>
        {:else}
          {#each $hostsStore as host (host.id)}
            <div class="host-card-wrapper">
              <button
                class="host-card"
                on:click={() => selectHost(host)}
              >
                <!-- Host card content -->
              </button>
              <button
                class="host-edit-btn"
                on:click={(e) => editHost(e, host)}
              >
                <!-- Edit button -->
              </button>
            </div>
          {/each}
        {/if}
      </div>

      <button
        on:click={() => dispatch('manage')}
        class="manage-btn"
      >
        + Manage Connections
      </button>
    </div>
  {/if}
</aside>
```

---

## 11. Tab Switching Implementation

**File**: `/Users/g0shk3/Developer/GTerm/src/App.svelte` (lines 70-72)

```javascript
function switchTab(tabId) {
  activeTabId.set(tabId);
}
```

**Current Usage** (line 113):
```svelte
<button
  class="modern-tab"
  class:active={tab.id === $activeTabId}
  on:click={() => switchTab(tab.id)}
>
```

**Not bound to any keyboard shortcut currently** - only supports click.

---

## 12. Terminal Component Input Handling

**File**: `/Users/g0shk3/Developer/GTerm/src/lib/components/Terminal.svelte` (lines 61-83)

```javascript
// Handle user input
terminal.onData(async (data) => {
  try {
    await invoke('ssh_send_input', {
      sessionId: tab.sessionId,
      data: data,
    });
  } catch (error) {
    console.error('Failed to send input:', error);
  }
});

// Handle terminal resize
terminal.onResize(async ({ cols, rows }) => {
  try {
    await invoke('ssh_resize', {
      sessionId: tab.sessionId,
      cols,
      rows,
    });
  } catch (error) {
    console.error('Failed to resize:', error);
  }
});

// Window resize handler
window.addEventListener('resize', handleResize);
```

**Note**: Terminal captures all keyboard input - shortcuts must be handled at app level or explicitly excluded.

---

## Summary of Code Locations

| Feature | File | Line(s) | Status |
|---------|------|---------|--------|
| Keyboard handler registration | App.svelte | 26-37 | Implemented |
| Main shortcut logic | App.svelte | 74-98 | Implemented (3 shortcuts) |
| Tab store | tabs.js | Full file | Ready for keyboard nav |
| Host store | hosts.js | Full file | Ready for keyboard search |
| Modal escape key | HostManager.svelte | 139 | Implemented |
| File enter key | SFTP.svelte | 173 | Implemented |
| Sidebar toggle | Sidebar.svelte | 25 | Click only |
| Tauri config | tauri.conf.json | Full file | No menu section |
| Capabilities | main.json | Full file | Permissions exist, unused |
| Backend setup | main.rs | 153-175 | No menu builder |
| Dependencies | Cargo.toml | Full file | Missing keyboard plugins |

