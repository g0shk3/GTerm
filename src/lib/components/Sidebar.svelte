<script>
  import { createEventDispatcher } from 'svelte';
  import { hostsStore } from '../stores/hosts';

  export let isOpen = true;

  const dispatch = createEventDispatcher();

  function selectHost(host) {
    dispatch('connect', host);
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
      <button
        on:click={() => dispatch('manage')}
        class="manage-btn"
      >
        + Manage Connections
      </button>

      <div class="hosts-list">
        {#if $hostsStore.length === 0}
          <div class="empty-state">
            <p>No saved connections</p>
            <p class="hint">Click "Manage Connections" to add</p>
          </div>
        {:else}
          {#each $hostsStore as host (host.id)}
            <button
              class="host-card"
              on:click={() => selectHost(host)}
              title="Connect to {host.name || host.host}"
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
    </div>
  {/if}
</aside>

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
    @apply flex flex-col gap-3 p-4;
    flex: 1;
    overflow-y: auto;
  }

  .manage-btn {
    @apply w-full px-4 py-2.5 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors text-sm font-medium;
    white-space: nowrap;
  }

  .hosts-list {
    @apply flex flex-col gap-2;
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
    @apply flex items-center gap-3 p-3 rounded-lg bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-700;
    @apply hover:border-blue-500 dark:hover:border-blue-500 hover:shadow-md transition-all cursor-pointer;
    text-align: left;
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
</style>
