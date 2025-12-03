<script>
  import ConfigurationTab from './ConfigurationTab.svelte';
  import ShortcutsTab from './ShortcutsTab.svelte';
  import ConnectionsTab from './ConnectionsTab.svelte';
  import PrivateKeysTab from './PrivateKeysTab.svelte';
  import SnippetsTab from './SnippetsTab.svelte';

  let activeTab = 'configuration';

  const tabs = [
    { id: 'configuration', label: 'Configuration' },
    { id: 'shortcuts', label: 'Shortcuts' },
    { id: 'connections', label: 'SSH Connections' },
    { id: 'privateKeys', label: 'Private Keys' },
    { id: 'snippets', label: 'Snippets' }
  ];
</script>

<div class="settings-page">
  <!-- Tabs -->
  <div class="tabs-container">
    {#each tabs as tab}
      <button
        class="tab-button"
        class:active={activeTab === tab.id}
        on:click={() => activeTab = tab.id}
      >
        {tab.label}
      </button>
    {/each}
  </div>

  <!-- Tab Content -->
  <div class="settings-content">
    {#if activeTab === 'configuration'}
      <ConfigurationTab />
    {:else if activeTab === 'shortcuts'}
      <ShortcutsTab />
    {:else if activeTab === 'connections'}
      <ConnectionsTab />
    {:else if activeTab === 'privateKeys'}
      <PrivateKeysTab />
    {:else if activeTab === 'snippets'}
      <SnippetsTab />
    {/if}
  </div>
</div>

<style>
  .settings-page {
    @apply w-full h-full flex flex-col;
    @apply bg-gray-800;
    overflow: hidden;
  }

  .tabs-container {
    @apply flex gap-2 px-6 pt-4;
    @apply border-b border-gray-700;
    flex-shrink: 0;
    overflow-x: auto;
    scrollbar-width: thin;
  }

  .tabs-container::-webkit-scrollbar {
    height: 4px;
  }

  .tabs-container::-webkit-scrollbar-track {
    @apply bg-transparent;
  }

  .tabs-container::-webkit-scrollbar-thumb {
    @apply bg-gray-600 rounded-full;
  }

  .tab-button {
    @apply px-6 py-3 text-sm font-medium;
    @apply text-gray-400 hover:text-gray-200;
    @apply border-b-2 border-transparent;
    @apply transition-all;
    @apply whitespace-nowrap;
    position: relative;
  }

  .tab-button.active {
    @apply text-blue-400 border-blue-400;
  }

  .settings-content {
    @apply flex-1 overflow-auto px-6 py-6;
  }

  /* Custom scrollbar */
  .settings-content::-webkit-scrollbar {
    width: 8px;
  }

  .settings-content::-webkit-scrollbar-track {
    @apply bg-transparent;
  }

  .settings-content::-webkit-scrollbar-thumb {
    @apply bg-gray-600 rounded-full;
  }

  .settings-content::-webkit-scrollbar-thumb:hover {
    @apply bg-gray-500;
  }
</style>
