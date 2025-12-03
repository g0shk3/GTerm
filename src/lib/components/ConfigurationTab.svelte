
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
</style>
