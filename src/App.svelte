<script>
  import { onMount } from 'svelte';
  import { theme } from './lib/stores/theme';
  import { tabs, activeTabId, createTab, closeTab } from './lib/stores/tabs';
  import { hostsStore } from './lib/stores/hosts';
  import HostManager from './lib/components/HostManager.svelte';
  import Terminal from './lib/components/Terminal.svelte';
  import SFTP from './lib/components/SFTP.svelte';
  import Sidebar from './lib/components/Sidebar.svelte';

  let showHostManager = false;
  let currentView = 'welcome'; // 'welcome', 'tabs'
  let sidebarOpen = true;

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

  function handleSidebarConnect(event) {
    const host = event.detail;
    createTab(host);
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

<div class="app-container">
  <!-- Modern Header -->
  <header class="modern-header">
    <div class="header-center">
      <!-- Modern Tabs -->
      {#if $tabs.length > 0}
        <div class="modern-tabs">
          {#each $tabs as tab (tab.id)}
            <button
              class="modern-tab"
              class:active={tab.id === $activeTabId}
              on:click={() => switchTab(tab.id)}
            >
              <div class="tab-content-wrapper">
                <span
                  class="tab-indicator"
                  class:connected={tab.connected}
                  class:disconnected={!tab.connected}
                />
                <span class="tab-label">{tab.title}</span>
                <button
                  class="tab-close-btn"
                  on:click|stopPropagation={() => handleCloseTab(tab.id)}
                  title="Close"
                >
                  <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M3 3l8 8M11 3l-8 8" />
                  </svg>
                </button>
              </div>
            </button>
          {/each}
        </div>
      {/if}
    </div>

    <div class="header-right">
      <button
        on:click={() => { showHostManager = true; currentView = 'tabs'; }}
        class="header-btn new-connection-btn"
        title="New Connection"
      >
        <svg width="18" height="18" viewBox="0 0 18 18" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M9 3v12M3 9h12" />
        </svg>
        <span>New</span>
      </button>
      <button on:click={() => sidebarOpen = !sidebarOpen} class="header-btn" title="Toggle connections">
        <svg width="18" height="18" viewBox="0 0 18 18" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="2" y="3" width="14" height="12" rx="2" />
          <path d="M6 7h6M6 11h4" />
        </svg>
      </button>
      <button on:click={toggleTheme} class="header-btn" title="Toggle theme">
        {#if $theme === 'dark'}
          <svg width="18" height="18" viewBox="0 0 18 18" fill="currentColor">
            <circle cx="9" cy="9" r="4" />
            <path d="M9 1v2M9 15v2M1 9h2M15 9h2M3.5 3.5l1.4 1.4M13.1 13.1l1.4 1.4M3.5 14.5l1.4-1.4M13.1 4.9l1.4-1.4" stroke="currentColor" stroke-width="1.5" />
          </svg>
        {:else}
          <svg width="18" height="18" viewBox="0 0 18 18" fill="currentColor">
            <path d="M9 2a7 7 0 0 0 6 11.9A7 7 0 1 1 4.1 9 7 7 0 0 0 9 2z" />
          </svg>
        {/if}
      </button>
    </div>
  </header>

  <!-- Main Layout -->
  <div class="main-layout">
    <!-- Content Area -->
    <main class="content-area">
      {#if currentView === 'welcome'}
        <div class="welcome-screen">
          <div class="welcome-content">
            <div class="welcome-icon">
              <svg width="64" height="64" viewBox="0 0 64 64" fill="none" stroke="currentColor" stroke-width="2">
                <rect x="8" y="8" width="48" height="48" rx="4" />
                <path d="M16 24h32M16 32h32M16 40h24" />
              </svg>
            </div>
            <h2 class="welcome-title">Welcome to GTerm</h2>
            <p class="welcome-subtitle">
              Modern, secure SSH terminal for macOS
            </p>
            <button on:click={() => showHostManager = true} class="welcome-btn">
              <svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M10 4v12M4 10h12" />
              </svg>
              Connect to Server
            </button>
          </div>
        </div>
      {:else if currentView === 'tabs'}
        <!-- Render all tabs but show only the active one -->
        {#each $tabs as tab (tab.id)}
          <div class="tab-content" class:hidden={tab.id !== $activeTabId}>
            {#if tab.type === 'terminal'}
              <Terminal {tab} />
            {:else if tab.type === 'sftp'}
              <SFTP {tab} />
            {/if}
          </div>
        {/each}
      {/if}
    </main>

    <!-- Sidebar (Right side) -->
    <Sidebar
      bind:isOpen={sidebarOpen}
      on:connect={handleSidebarConnect}
      on:manage={() => showHostManager = true}
    />
  </div>

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
    @apply flex flex-col w-full h-screen overflow-hidden;
    @apply bg-gradient-to-br from-gray-50 to-gray-100 dark:from-gray-900 dark:to-gray-800;
  }

  /* Modern Header */
  .modern-header {
    @apply flex items-center justify-between px-6 py-3;
    @apply bg-white/80 dark:bg-gray-900/80 backdrop-blur-lg;
    @apply border-b border-gray-200/50 dark:border-gray-700/50;
    flex-shrink: 0;
    z-index: 10;
  }

  .header-right {
    @apply flex items-center gap-3;
    flex: 0 0 auto;
  }

  .header-center {
    @apply flex-1 flex items-center justify-start;
    min-width: 0;
  }

  .header-btn {
    @apply flex items-center gap-2 px-3 py-2 rounded-lg;
    @apply text-gray-700 dark:text-gray-300;
    @apply hover:bg-gray-100 dark:hover:bg-gray-800;
    @apply transition-all duration-200;
    font-size: 14px;
    font-weight: 500;
  }

  .new-connection-btn {
    @apply bg-blue-600 text-white;
    @apply hover:bg-blue-700 dark:hover:bg-blue-600;
  }

  /* Modern Tabs */
  .modern-tabs {
    @apply flex items-center gap-2 flex-1;
    overflow-x: auto;
    scrollbar-width: none;
    min-width: 0;
  }

  .modern-tabs::-webkit-scrollbar {
    display: none;
  }

  .modern-tab {
    @apply relative px-5 py-2 rounded-lg;
    @apply bg-transparent hover:bg-gray-100 dark:hover:bg-gray-800;
    @apply transition-all duration-200;
    border: 1px solid transparent;
    min-width: max-content;
    flex-shrink: 0;
  }

  .modern-tab.active {
    @apply bg-white dark:bg-gray-800;
    @apply border-gray-200 dark:border-gray-700;
    @apply shadow-sm;
  }

  .tab-content-wrapper {
    @apply flex items-center gap-2;
  }

  .tab-indicator {
    @apply w-2 h-2 rounded-full;
    transition: background-color 0.2s ease;
  }

  .tab-indicator.connected {
    @apply bg-green-500;
    box-shadow: 0 0 6px rgba(34, 197, 94, 0.5);
  }

  .tab-indicator.disconnected {
    @apply bg-red-500;
    box-shadow: 0 0 6px rgba(239, 68, 68, 0.5);
  }

  .tab-label {
    @apply text-sm font-medium text-gray-700 dark:text-gray-300;
  }

  .modern-tab.active .tab-label {
    @apply text-gray-900 dark:text-white;
  }

  .tab-close-btn {
    @apply p-1 rounded opacity-0 hover:bg-red-100 dark:hover:bg-red-900/30;
    @apply text-gray-400 hover:text-red-600 dark:hover:text-red-400;
    @apply transition-all duration-200;
  }

  .modern-tab:hover .tab-close-btn,
  .modern-tab.active .tab-close-btn {
    opacity: 1;
  }

  /* Main Layout */
  .main-layout {
    @apply flex flex-1 overflow-hidden;
  }

  .content-area {
    @apply flex-1 overflow-hidden relative;
  }

  .tab-content {
    @apply w-full h-full absolute top-0 left-0;
  }

  .tab-content.hidden {
    display: none;
  }

  /* Welcome Screen */
  .welcome-screen {
    @apply flex items-center justify-center h-full;
  }

  .welcome-content {
    @apply text-center;
    animation: fadeIn 0.5s ease-out;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(20px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .welcome-icon {
    @apply inline-flex items-center justify-center w-20 h-20 mb-6;
    @apply rounded-2xl bg-blue-100 dark:bg-blue-900/30;
    @apply text-blue-600 dark:text-blue-500;
  }

  .welcome-title {
    @apply text-4xl font-bold mb-3;
    @apply bg-gradient-to-r from-gray-900 to-gray-600 dark:from-white dark:to-gray-400;
    @apply bg-clip-text text-transparent;
  }

  .welcome-subtitle {
    @apply text-lg text-gray-600 dark:text-gray-400 mb-8;
  }

  .welcome-btn {
    @apply inline-flex items-center gap-2 px-6 py-3;
    @apply bg-blue-600 hover:bg-blue-700 text-white;
    @apply rounded-xl font-medium;
    @apply transition-all duration-200;
    @apply shadow-lg shadow-blue-500/30 hover:shadow-xl hover:shadow-blue-500/40;
  }
</style>
