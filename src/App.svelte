<script>
  import { onMount } from 'svelte';
  import { theme } from './lib/stores/theme';
  import { tabs, activeTabId, createTab, closeTab } from './lib/stores/tabs';
  import { hostsStore } from './lib/stores/hosts';
  import HostManager from './lib/components/HostManager.svelte';
  import Terminal from './lib/components/Terminal.svelte';
  import SFTP from './lib/components/SFTP.svelte';

  let showHostManager = false;
  let currentView = 'welcome'; // 'welcome', 'tabs'

  onMount(() => {
    // Initialize theme
    if ($theme === 'dark') {
      document.documentElement.classList.add('dark');
    }
  });

  function toggleTheme() {
    theme.update(t => t === 'dark' ? 'light' : 'dark');
  }

  function handleNewConnection(event) {
    const host = event.detail;
    createTab(host);
    showHostManager = false;
    currentView = 'tabs';
  }

  function handleCloseTab(tabId) {
    closeTab(tabId);
    if ($tabs.length === 0) {
      currentView = 'welcome';
    }
  }

  function switchTab(tabId) {
    activeTabId.set(tabId);
  }

  $: activeTab = $tabs.find(t => t.id === $activeTabId);
</script>

<div class="app-container bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100">
  <!-- Header -->
  <header class="header bg-gray-100 dark:bg-gray-800 border-b border-gray-300 dark:border-gray-700">
    <div class="flex items-center justify-between px-4 py-2">
      <div class="flex items-center space-x-4">
        <h1 class="text-lg font-bold">GTerm</h1>
        <button
          on:click={() => { showHostManager = true; currentView = 'tabs'; }}
          class="btn-primary"
        >
          + New Connection
        </button>
      </div>

      <div class="flex items-center space-x-2">
        <button on:click={toggleTheme} class="btn-icon" title="Toggle theme">
          {#if $theme === 'dark'}
            ☀️
          {:else}
            🌙
          {/if}
        </button>
      </div>
    </div>

    <!-- Tabs -->
    {#if $tabs.length > 0}
      <div class="tabs-container flex overflow-x-auto border-t border-gray-300 dark:border-gray-700">
        {#each $tabs as tab (tab.id)}
          <div
            class="tab"
            class:active={tab.id === $activeTabId}
            on:click={() => switchTab(tab.id)}
            on:keydown={(e) => e.key === 'Enter' && switchTab(tab.id)}
            role="tab"
            tabindex="0"
          >
            <span class="tab-title">{tab.title}</span>
            <span
              class="tab-status"
              class:connected={tab.connected}
              class:disconnected={!tab.connected}
            >
              ●
            </span>
            <button
              class="tab-close"
              on:click|stopPropagation={() => handleCloseTab(tab.id)}
              title="Close tab"
            >
              ×
            </button>
          </div>
        {/each}
      </div>
    {/if}
  </header>

  <!-- Main content -->
  <main class="main-content">
    {#if currentView === 'welcome'}
      <div class="welcome-screen">
        <div class="welcome-content">
          <h2 class="text-3xl font-bold mb-4">Welcome to GTerm</h2>
          <p class="text-gray-600 dark:text-gray-400 mb-8">
            Fast, secure SSH terminal for macOS
          </p>
          <button on:click={() => showHostManager = true} class="btn-primary-large">
            Connect to Server
          </button>
        </div>
      </div>
    {:else if currentView === 'tabs' && activeTab}
      {#if activeTab.type === 'terminal'}
        <Terminal tab={activeTab} />
      {:else if activeTab.type === 'sftp'}
        <SFTP tab={activeTab} />
      {/if}
    {/if}
  </main>

  <!-- Host Manager Modal -->
  {#if showHostManager}
    <HostManager
      on:connect={handleNewConnection}
      on:close={() => showHostManager = false}
    />
  {/if}
</div>

<style>
  .app-container {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100vh;
    overflow: hidden;
  }

  .header {
    flex-shrink: 0;
  }

  .main-content {
    flex: 1;
    overflow: hidden;
  }

  .btn-primary {
    @apply px-4 py-1.5 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors text-sm font-medium;
  }

  .btn-primary-large {
    @apply px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors text-base font-medium;
  }

  .btn-icon {
    @apply p-2 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors;
  }

  .tabs-container {
    gap: 0;
  }

  .tab {
    @apply flex items-center gap-2 px-4 py-2 bg-gray-200 dark:bg-gray-700 border-r border-gray-300 dark:border-gray-600 cursor-pointer transition-colors min-w-max;
  }

  .tab:hover {
    @apply bg-gray-300 dark:bg-gray-600;
  }

  .tab.active {
    @apply bg-white dark:bg-gray-900 border-b-2 border-blue-600;
  }

  .tab-title {
    @apply text-sm;
  }

  .tab-status {
    @apply text-xs;
  }

  .tab-status.connected {
    @apply text-green-500;
  }

  .tab-status.disconnected {
    @apply text-red-500;
  }

  .tab-close {
    @apply text-xl leading-none hover:text-red-500 transition-colors;
  }

  .welcome-screen {
    @apply flex items-center justify-center h-full;
  }

  .welcome-content {
    @apply text-center;
  }
</style>
