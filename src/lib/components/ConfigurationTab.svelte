
<script>
  import { settings } from '../stores/settings.js';

  async function handleAutoStartChange(e) {
    const newValue = e.target.checked;
    await settings.update(s => ({ ...s, autoStartLocalTerminal: newValue }));
  }
</script>

<div class="config-content">
  <div class="config-section">
    <h3 class="section-title">General</h3>

    <label class="config-item">
      <div class="config-item-info">
        <span class="config-item-label">Auto-start local terminal</span>
        <span class="config-item-description">Automatically open a local terminal on startup</span>
      </div>
      <input
        type="checkbox"
        class="toggle-checkbox"
        checked={$settings.autoStartLocalTerminal}
        on:change={handleAutoStartChange}
      />
    </label>
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
    @apply hover:border-blue-500 transition-colors cursor-pointer;
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

  .toggle-checkbox {
    @apply h-5 w-5 rounded;
    @apply bg-gray-700 border-2 border-gray-600;
    @apply checked:bg-blue-600 checked:border-blue-600;
    @apply focus:ring-2 focus:ring-blue-500 focus:ring-offset-0;
    @apply cursor-pointer transition-colors;
    appearance: none;
    position: relative;
  }

  .toggle-checkbox:checked::before {
    content: '';
    @apply absolute inset-0;
    background-image: url("data:image/svg+xml,%3csvg viewBox='0 0 16 16' fill='white' xmlns='http://www.w3.org/2000/svg'%3e%3cpath d='M12.207 4.793a1 1 0 010 1.414l-5 5a1 1 0 01-1.414 0l-2-2a1 1 0 011.414-1.414L6.5 9.086l4.293-4.293a1 1 0 011.414 0z'/%3e%3c/svg%3e");
    background-size: 100% 100%;
    background-position: center;
    background-repeat: no-repeat;
  }
</style>
