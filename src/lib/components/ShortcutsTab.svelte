
<script>
  import { onMount } from 'svelte';
  import { shortcuts, saveShortcuts, resetShortcuts, getShortcutDisplay } from '../stores/shortcuts';

  let currentShortcuts = [];
  let editingShortcut = null;
  let recordingKey = false;
  let tempKey = '';
  let tempMetaKey = false;
  let tempShiftKey = false;
  let tempAltKey = false;

  $: currentShortcuts = $shortcuts;

  // Group shortcuts by category
  $: groupedShortcuts = currentShortcuts.reduce((acc, shortcut) => {
    if (!acc[shortcut.category]) {
      acc[shortcut.category] = [];
    }
    acc[shortcut.category].push(shortcut);
    return acc;
  }, {});

  function startEditing(shortcut) {
    editingShortcut = shortcut.id;
    tempKey = shortcut.key;
    tempMetaKey = shortcut.metaKey;
    tempShiftKey = shortcut.shiftKey;
    tempAltKey = shortcut.altKey;
  }

  function cancelEditing() {
    editingShortcut = null;
    recordingKey = false;
    tempKey = '';
    tempMetaKey = false;
    tempShiftKey = false;
    tempAltKey = false;
  }

  function startRecording(e) {
    e.preventDefault();
    recordingKey = true;
    tempKey = '';
    tempMetaKey = false;
    tempShiftKey = false;
    tempAltKey = false;
  }

  function handleKeyDown(e) {
    if (!recordingKey) return;

    e.preventDefault();
    e.stopPropagation();

    // Ignore modifier keys alone
    if (['Meta', 'Shift', 'Alt', 'Control'].includes(e.key)) {
      return;
    }

    tempKey = e.key;
    tempMetaKey = e.metaKey;
    tempShiftKey = e.shiftKey;
    tempAltKey = e.altKey;
    recordingKey = false;
  }

  async function saveEditing() {
    if (!editingShortcut || !tempKey) {
      cancelEditing();
      return;
    }

    const updated = currentShortcuts.map(s => {
      if (s.id === editingShortcut) {
        return {
          ...s,
          key: tempKey,
          metaKey: tempMetaKey,
          shiftKey: tempShiftKey,
          altKey: tempAltKey
        };
      }
      return s;
    });

    await saveShortcuts(updated);
    cancelEditing();
  }

  async function handleReset() {
    if (confirm('Are you sure you want to reset all shortcuts to defaults?')) {
      await resetShortcuts();
    }
  }

  onMount(() => {
    const handleKeyPress = (e) => {
      if (recordingKey) {
        handleKeyDown(e);
      }
    };

    // We need to listen on the window to capture keys even if the button is not focused
    window.addEventListener('keydown', handleKeyPress, { capture: true });

    return () => {
      window.removeEventListener('keydown', handleKeyPress, { capture: true });
    };
  });
</script>

<div class="shortcuts-content">
  <div class="flex justify-end mb-4">
    <button on:click={handleReset} class="px-3 py-1.5 rounded-md bg-red-800 text-white hover:bg-red-700 transition-colors text-sm">Reset to Defaults</button>
  </div>
  {#if Object.keys(groupedShortcuts).length === 0}
    <div class="empty-state">
      <p>No shortcuts found</p>
    </div>
  {:else}
    {#each Object.entries(groupedShortcuts) as [category, categoryShortcuts]}
      <div class="category-section">
        <h3 class="category-title">{category.charAt(0).toUpperCase() + category.slice(1)}</h3>
        <div class="shortcuts-list">
          {#each categoryShortcuts as shortcut}
            <div class="shortcut-item">
              <div class="shortcut-info">
                <div class="shortcut-name">{shortcut.name}</div>
                <div class="shortcut-description">{shortcut.description}</div>
              </div>

              {#if editingShortcut === shortcut.id}
                <div class="shortcut-editor">
                  <button
                    class="record-btn"
                    class:recording={recordingKey}
                    on:click={startRecording}
                  >
                    {#if recordingKey}
                      Press a key...
                    {:else if tempKey}
                      <span class="font-mono">{tempMetaKey ? '⌘' : ''}{tempShiftKey ? '⇧' : ''}{tempAltKey ? '⌥' : ''}{tempKey.toUpperCase()}</span>
                    {:else}
                      Click to record
                    {/if}
                  </button>
                  <div class="editor-actions">
                    <button class="save-btn" on:click={saveEditing} disabled={!tempKey} aria-label="Save shortcut">
                      <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M2 7l3 3 7-7"/>
                      </svg>
                    </button>
                    <button class="cancel-btn" on:click={cancelEditing} aria-label="Cancel editing">
                      <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M2 2l10 10M12 2L2 12"/>
                      </svg>
                    </button>
                  </div>
                </div>
              {:else}
                <div class="shortcut-actions">
                  <div class="shortcut-keys">
                    {getShortcutDisplay(shortcut)}
                  </div>
                  <button class="edit-btn" on:click={() => startEditing(shortcut)} aria-label="Edit shortcut">
                    <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="1.5">
                      <path d="M9.5 2L12 4.5L5 11.5L2 12L2.5 9L9.5 2Z"/>
                    </svg>
                  </button>
                </div>
              {/if}
            </div>
          {/each}
        </div>
      </div>
    {/each}
  {/if}
</div>

<style>
  .shortcuts-content {
    @apply flex-1;
  }

  .empty-state {
    @apply text-center py-12 text-gray-400;
  }

  .category-section {
    @apply mb-6 last:mb-0;
  }

  .category-title {
    @apply text-sm font-semibold text-gray-400 uppercase tracking-wider mb-3;
  }

  .shortcuts-list {
    @apply flex flex-col gap-2;
  }

  .shortcut-item {
    @apply flex items-center justify-between px-4 py-3 rounded-lg;
    @apply bg-gray-900 border border-gray-700;
    @apply hover:border-blue-500 transition-colors;
  }

  .shortcut-info {
    @apply flex-1 min-w-0;
  }

  .shortcut-name {
    @apply text-sm font-medium text-gray-100;
  }

  .shortcut-description {
    @apply text-xs text-gray-400 mt-0.5;
  }

  .shortcut-actions {
    @apply flex items-center gap-3;
  }

  .shortcut-keys {
    @apply px-3 py-1.5 rounded-md bg-gray-700;
    @apply text-sm font-mono font-semibold text-gray-300;
  }

  .edit-btn {
    @apply p-2 rounded-md hover:bg-gray-600;
    @apply text-gray-400 transition-colors;
  }

  .shortcut-editor {
    @apply flex items-center gap-2;
  }

  .record-btn {
    @apply px-4 py-2 rounded-md border-2 border-dashed;
    @apply text-sm font-semibold min-w-[140px];
    @apply transition-all text-center;
  }

  .record-btn:not(.recording) {
    @apply border-gray-600 bg-gray-800;
    @apply text-gray-400 hover:border-blue-400 hover:bg-blue-950;
  }

  .record-btn.recording {
    @apply border-blue-500 bg-blue-950 text-blue-400;
    animation: pulse 1.5s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  }

  @keyframes pulse {
    0%, 100% {
      opacity: 1;
    }
    50% {
      opacity: 0.6;
    }
  }

  .editor-actions {
    @apply flex gap-1;
  }

  .save-btn,
  .cancel-btn {
    @apply p-2 rounded-md transition-colors;
  }

  .save-btn {
    @apply bg-green-700 hover:bg-green-600 text-white;
  }

  .save-btn:disabled {
    @apply bg-gray-700 cursor-not-allowed text-gray-400;
  }

  .cancel-btn {
    @apply bg-gray-700 hover:bg-gray-600 text-gray-300;
  }
</style>
