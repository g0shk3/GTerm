<script>
  import { createEventDispatcher, onMount } from 'svelte';
  import { hostsStore, removeAndReloadHost, addAndReloadHost, updateHosts } from '../stores/hosts';
  import { dndzone } from 'svelte-dnd-action';
  import { getVersion } from '@tauri-apps/api/app';

  export let isOpen = true;

  const dispatch = createEventDispatcher();

  let appVersion = '1.0.1';

  let selectedHostId = null;
  let contextMenu = null;
  let contextMenuHost = null;
  let hosts = [];

  $: hosts = $hostsStore;

  const dndConfig = {
    items: hosts,
    flipDurationMs: 200
  };

  function handleDndConsider(e) {
    hosts = e.detail.items;
  }

  function handleDndFinalize(e) {
    hosts = e.detail.items;
    updateHosts(hosts);
  }

  onMount(async () => {
    // Get app version
    try {
      appVersion = await getVersion();
    } catch (err) {
      console.error('Failed to get app version:', err);
    }

    // Close context menu when clicking anywhere
    const handleClick = () => {
      contextMenu = null;
      contextMenuHost = null;
    };
    document.addEventListener('click', handleClick);

    return () => {
      document.removeEventListener('click', handleClick);
    };
  });

  function markHost(event, host) {
    event.preventDefault();
    selectedHostId = host.id;
  }

  function connectHost(host) {
    dispatch('connect', host);
  }

  function editHost(event, host) {
    event.stopPropagation();
    dispatch('edit', host);
    contextMenu = null;
  }

  async function duplicateHost(event, host) {
    event.stopPropagation();
    const duplicatedHost = {
      ...host,
      id: `host-${Date.now()}`,
      name: `${host.name} (Copy)`
    };
    await addAndReloadHost(duplicatedHost);
    contextMenu = null;
  }

  async function deleteHost(event, host) {
    event.stopPropagation();
    if (confirm(`Are you sure you want to delete "${host.name}"?`)) {
      await removeAndReloadHost(host.id);
      if (selectedHostId === host.id) {
        selectedHostId = null;
      }
    }
    contextMenu = null;
  }

  function handleContextMenu(event, host) {
    event.preventDefault();
    event.stopPropagation();

    // Calculate menu position with boundary checks
    const menuWidth = 180; // Approximate context menu width
    const menuHeight = 120; // Approximate context menu height
    const padding = 10; // Padding from edges

    let x = event.clientX;
    let y = event.clientY;

    // Check right boundary
    if (x + menuWidth > window.innerWidth - padding) {
      x = window.innerWidth - menuWidth - padding;
    }

    // Check bottom boundary
    if (y + menuHeight > window.innerHeight - padding) {
      y = window.innerHeight - menuHeight - padding;
    }

    // Check left boundary
    if (x < padding) {
      x = padding;
    }

    // Check top boundary
    if (y < padding) {
      y = padding;
    }

    contextMenu = { x, y };
    contextMenuHost = host;
    selectedHostId = host.id;
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
      <div
        class="hosts-list"
        use:dndzone={{ items: hosts, flipDurationMs: 200 }}
        on:consider={handleDndConsider}
        on:finalize={handleDndFinalize}
      >
        {#if hosts.length === 0}
          <div class="empty-state">
            <p>No saved connections</p>
            <p class="hint">Click "Manage Connections" to add</p>
          </div>
        {:else}
          {#each hosts as host (host.id)}
            <button
              class="host-card"
              class:selected={selectedHostId === host.id}
              on:click={(e) => markHost(e, host)}
              on:dblclick={() => connectHost(host)}
              on:contextmenu={(e) => handleContextMenu(e, host)}
              title="Double-click to connect, right-click for options"
            >

              <div class="host-icon">
                <svg width="20" height="20" viewBox="0 0 20 20" fill="currentColor">
                  <path d="M3 4a1 1 0 011-1h12a1 1 0 011 1v2a1 1 0 01-1 1H4a1 1 0 01-1-1V4zM3 10a1 1 0 011-1h6a1 1 0 011 1v6a1 1 0 01-1 1H4a1 1 0 01-1-1v-6zM14 9a1 1 0 00-1 1v6a1 1 0 001 1h2a1 1 0 001-1v-6a1 1 0 00-1-1h-2z"/>
                </svg>
              </div>
              <div class="host-info">
                <div class="host-name">{host.name || host.host}</div>
                <div class="host-details">{host.username}@{host.host}</div>
              </div>
            </button>
          {/each}
        {/if}
      </div>

      <button
        on:click={() => dispatch('manage')}
        class="manage-btn"
      >
        + Manage Connections
      </button>

      <div class="version-info">
        GTerm v{appVersion}
      </div>
    </div>
  {/if}
</aside>

<!-- Context Menu -->
{#if contextMenu && contextMenuHost}
  <div
    class="context-menu"
    style="left: {contextMenu.x}px; top: {contextMenu.y}px;"
    on:click|stopPropagation
  >
    <button
      class="context-menu-item"
      on:click={(e) => connectHost(contextMenuHost)}
    >
      <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="1.5">
        <path d="M7 1v12M1 7h12"/>
      </svg>
      Connect
    </button>
    <button
      class="context-menu-item"
      on:click={(e) => editHost(e, contextMenuHost)}
    >
      <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="1.5">
        <path d="M9.5 2L12 4.5L5 11.5L2 12L2.5 9L9.5 2Z"/>
      </svg>
      Edit
    </button>
    <button
      class="context-menu-item"
      on:click={(e) => duplicateHost(e, contextMenuHost)}
    >
      <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="1.5">
        <rect x="3" y="3" width="7" height="7" rx="1"/>
        <path d="M5 1h6a2 2 0 012 2v6"/>
      </svg>
      Duplicate
    </button>
    <div class="context-menu-divider"></div>
    <button
      class="context-menu-item danger"
      on:click={(e) => deleteHost(e, contextMenuHost)}
    >
      <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="1.5">
        <path d="M1 3h12M5 1h4M5 6v5M9 6v5"/>
        <path d="M3 3h8v9a1 1 0 01-1 1H4a1 1 0 01-1-1V3z"/>
      </svg>
      Delete
    </button>
  </div>
{/if}

<style>
  .sidebar {
    @apply bg-gray-50 dark:bg-gray-800 border-l border-gray-200 dark:border-gray-700;
    width: 280px;
    flex-shrink: 0;
    transition: width 0.3s ease;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .sidebar:not(.open) {
    width: 0;
    border: none;
  }

  .sidebar-header {
    @apply flex items-center gap-3 px-4 py-3 border-b border-gray-200 dark:border-gray-700;
    flex-shrink: 0;
  }

  .sidebar-title {
    @apply text-sm font-semibold text-gray-700 dark:text-gray-300 flex-1;
    white-space: nowrap;
  }

  .toggle-btn {
    @apply p-1.5 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors text-gray-600 dark:text-gray-400;
    font-size: 18px;
    line-height: 1;
    flex-shrink: 0;
  }

  .sidebar-content {
    @apply flex flex-col p-4 gap-4;
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow-y: hidden;
  }

  .hosts-list {
    @apply flex flex-col gap-2 flex-1 overflow-y-auto;
  }

  .manage-btn {
    @apply w-full px-4 py-2.5 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors text-sm font-medium;
    white-space: nowrap;
    flex-shrink: 0;
    margin-top: auto;
  }

  .version-info {
    @apply text-center text-xs text-gray-400 dark:text-gray-500 py-2;
    flex-shrink: 0;
  }

  .empty-state {
    @apply text-center py-8 text-gray-500 dark:text-gray-400;
  }

  .empty-state p {
    @apply text-sm;
  }

  .empty-state .hint {
    @apply text-xs mt-2 text-gray-400 dark:text-gray-500;
  }

  .host-card {
    @apply w-full flex items-center gap-4 pl-3 pr-3 py-3 rounded-lg bg-white dark:bg-gray-900 border-2 border-gray-200 dark:border-gray-700;
    @apply hover:border-blue-400 dark:hover:border-blue-400 hover:shadow-md transition-all cursor-pointer text-left;
  }

  .host-card.selected {
    @apply border-blue-500 dark:border-blue-500 bg-blue-50 dark:bg-blue-950/20 shadow-lg;
    @apply border-2;
  }

  .host-card.selected .host-icon {
    @apply bg-blue-100 dark:bg-blue-900 text-blue-600 dark:text-blue-400;
  }

  .host-icon {
    @apply flex items-center justify-center w-10 h-10 rounded-lg bg-blue-100 dark:bg-blue-900 text-blue-600 dark:text-blue-400;
    flex-shrink: 0;
  }

  .host-info {
    @apply flex-1 min-w-0;
  }

  .host-name {
    @apply text-sm font-medium text-gray-900 dark:text-gray-100 truncate;
  }

  .host-details {
    @apply text-xs text-gray-500 dark:text-gray-400 truncate;
  }

  /* Custom scrollbar */
  .sidebar-content::-webkit-scrollbar {
    width: 6px;
  }

  .sidebar-content::-webkit-scrollbar-track {
    @apply bg-transparent;
  }

  .sidebar-content::-webkit-scrollbar-thumb {
    @apply bg-gray-300 dark:bg-gray-600 rounded-full;
  }

  .sidebar-content::-webkit-scrollbar-thumb:hover {
    @apply bg-gray-400 dark:bg-gray-500;
  }

  /* Context Menu */
  .context-menu {
    @apply fixed bg-white dark:bg-gray-800 rounded-lg shadow-xl border border-gray-200 dark:border-gray-700 z-[60];
    @apply py-1 min-w-[180px];
  }

  .context-menu-item {
    @apply w-full px-3 py-2 text-left text-sm flex items-center gap-2;
    @apply text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700;
    @apply transition-colors border-0 bg-transparent cursor-pointer;
  }

  .context-menu-item.danger {
    @apply text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/30;
  }

  .context-menu-divider {
    @apply h-px bg-gray-200 dark:bg-gray-700 my-1;
  }

  .drag-handle {
    @apply cursor-grab text-gray-400 dark:text-gray-500 pr-2;
  }

  :global([dnd-zone]) {
    outline: none !important;
    border: none !important;
    box-shadow: none !important;
  }
</style>
