<script>
  import { onMount, createEventDispatcher } from 'svelte';
  import { getSnippets, addAndReloadSnippet, removeAndReloadSnippet } from '../stores/snippets';

  const dispatch = createEventDispatcher();

  let snippets = [];

  let snippetForm = {
    id: null,
    name: '',
    content: '',
  };

  onMount(async () => {
    snippets = await getSnippets();
  });

  async function handleSaveSnippet() {
    if (!snippetForm.name || !snippetForm.content) {
      alert('Please fill in both snippet name and content');
      return;
    }

    const snippet = {
      ...snippetForm,
      id: snippetForm.id || `snippet-${Date.now()}`,
    };

    snippets = await addAndReloadSnippet(snippet);
    resetSnippetForm();
  }

  async function handleDeleteSnippet(id) {
    if (confirm('Are you sure you want to delete this snippet?')) {
      snippets = await removeAndReloadSnippet(id);
    }
  }

  function handleEditSnippet(snippet) {
    snippetForm = { ...snippet };
  }

  function resetSnippetForm() {
    snippetForm = {
      id: null,
      name: '',
      content: '',
    };
  }
</script>

<div class="form-section">
  <h3 class="text-lg font-semibold mb-3">
    {snippetForm.id ? 'Edit Snippet' : 'Create New Snippet'}
  </h3>

  <form on:submit|preventDefault={handleSaveSnippet}>
    <div class="form-group">
      <label for="snippet-name">Name *</label>
      <input
        id="snippet-name"
        type="text"
        bind:value={snippetForm.name}
        placeholder="e.g., ansible, docker, dev-env"
        required
        autocomplete="off"
        spellcheck="false"
      />
    </div>

    <div class="form-group">
      <label for="snippet-content">Command/Script *</label>
      <textarea
        id="snippet-content"
        bind:value={snippetForm.content}
        placeholder="e.g., sudo su ansible&#10;or any command you want to auto-run"
        rows="6"
        required
        autocapitalize="off"
        spellcheck="false"
      ></textarea>
    </div>

    <div class="form-actions">
      <button type="submit" class="btn-primary">
        {snippetForm.id ? 'Update' : 'Create'}
      </button>
      {#if snippetForm.id}
        <button type="button" class="btn-secondary" on:click={resetSnippetForm}>
          Cancel
        </button>
      {/if}
    </div>
  </form>

  <!-- Snippets List -->
  <div class="list-section">
    <h3 class="text-lg font-semibold mb-3 mt-6">Your Snippets</h3>
    {#if snippets.length === 0}
      <p class="text-gray-500 dark:text-gray-400 text-sm">No snippets created yet</p>
    {:else}
      <div class="list">
        {#each snippets as snippet (snippet.id)}
          <div class="list-item">
            <div class="item-info">
              <div class="item-name">{snippet.name}</div>
              <div class="item-preview">{snippet.content}</div>
            </div>
            <div class="item-actions">
              <button
                type="button"
                class="action-btn edit-btn"
                on:click={() => handleEditSnippet(snippet)}
                title="Edit"
              >
                ✎
              </button>
              <button
                type="button"
                class="action-btn delete-btn"
                on:click={() => handleDeleteSnippet(snippet.id)}
                title="Delete"
              >
                🗑
              </button>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>

<style>
  .form-section {
    @apply flex flex-col;
  }

  .form-group {
    @apply mb-4;
  }

  label {
    @apply block text-sm font-medium mb-1 text-gray-300;
  }

  input, textarea {
    @apply w-full px-3 py-2 border border-gray-600 rounded-md bg-gray-900 text-gray-300;
  }

  textarea {
    @apply font-mono text-sm;
  }

  .form-actions {
    @apply flex gap-3 mt-6 pt-4 border-t border-gray-700;
  }

  .btn-primary {
    @apply px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors font-medium;
  }

  .btn-secondary {
    @apply px-4 py-2 bg-gray-700 text-gray-100 rounded-md hover:bg-gray-600 transition-colors font-medium;
  }

  /* List Section */
  .list-section {
    @apply mt-6;
  }

  .list {
    @apply space-y-3;
  }

  .list-item {
    @apply flex items-start justify-between gap-4 p-3 rounded-lg border border-gray-700;
    @apply bg-gray-800 hover:bg-gray-700 transition-colors;
  }

  .item-info {
    @apply flex-1 min-w-0;
  }

  .item-name {
    @apply text-sm font-semibold text-white truncate;
  }

  .item-preview {
    @apply text-xs text-gray-400 mt-1 truncate;
  }

  .item-actions {
    @apply flex gap-2 flex-shrink-0;
  }

  .action-btn {
    @apply w-8 h-8 flex items-center justify-center rounded-md transition-colors;
    @apply text-base leading-none;
  }

  .edit-btn {
    @apply bg-gray-700 text-gray-400;
    @apply hover:bg-blue-600 hover:text-white;
  }

  .delete-btn {
    @apply bg-gray-700 text-gray-400;
    @apply hover:bg-red-600 hover:text-white;
  }
</style>
