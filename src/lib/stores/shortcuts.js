import { writable } from 'svelte/store';
import { Store } from '@tauri-apps/plugin-store';

// Default keyboard shortcuts
export const defaultShortcuts = [
  {
    id: 'close-tab',
    name: 'Close Tab',
    description: 'Close the current tab',
    key: 'w',
    metaKey: true,
    shiftKey: false,
    altKey: false,
    category: 'tabs'
  },
  {
    id: 'quit-app',
    name: 'Quit Application',
    description: 'Close the application',
    key: 'q',
    metaKey: true,
    shiftKey: false,
    altKey: false,
    category: 'app'
  },
  {
    id: 'clear-terminal',
    name: 'Clear Terminal',
    description: 'Clear the terminal screen',
    key: 'k',
    metaKey: true,
    shiftKey: false,
    altKey: false,
    category: 'terminal'
  },
  {
    id: 'duplicate-session',
    name: 'Duplicate Session',
    description: 'Duplicate the current session in a new tab',
    key: 'd',
    metaKey: true,
    shiftKey: true,
    altKey: false,
    category: 'tabs'
  },
  {
    id: 'split-vertical',
    name: 'Split Vertical',
    description: 'Split the terminal vertically',
    key: 'e',
    metaKey: true,
    shiftKey: true,
    altKey: false,
    category: 'panes'
  },
  {
    id: 'host-selector',
    name: 'Host Selector',
    description: 'Open quick host selector',
    key: 'e',
    metaKey: true,
    shiftKey: false,
    altKey: false,
    category: 'navigation'
  },
  {
    id: 'new-terminal',
    name: 'New Terminal',
    description: 'Open new local terminal',
    key: 't',
    metaKey: true,
    shiftKey: false,
    altKey: false,
    category: 'tabs'
  },
  {
    id: 'toggle-sidebar',
    name: 'Toggle Sidebar',
    description: 'Show/hide the sidebar',
    key: ',',
    metaKey: true,
    shiftKey: false,
    altKey: false,
    category: 'navigation'
  },
  {
    id: 'previous-tab',
    name: 'Previous Tab',
    description: 'Switch to the previous tab',
    key: '[',
    metaKey: true,
    shiftKey: false,
    altKey: false,
    category: 'tabs'
  },
  {
    id: 'next-tab',
    name: 'Next Tab',
    description: 'Switch to the next tab',
    key: ']',
    metaKey: true,
    shiftKey: false,
    altKey: false,
    category: 'tabs'
  },
  {
    id: 'tab-1',
    name: 'Tab 1',
    description: 'Switch to tab 1',
    key: '1',
    metaKey: true,
    shiftKey: false,
    altKey: false,
    category: 'tabs'
  },
  {
    id: 'tab-2',
    name: 'Tab 2',
    description: 'Switch to tab 2',
    key: '2',
    metaKey: true,
    shiftKey: false,
    altKey: false,
    category: 'tabs'
  },
  {
    id: 'tab-3',
    name: 'Tab 3',
    description: 'Switch to tab 3',
    key: '3',
    metaKey: true,
    shiftKey: false,
    altKey: false,
    category: 'tabs'
  },
  {
    id: 'tab-4',
    name: 'Tab 4',
    description: 'Switch to tab 4',
    key: '4',
    metaKey: true,
    shiftKey: false,
    altKey: false,
    category: 'tabs'
  },
  {
    id: 'tab-5',
    name: 'Tab 5',
    description: 'Switch to tab 5',
    key: '5',
    metaKey: true,
    shiftKey: false,
    altKey: false,
    category: 'tabs'
  },
  {
    id: 'tab-6',
    name: 'Tab 6',
    description: 'Switch to tab 6',
    key: '6',
    metaKey: true,
    shiftKey: false,
    altKey: false,
    category: 'tabs'
  },
  {
    id: 'tab-7',
    name: 'Tab 7',
    description: 'Switch to tab 7',
    key: '7',
    metaKey: true,
    shiftKey: false,
    altKey: false,
    category: 'tabs'
  },
  {
    id: 'tab-8',
    name: 'Tab 8',
    description: 'Switch to tab 8',
    key: '8',
    metaKey: true,
    shiftKey: false,
    altKey: false,
    category: 'tabs'
  },
  {
    id: 'tab-9',
    name: 'Tab 9',
    description: 'Switch to tab 9',
    key: '9',
    metaKey: true,
    shiftKey: false,
    altKey: false,
    category: 'tabs'
  }
];

// Create the shortcuts store
function createShortcutsStore() {
  const { subscribe, set, update } = writable(defaultShortcuts);

  return {
    subscribe,
    set,
    update,
    reset: () => set(defaultShortcuts),
    updateShortcut: (id, newShortcut) => {
      update(shortcuts => {
        const index = shortcuts.findIndex(s => s.id === id);
        if (index !== -1) {
          shortcuts[index] = { ...shortcuts[index], ...newShortcut };
        }
        return shortcuts;
      });
    }
  };
}

export const shortcuts = createShortcutsStore();

// Load shortcuts from store
export async function loadShortcuts() {
  try {
    const store = new Store('settings.json');
    const saved = await store.get('shortcuts');
    if (saved && Array.isArray(saved)) {
      shortcuts.set(saved);
    }
  } catch (error) {
    console.error('Failed to load shortcuts:', error);
  }
}

// Save shortcuts to store
export async function saveShortcuts(shortcutsData) {
  try {
    const store = new Store('settings.json');
    await store.set('shortcuts', shortcutsData);
    await store.save();
    shortcuts.set(shortcutsData);
  } catch (error) {
    console.error('Failed to save shortcuts:', error);
  }
}

// Reset shortcuts to defaults
export async function resetShortcuts() {
  try {
    const store = new Store('settings.json');
    await store.set('shortcuts', defaultShortcuts);
    await store.save();
    shortcuts.reset();
  } catch (error) {
    console.error('Failed to reset shortcuts:', error);
  }
}

// Get shortcut display string
export function getShortcutDisplay(shortcut) {
  const parts = [];
  if (shortcut.metaKey) parts.push('⌘');
  if (shortcut.shiftKey) parts.push('⇧');
  if (shortcut.altKey) parts.push('⌥');
  parts.push(shortcut.key.toUpperCase());
  return parts.join('');
}
