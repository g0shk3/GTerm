import { writable } from 'svelte/store';

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
      let newActive = null;
      tabs.subscribe(t => {
        if (t.length > 0) {
          newActive = t[t.length - 1].id;
        }
      })();
      return newActive;
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
