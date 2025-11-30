import { writable, get } from 'svelte/store';

export const tabs = writable([]);
export const activeTabId = writable(null);

let tabCounter = 0;
let paneCounter = 0;

function createPane(host, type = 'terminal') {
  return {
    id: `pane-${++paneCounter}`,
    sessionId: `session-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
    host,
    type,
    connected: false,
  };
}

export function createTab(host) {
  const id = `tab-${++tabCounter}`;
  const mainPane = createPane(host);

  const newTab = {
    id,
    title: host.name || host.host,
    splitLayout: 'none', // 'none', 'vertical', 'horizontal'
    panes: [mainPane],
    activePaneId: mainPane.id,
    // Legacy fields for backward compatibility
    sessionId: mainPane.sessionId,
    host: mainPane.host,
    type: mainPane.type,
    connected: mainPane.connected,
  };

  tabs.update(t => [...t, newTab]);
  activeTabId.set(id);

  return newTab;
}

export function closeTab(id) {
  // Find the index of the tab we are closing, BEFORE it's removed.
  const allTabs = get(tabs);
  const closingTabIndex = allTabs.findIndex(tab => tab.id === id);

  // If tab not found, do nothing.
  if (closingTabIndex === -1) {
    return;
  }

  // Update the tabs list by filtering out the closed tab.
  tabs.update(t => t.filter(tab => tab.id !== id));

  // Update the active tab ID.
  activeTabId.update(currentActiveId => {
    // Only proceed if we are closing the currently active tab.
    if (currentActiveId === id) {
      const remainingTabs = get(tabs); // This now has the tab removed.

      // If no tabs are left, there's no active tab.
      if (remainingTabs.length === 0) {
        return null;
      }

      // Determine the new index. Try to keep it the same.
      // If the closed tab was the last one, the new index will be out of bounds,
      // so we take the new last index.
      const newIndex = Math.min(closingTabIndex, remainingTabs.length - 1);

      // Get the ID of the new active tab.
      const newActiveTabId = remainingTabs[newIndex].id;

      // Dispatch event to handle focus in the UI.
      setTimeout(() => {
        window.dispatchEvent(new CustomEvent('tabSwitched'));
      }, 50);

      return newActiveTabId;
    }

    // If we closed a background tab, the active tab doesn't change.
    return currentActiveId;
  });
}

export function renameTab(id, newTitle) {
  tabs.update(t => {
    return t.map(tab => {
      if (tab.id === id) {
        return { ...tab, title: newTitle };
      }
      return tab;
    });
  });
}

export function duplicateTab(id) {
  const currentTabs = get(tabs);
  const tab = currentTabs.find(t => t.id === id);
  if (!tab) return;

  // Create new tab with the same host
  const newTab = createTab(tab.host);
  return newTab;
}

export function reorderTabs(newTabsOrder) {
  tabs.set(newTabsOrder);
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

// Split pane functions
export function splitPane(tabId, direction = 'vertical') {
  tabs.update(t => {
    const tab = t.find(tab => tab.id === tabId);
    if (!tab) return t;

    // Get the active pane to clone its host
    const activePane = tab.panes.find(p => p.id === tab.activePaneId) || tab.panes[0];
    const newPane = createPane(activePane.host, activePane.type);

    // Add new pane
    tab.panes.push(newPane);
    tab.splitLayout = direction;
    tab.activePaneId = newPane.id;

    return t;
  });
}

export function closePane(tabId, paneId) {
  tabs.update(t => {
    const tab = t.find(tab => tab.id === tabId);
    if (!tab || tab.panes.length <= 1) return t;

    // Find the index of the pane being closed BEFORE it's removed
    const closingPaneIndex = tab.panes.findIndex(p => p.id === paneId);

    // Remove the pane
    tab.panes = tab.panes.filter(p => p.id !== paneId);

    // If we closed the active pane, switch to another
    if (tab.activePaneId === paneId) {
      // Determine the new index. Try to select the previous pane.
      const newIndex = Math.max(0, closingPaneIndex - 1);
      tab.activePaneId = tab.panes[newIndex].id;
    }

    // If only one pane left, reset layout
    if (tab.panes.length === 1) {
      tab.splitLayout = 'none';
    }

    // Update legacy fields for compatibility
    const mainPane = tab.panes[0];
    tab.sessionId = mainPane.sessionId;
    tab.host = mainPane.host;
    tab.type = mainPane.type;
    tab.connected = mainPane.connected;

    // Dispatch a resize event to ensure the remaining panes are resized
    setTimeout(() => window.dispatchEvent(new Event('resize')), 50);

    return t;
  });
}

export function setActivePane(tabId, paneId) {
  tabs.update(t => {
    const tab = t.find(tab => tab.id === tabId);
    if (tab) {
      tab.activePaneId = paneId;
    }
    return t;
  });
}

export function updatePaneConnection(tabId, paneId, connected) {
  tabs.update(t => {
    const tab = t.find(tab => tab.id === tabId);
    if (tab) {
      const pane = tab.panes.find(p => p.id === paneId);
      if (pane) {
        pane.connected = connected;
        // Update tab connection status (all panes must be connected)
        tab.connected = tab.panes.every(p => p.connected);
      }
    }
    return t;
  });
}
