<script>
  import { createEventDispatcher, onMount } from 'svelte';
  import { hostsStore, removeAndReloadHost, addAndReloadHost, updateHosts } from '../stores/hosts';
  import { dndzone } from 'svelte-dnd-action';

  const dispatch = createEventDispatcher();

  let searchQuery = '';
  let filteredHosts = [];
  let selectedIndex = -1;
  let searchInput;
  let contextMenu = null;
  let contextMenuHost = null;

  const dndConfig = {
    items: filteredHosts,
    flipDurationMs: 200,
    dragDisabled: searchQuery.trim() !== ''
  };

  function handleDndConsider(e) {
    filteredHosts = e.detail.items;
  }

  function handleDndFinalize(e) {
    filteredHosts = e.detail.items;
    if (searchQuery.trim() === '') {
      updateHosts(filteredHosts);
    }
  }

  $: {
    if (searchQuery.trim() === '') {
      filteredHosts = $hostsStore;
    } else {
      const query = searchQuery.toLowerCase();
      filteredHosts = $hostsStore.filter(host =>
        host.name.toLowerCase().includes(query) ||
        host.host.toLowerCase().includes(query) ||
        host.username.toLowerCase().includes(query)
      );
    }
    dndConfig.items = filteredHosts;
    dndConfig.dragDisabled = searchQuery.trim() !== '';
    selectedIndex = -1;
  }

  onMount(() => {
    if (searchInput) {
      searchInput.focus();
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

  function handleSelect(host) {
    dispatch('select', host);
  }

  function handleEdit(host, event) {
    event.stopPropagation();
    dispatch('edit', host);
  }

  async function handleDuplicate(host, event) {
    event.stopPropagation();
    const duplicatedHost = {
      ...host,
      id: `host-${Date.now()}`,
      name: `${host.name} (Copy)`
    };
    await addAndReloadHost(duplicatedHost);
  }

  async function handleDelete(host, event) {
    event.stopPropagation();
    if (confirm(`Are you sure you want to delete "${host.name}"?`)) {
      await removeAndReloadHost(host.id);
    }
  }

  function handleContextMenu(event, host) {
    event.preventDefault();
    event.stopPropagation();
    contextMenu = { x: event.clientX, y: event.clientY };
    contextMenuHost = host;
  }

  function handleKeyDown(e) {
    if (e.key === 'Escape') {
      e.preventDefault();
      dispatch('close');
      return;
    }

    if (e.key === 'ArrowDown') {
      e.preventDefault();
      selectedIndex = Math.min(selectedIndex + 1, filteredHosts.length - 1);
      return;
    }

    if (e.key === 'ArrowUp') {
      e.preventDefault();
      selectedIndex = Math.max(selectedIndex - 1, -1);
      return;
    }

    if (e.key === 'Enter' && selectedIndex >= 0) {
      e.preventDefault();
      handleSelect(filteredHosts[selectedIndex]);
      return;
    }
  }

  function handleOverlayClick() {
    dispatch('close');
  }
</script>

<div class="modal-overlay" on:click={handleOverlayClick} role="presentation">
  <div class="host-selector" on:click|stopPropagation role="dialog">
    <!-- Search Box -->
    <div class="search-container">
      <div class="search-input-wrapper">
        <svg class="search-icon" width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5">
          <circle cx="6" cy="6" r="5" />
          <path d="M11 11l4 4" />
        </svg>
        <input
          bind:this={searchInput}
          bind:value={searchQuery}
          on:keydown={handleKeyDown}
          type="text"
          placeholder="Search hosts..."
          class="search-input"
        />
        {#if searchQuery}
          <button class="clear-btn" on:click={() => searchQuery = ''}>
            <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M3 3l8 8M11 3l-8 8" />
            </svg>
          </button>
        {/if}
      </div>
    </div>

    <!-- Hosts List -->
    <div
      class="hosts-list"
      use:dndzone={dndConfig}
      on:consider={handleDndConsider}
      on:finalize={handleDndFinalize}
    >
      {#if filteredHosts.length === 0}
        <div class="empty-state">
          {#if searchQuery}
            <p>No hosts found matching "{searchQuery}"</p>
          {:else}
            <p>No saved connections</p>
          {/if}
        </div>
      {:else}
        {#each filteredHosts as host, index (host.id)}
          <div
            class="host-item-wrapper"
            class:selected={index === selectedIndex}
            on:mouseenter={() => selectedIndex = index}
            on:contextmenu={(e) => handleContextMenu(e, host)}
          >

            <button
              class="host-item"
              on:click={() => handleSelect(host)}
            >
              <div class="host-icon">
                <svg width="20" height="20" viewBox="0 0 20 20" fill="currentColor">
                  <path d="M3 4a1 1 0 011-1h12a1 1 0 011 1v2a1 1 0 01-1 1H4a1 1 0 01-1-1V4zM3 10a1 1 0 011-1h6a1 1 0 011 1v6a1 1 0 01-1 1H4a1 1 0 01-1-1v-6zM14 9a1 1 0 00-1 1v6a1 1 0 001 1h2a1 1 0 001-1v-6a1 1 0 00-1-1h-2z"/>
                </svg>
              </div>
              <div class="host-info">
                <div class="host-name">{host.name}</div>
                <div class="host-details">{host.username}@{host.host}</div>
              </div>
            </button>
            <div class="host-actions">
              <button
                class="action-btn edit-btn"
                on:click={(e) => handleEdit(host, e)}
                title="Edit"
              >
                <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="1.5">
                  <path d="M9.5 2L12 4.5L5 11.5L2 12L2.5 9L9.5 2Z"/>
                </svg>
              </button>
              <button
                class="action-btn duplicate-btn"
                on:click={(e) => handleDuplicate(host, e)}
                title="Duplicate"
              >
                <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="1.5">
                  <rect x="3" y="3" width="7" height="7" rx="1"/>
                  <path d="M5 1h6a2 2 0 012 2v6"/>
                </svg>
              </button>
              <button
                class="action-btn delete-btn"
                on:click={(e) => handleDelete(host, e)}
                title="Delete"
              >
                <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="1.5">
                  <path d="M1 3h12M5 1h4M5 6v5M9 6v5"/>
                  <path d="M3 3h8v9a1 1 0 01-1 1H4a1 1 0 01-1-1V3z"/>
                </svg>
              </button>
            </div>
          </div>
        {/each}
      {/if}
    </div>

    <!-- Footer -->
    <div class="footer">
      <div class="shortcut-info">
        Press <kbd>↑↓</kbd> to navigate, <kbd>↵</kbd> to connect, <kbd>Esc</kbd> to close
      </div>
    </div>
  </div>
</div>

<!-- Context Menu -->
{#if contextMenu && contextMenuHost}
  <div
    class="context-menu"
    style="left: {contextMenu.x}px; top: {contextMenu.y}px;"
    on:click|stopPropagation
  >
    <button
      class="context-menu-item"
      on:click={(e) => { handleEdit(contextMenuHost, e); contextMenu = null; }}
    >
      <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="1.5">
        <path d="M9.5 2L12 4.5L5 11.5L2 12L2.5 9L9.5 2Z"/>
      </svg>
      Edit
    </button>
    <button
      class="context-menu-item"
      on:click={(e) => { handleDuplicate(contextMenuHost, e); contextMenu = null; }}
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
      on:click={(e) => { handleDelete(contextMenuHost, e); contextMenu = null; }}
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
  .modal-overlay {
    @apply fixed inset-0 bg-black/40 flex items-start justify-center z-50 pt-20;
    backdrop-filter: blur(2px);
  }

  .host-selector {
    @apply bg-white dark:bg-gray-800 rounded-xl shadow-2xl w-full max-w-md overflow-hidden flex flex-col max-h-96;
    border: 1px solid rgba(59, 130, 246, 0.2);
  }

  /* Search Container */
  .search-container {
    @apply p-4 border-b border-gray-200 dark:border-gray-700 flex-shrink-0;
  }

  .search-input-wrapper {
    @apply flex items-center gap-2 px-3 py-2 rounded-lg;
    @apply bg-gray-100 dark:bg-gray-700 border border-gray-200 dark:border-gray-600;
    @apply focus-within:border-blue-500 dark:focus-within:border-blue-400 transition-colors;
  }

  .search-icon {
    @apply text-gray-400 dark:text-gray-500 flex-shrink-0;
  }

  .search-input {
    @apply flex-1 bg-transparent outline-none text-gray-900 dark:text-white text-sm;
  }

  .search-input::placeholder {
    @apply text-gray-500 dark:text-gray-400;
  }

  .clear-btn {
    @apply p-1 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors flex-shrink-0;
  }

  /* Hosts List */
  .hosts-list {
    @apply flex-1 overflow-y-auto flex flex-col;
  }

  .empty-state {
    @apply flex items-center justify-center py-8 text-gray-500 dark:text-gray-400 text-sm text-center px-4;
  }

  /* Host Items */
  .host-item-wrapper {
    @apply flex items-center py-2 transition-colors border-l-2 border-transparent;
    @apply hover:bg-gray-100 dark:hover:bg-gray-700;
  }

  .host-item {
    @apply flex items-center gap-3 flex-1 text-left bg-transparent border-0 pl-4 pr-3 py-0;
    @apply text-gray-700 dark:text-gray-300 cursor-pointer;
  }

  .host-actions {
    @apply flex gap-1 opacity-0 transition-opacity;
  }

  .host-item-wrapper:hover .host-actions {
    @apply opacity-100;
  }

  .action-btn {
    @apply w-7 h-7 flex items-center justify-center rounded-md transition-all;
    @apply border-0 cursor-pointer;
  }

  .edit-btn {
    @apply bg-blue-100 dark:bg-blue-900/50 text-blue-600 dark:text-blue-400;
    @apply hover:bg-blue-200 dark:hover:bg-blue-800;
  }

  .duplicate-btn {
    @apply bg-green-100 dark:bg-green-900/50 text-green-600 dark:text-green-400;
    @apply hover:bg-green-200 dark:hover:bg-green-800;
  }

  .delete-btn {
    @apply bg-red-100 dark:bg-red-900/50 text-red-600 dark:text-red-400;
    @apply hover:bg-red-200 dark:hover:bg-red-800;
  }

  .host-icon {
    @apply flex items-center justify-center w-10 h-10 rounded-lg bg-blue-100 dark:bg-blue-900 text-blue-600 dark:text-blue-400 flex-shrink-0;
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

  /* Footer */
  .footer {
    @apply px-4 py-3 border-t border-gray-200 dark:border-gray-700 flex-shrink-0;
    @apply bg-gray-50 dark:bg-gray-900/50;
  }

  .shortcut-info {
    @apply text-xs text-gray-500 dark:text-gray-400 flex items-center gap-2 justify-center;
  }

  kbd {
    @apply px-2 py-0.5 rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-700 dark:text-gray-300 font-mono text-xs;
  }

  /* Custom scrollbar */
  .hosts-list::-webkit-scrollbar {
    width: 6px;
  }

  .hosts-list::-webkit-scrollbar-track {
    @apply bg-transparent;
  }

  .hosts-list::-webkit-scrollbar-thumb {
    @apply bg-gray-300 dark:bg-gray-600 rounded-full;
  }

  .hosts-list::-webkit-scrollbar-thumb:hover {
    @apply bg-gray-400 dark:bg-gray-500;
  }

  /* Context Menu */
  .context-menu {
    @apply fixed bg-white dark:bg-gray-800 rounded-lg shadow-xl border border-gray-200 dark:border-gray-700 z-[60];
    @apply py-1 min-w-[160px];
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
