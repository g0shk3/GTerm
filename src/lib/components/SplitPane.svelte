<script>
  import Terminal from './Terminal.svelte';
  import SFTP from './SFTP.svelte';
  import { setActivePane, closePane } from '../stores/tabs';

  export let tab;

  function handlePaneClick(paneId) {
    setActivePane(tab.id, paneId);
  }

  function handleClosePane(paneId) {
    if (tab.panes.length > 1) {
      closePane(tab.id, paneId);
    }
  }
</script>

<div class="split-container" class:vertical={tab.splitLayout === 'vertical'} class:horizontal={tab.splitLayout === 'horizontal'}>
  {#each tab.panes as pane (pane.id)}
    <div
      class="split-pane"
      class:active={pane.id === tab.activePaneId}
      on:click={() => handlePaneClick(pane.id)}
      on:keydown={(e) => e.key === 'Enter' && handlePaneClick(pane.id)}
      role="button"
      tabindex="0"
    >
      <!-- Close button (only show if more than 1 pane) -->
      {#if tab.panes.length > 1}
        <button
          class="pane-close-btn"
          on:click|stopPropagation={() => handleClosePane(pane.id)}
          title="Close pane"
        >
          <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M3 3l8 8M11 3l-8 8" />
          </svg>
        </button>
      {/if}

      <!-- Render terminal or SFTP based on pane type -->
      {#if pane.type === 'terminal'}
        <Terminal tab={{ ...tab, ...pane, id: pane.id }} />
      {:else if pane.type === 'sftp'}
        <SFTP tab={{ ...tab, ...pane, id: pane.id }} />
      {/if}
    </div>
  {/each}
</div>

<style>
  .split-container {
    @apply w-full h-full flex;
  }

  .split-container.vertical {
    @apply flex-row;
  }

  .split-container.horizontal {
    @apply flex-col;
  }

  .split-pane {
    @apply relative flex-1;
    @apply border-2 border-transparent;
    @apply transition-all duration-200;
    min-width: 200px;
    min-height: 100px;
  }

  .split-pane.active {
    @apply border-blue-500/30;
  }

  .split-container.vertical .split-pane:not(:last-child) {
    @apply border-r-2 border-gray-300 dark:border-gray-700;
  }

  .split-container.horizontal .split-pane:not(:last-child) {
    @apply border-b-2 border-gray-300 dark:border-gray-700;
  }

  .pane-close-btn {
    @apply absolute top-2 right-2 z-10;
    @apply p-1.5 rounded opacity-0 hover:bg-red-100 dark:hover:bg-red-900/30;
    @apply text-gray-400 hover:text-red-600 dark:hover:text-red-400;
    @apply transition-all duration-200;
    @apply bg-white/90 dark:bg-gray-900/90 backdrop-blur;
  }

  .split-pane:hover .pane-close-btn,
  .split-pane.active .pane-close-btn {
    opacity: 1;
  }

  .split-pane:focus {
    @apply outline-none;
  }
</style>
