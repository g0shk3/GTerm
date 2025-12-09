
<script>
  import { settings } from '../stores/settings.js';
  import Switch from './Switch.svelte';

  async function handleSettingChange(key, value) {
    await settings.update(s => ({ ...s, [key]: value }));
  }
</script>

<div class="config-content">
  <div class="config-section">
    <h3 class="section-title">General</h3>

    <div class="config-item">
      <div class="config-item-info">
        <span class="config-item-label">Auto-start local terminal</span>
        <span class="config-item-description">Automatically open a local terminal on startup</span>
      </div>
      <Switch
        checked={$settings.autoStartLocalTerminal}
        on:change={e => handleSettingChange('autoStartLocalTerminal', e.target.checked)}
      />
    </div>

    <div class="config-item">
      <div class="config-item-info">
        <span class="config-item-label">Copy on select</span>
        <span class="config-item-description">Automatically copy selected text to clipboard</span>
      </div>
      <Switch
        checked={$settings.autoCopyOnSelect}
        on:change={e => handleSettingChange('autoCopyOnSelect', e.target.checked)}
      />
    </div>

    <div class="config-item">
      <div class="config-item-info">
        <span class="config-item-label">Remember window size and position</span>
        <span class="config-item-description">Save and restore the window dimensions between sessions</span>
      </div>
      <Switch
        checked={$settings.rememberWindowSize}
        on:change={e => handleSettingChange('rememberWindowSize', e.target.checked)}
      />
    </div>
  </div>

  <div class="config-section">
    <h3 class="section-title">Terminal</h3>
    <div class="config-item">
      <div class="config-item-info">
        <span class="config-item-label">Scrollback lines</span>
        <span class="config-item-description">Number of lines to keep in terminal history</span>
      </div>
       <input
        type="number"
        class="input-field"
        min="1"
        step="100"
        bind:value={$settings.scrollback}
        on:change={e => handleSettingChange('scrollback', parseInt(e.target.value, 10))}
      />
    </div>
    <div class="config-item">
      <div class="config-item-info">
        <span class="config-item-label">Open new tabs next to active</span>
        <span class="config-item-description">Open new tabs immediately after the current active tab</span>
      </div>
      <Switch
        checked={$settings.openTabsNextToActive}
        on:change={e => handleSettingChange('openTabsNextToActive', e.target.checked)}
      />
    </div>
    <div class="config-item">
      <div class="config-item-info">
        <span class="config-item-label">Search direction</span>
        <span class="config-item-description">Direction to search when pressing Enter in search box</span>
      </div>
      <select
        class="input-field"
        bind:value={$settings.searchDirection}
        on:change={e => handleSettingChange('searchDirection', e.target.value)}
      >
        <option value="bottomToTop">Bottom to Top (↑)</option>
        <option value="topToBottom">Top to Bottom (↓)</option>
      </select>
    </div>
  </div>
</div>

<style>
  .config-content {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .config-section {
    @apply space-y-4;
  }

  .section-title {
    @apply text-sm font-semibold text-gray-400 uppercase tracking-wider mb-3;
  }

  .config-item {
    @apply flex items-center justify-between p-4 rounded-lg;
    @apply bg-gray-900 border border-gray-700;
    @apply hover:border-blue-500 transition-colors;
    @apply w-full;
  }

  .config-item-info {
    @apply flex flex-col gap-1;
  }

  .config-item-label {
    @apply text-sm font-medium text-gray-100;
  }

  .config-item-description {
    @apply text-xs text-gray-400;
  }

  .input-field {
      @apply w-28 bg-gray-800 border border-gray-600 rounded-md text-white text-sm p-2;
      @apply focus:ring-2 focus:ring-blue-500 focus:border-blue-500;
  }
</style>
