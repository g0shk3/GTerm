<script>
  import { onMount, onDestroy } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { theme } from './lib/stores/theme';
  import { tabs, activeTabId, createTab, closeTab, splitPane, renameTab, duplicateTab } from './lib/stores/tabs';
  import { hostsStore, loadHosts } from './lib/stores/hosts';
  import { loadSnippets } from './lib/stores/snippets';
  import HostManager from './lib/components/HostManager.svelte';
  import HostSelector from './lib/components/HostSelector.svelte';
  import Terminal from './lib/components/Terminal.svelte';
  import SFTP from './lib/components/SFTP.svelte';
  import SplitPane from './lib/components/SplitPane.svelte';
  import Sidebar from './lib/components/Sidebar.svelte';

  let showHostManager = false;
  let showHostSelector = false;
  let currentView = 'welcome'; // 'welcome', 'tabs'
  let sidebarOpen = true;
  let editingHost = null;
  let tabContextMenu = null;
  let contextMenuTab = null;
  let editingTabId = null;
  let renameInput = '';

  onMount(async () => {
    // Initialize theme
    if ($theme === 'dark') {
      document.documentElement.classList.add('dark');
    }

    // Load saved hosts and snippets
    await loadHosts();
    await loadSnippets();

    // Close context menu when clicking anywhere
    const handleClick = () => {
      tabContextMenu = null;
      contextMenuTab = null;
    };
    document.addEventListener('click', handleClick);

    // Register keyboard shortcuts - премахни стари, ако има
    document.removeEventListener('keydown', keyboardHandler);
    document.addEventListener('keydown', keyboardHandler);

    return () => {
      document.removeEventListener('click', handleClick);
    };
  });

  onDestroy(() => {
    // Cleanup keyboard shortcuts
    document.removeEventListener('keydown', keyboardHandler);
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

  function handleSidebarEdit(event) {
    const host = event.detail;
    editingHost = host;
    showHostManager = true;
  }

  function createLocalTerminal() {
    const localHost = {
      id: `local-${Date.now()}`,
      name: 'Local Terminal',
      type: 'local',
      shell: null, // Will use default shell
      cwd: null, // Will use HOME directory
    };
    createTab(localHost);
    currentView = 'tabs';
  }

  function handleHostSelectorSelect(event) {
    const host = event.detail;
    createTab(host);
    showHostSelector = false;
    currentView = 'tabs';
  }

  function handleHostSelectorEdit(event) {
    const host = event.detail;
    editingHost = host;
    showHostSelector = false;
    showHostManager = true;
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

  function handleTabContextMenu(event, tab) {
    event.preventDefault();
    event.stopPropagation();
    tabContextMenu = { x: event.clientX, y: event.clientY };
    contextMenuTab = tab;
  }

  function handleRenameTab() {
    if (!contextMenuTab) return;
    editingTabId = contextMenuTab.id;
    renameInput = contextMenuTab.title;
    tabContextMenu = null;
    // Focus the input after it renders
    setTimeout(() => {
      const input = document.querySelector('.tab-rename-input');
      if (input) {
        input.focus();
        input.select();
      }
    }, 0);
  }

  function confirmRename() {
    if (!editingTabId || !renameInput.trim()) {
      cancelRename();
      return;
    }
    renameTab(editingTabId, renameInput.trim());
    editingTabId = null;
    renameInput = '';
  }

  function cancelRename() {
    editingTabId = null;
    renameInput = '';
  }

  function handleRenameKeydown(e) {
    if (e.key === 'Enter') {
      e.preventDefault();
      confirmRename();
    } else if (e.key === 'Escape') {
      e.preventDefault();
      cancelRename();
    }
  }

  function handleDuplicateTab() {
    if (!contextMenuTab) return;
    duplicateTab(contextMenuTab.id);
    tabContextMenu = null;
  }

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
      getCurrentWindow().close();
      return;
    }

    // Cmd+K - изчисти терминала
    if (e.metaKey && e.key === 'k') {
      e.preventDefault();
      // Dispatch global event за clear на терминала
      window.dispatchEvent(new CustomEvent('clearTerminal'));
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

    // Cmd+Shift+E - вертикален split (ляво/дясно)
    // Проверка ПРЕДИ Cmd+E, защото Shift прави key да е 'E' (главна буква)
    if (e.metaKey && e.shiftKey && (e.key === 'E' || e.key === 'e')) {
      e.preventDefault();
      if ($activeTabId) {
        splitPane($activeTabId, 'vertical');
      }
      return;
    }

    // Cmd+E - отвори host selector (быстър избор на профил)
    if (e.metaKey && !e.shiftKey && (e.key === 'e' || e.key === 'E')) {
      e.preventDefault();
      showHostSelector = true;
      return;
    }

    // Cmd+1 до Cmd+9 - превключи към таб 1-9
    if (e.metaKey && e.key >= '1' && e.key <= '9') {
      e.preventDefault();
      const tabIndex = parseInt(e.key) - 1;
      if (tabIndex < $tabs.length) {
        switchTab($tabs[tabIndex].id);
      }
      return;
    }

    // Cmd+[ и Cmd+] - превключи към предишен/следващ таб
    if (e.metaKey && (e.key === '[' || e.key === ']')) {
      e.preventDefault();
      const currentIndex = $tabs.findIndex(t => t.id === $activeTabId);
      if (currentIndex !== -1) {
        let newIndex;
        if (e.key === '[') {
          // Предишен таб (с wraparound)
          newIndex = currentIndex > 0 ? currentIndex - 1 : $tabs.length - 1;
        } else {
          // Следващ таб (с wraparound)
          newIndex = currentIndex < $tabs.length - 1 ? currentIndex + 1 : 0;
        }
        switchTab($tabs[newIndex].id);
      }
      return;
    }

    // Cmd+T - нов connection (отвори host manager за добавяне)
    if (e.metaKey && e.key === 't') {
      e.preventDefault();
      showHostManager = true;
      editingHost = null;
      currentView = 'tabs'; // Switch to tabs view
      return;
    }

    // Cmd+, - toggle sidebar
    if (e.metaKey && e.key === ',') {
      e.preventDefault();
      sidebarOpen = !sidebarOpen;
      return;
    }

    // Cmd+D - хоризонтален split (горе/долу)
    if (e.metaKey && !e.shiftKey && e.key === 'd') {
      e.preventDefault();
      if ($activeTabId) {
        splitPane($activeTabId, 'horizontal');
      }
      return;
    }
  };

  $: activeTab = $tabs.find(t => t.id === $activeTabId);
</script>

<div class="app-container">
  <!-- Modern Header -->
  <header class="modern-header" data-tauri-drag-region>
    <div class="header-center" data-tauri-drag-region>
      <!-- Modern Tabs -->
      <div class="modern-tabs" data-tauri-drag-region>
        {#each $tabs as tab (tab.id)}
          <button
            class="modern-tab"
            class:active={tab.id === $activeTabId}
            class:editing={editingTabId === tab.id}
            on:click={() => switchTab(tab.id)}
            on:contextmenu={(e) => handleTabContextMenu(e, tab)}
          >
            <div class="tab-content-wrapper">
              <span
                class="tab-indicator"
                class:connected={tab.connected}
                class:disconnected={!tab.connected}
              />
              {#if editingTabId === tab.id}
                <input
                  type="text"
                  class="tab-rename-input"
                  bind:value={renameInput}
                  on:keydown={handleRenameKeydown}
                  on:blur={confirmRename}
                  on:click|stopPropagation
                />
              {:else}
                <span class="tab-label">{tab.title}</span>
              {/if}
              {#if editingTabId !== tab.id}
                <button
                  class="tab-close-btn"
                  on:click|stopPropagation={() => handleCloseTab(tab.id)}
                  title="Close"
                >
                  <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M3 3l8 8M11 3l-8 8" />
                  </svg>
                </button>
              {/if}
            </div>
          </button>
        {/each}
      </div>
    </div>

    <div class="header-right">
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
            <div class="welcome-buttons">
              <button on:click={createLocalTerminal} class="welcome-btn welcome-btn-primary">
                <svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="2">
                  <rect x="3" y="3" width="14" height="14" rx="2" />
                  <path d="M6 8l3 3-3 3M10 14h4" />
                </svg>
                Local Terminal
              </button>
              <button on:click={() => showHostManager = true} class="welcome-btn">
                <svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M10 4v12M4 10h12" />
                </svg>
                Connect to Server
              </button>
            </div>
          </div>
        </div>
      {:else if currentView === 'tabs'}
        <!-- Render all tabs but show only the active one -->
        {#each $tabs as tab (tab.id)}
          <div class="tab-content" class:hidden={tab.id !== $activeTabId}>
            {#if tab.splitLayout !== 'none'}
              <!-- Render split panes -->
              <SplitPane {tab} />
            {:else}
              <!-- Render single pane (legacy mode) -->
              {#if tab.type === 'terminal'}
                <Terminal {tab} />
              {:else if tab.type === 'sftp'}
                <SFTP {tab} />
              {/if}
            {/if}
          </div>
        {/each}
      {/if}
    </main>

    <!-- Sidebar (Right side) -->
    <Sidebar
      bind:isOpen={sidebarOpen}
      on:connect={handleSidebarConnect}
      on:edit={handleSidebarEdit}
      on:manage={() => { showHostManager = true; editingHost = null; }}
    />
  </div>

  <!-- Host Manager Modal -->
  {#if showHostManager}
    <HostManager
      {editingHost}
      on:connect={handleNewConnection}
      on:close={() => { showHostManager = false; editingHost = null; }}
    />
  {/if}

  <!-- Host Selector Menu (Cmd+E) -->
  {#if showHostSelector}
    <HostSelector
      on:select={handleHostSelectorSelect}
      on:edit={handleHostSelectorEdit}
      on:close={() => { showHostSelector = false; }}
    />
  {/if}

  <!-- Tab Context Menu -->
  {#if tabContextMenu && contextMenuTab}
    <div
      class="context-menu"
      style="left: {tabContextMenu.x}px; top: {tabContextMenu.y}px;"
      on:click|stopPropagation
    >
      <button
        class="context-menu-item"
        on:click={handleRenameTab}
      >
        <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="1.5">
          <path d="M9.5 2L12 4.5L5 11.5L2 12L2.5 9L9.5 2Z"/>
        </svg>
        Rename
      </button>
      <button
        class="context-menu-item"
        on:click={handleDuplicateTab}
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
        on:click={() => { handleCloseTab(contextMenuTab.id); tabContextMenu = null; }}
      >
        <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M3 3l8 8M11 3l-8 8" />
        </svg>
        Close
      </button>
    </div>
  {/if}
</div>

<style>
  .app-container {
    @apply flex flex-col w-full h-screen overflow-hidden;
    background-color: #1f2937; /* Same as terminal - gray-800 */
  }

  /* Modern Header */
  .modern-header {
    @apply flex items-center justify-between;
    @apply bg-transparent;
    flex-shrink: 0;
    z-index: 10;
    padding-top: 6px;
    padding-bottom: 6px;
    padding-left: 80px; /* Space for macOS traffic lights */
    padding-right: 24px;
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

  .tab-rename-input {
    @apply px-2 py-0.5 text-sm bg-white dark:bg-gray-700 text-gray-900 dark:text-white;
    @apply border border-blue-500 dark:border-blue-400 rounded;
    @apply outline-none;
    min-width: 80px;
    max-width: 200px;
  }

  .modern-tab.editing {
    @apply bg-white dark:bg-gray-800;
    @apply border-gray-200 dark:border-gray-700;
    @apply shadow-sm;
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

  .welcome-buttons {
    @apply flex flex-col gap-3 w-full max-w-sm;
  }

  .welcome-btn {
    @apply inline-flex items-center justify-center gap-2 px-6 py-3;
    @apply bg-gray-700 hover:bg-gray-600 text-white;
    @apply rounded-xl font-medium;
    @apply transition-all duration-200;
    @apply shadow-lg hover:shadow-xl;
    @apply w-full;
  }

  .welcome-btn-primary {
    @apply bg-blue-600 hover:bg-blue-700;
    @apply shadow-blue-500/30 hover:shadow-blue-500/40;
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
</style>
