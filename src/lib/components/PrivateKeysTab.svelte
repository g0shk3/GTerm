<script>
  import { onMount, createEventDispatcher } from 'svelte';
  import { getHosts } from '../stores/hosts';
  import { getPrivateKeys, addAndReloadPrivateKey, removeAndReloadPrivateKey } from '../stores/privateKeys';
  import { open } from '@tauri-apps/plugin-dialog';
  import { invoke } from '@tauri-apps/api/core';

  const dispatch = createEventDispatcher();

  let hosts = [];
  let privateKeys = [];

  let privateKeyForm = {
    id: null,
    name: '',
    path: '',
  };

  onMount(async () => {
    hosts = await getHosts();
    privateKeys = await getPrivateKeys();
  });

  async function handleSelectKeyFile() {
    const selected = await open({
      multiple: false,
      directory: false,
      filters: [{
        name: 'SSH Keys',
        extensions: ['', 'pem', 'pub', 'key']
      }]
    });

    if (selected) {
      privateKeyForm.path = selected.path;
    }
  }

  async function handleSavePrivateKey() {
    if (!privateKeyForm.name || !privateKeyForm.path) {
      alert('Please fill in both private key name and path');
      return;
    }

    const privateKey = {
      ...privateKeyForm,
      id: privateKeyForm.id || `privateKey-${Date.now()}`,
    };

    privateKeys = await addAndReloadPrivateKey(privateKey);
    resetPrivateKeyForm();
  }

  async function handleDeletePrivateKey(id) {
    if (confirm('Are you sure you want to delete this private key?')) {
      // Check if this key is being used by any host
      const isKeyInUse = hosts.some(h => {
        const key = privateKeys.find(pk => pk.id === id);
        return h.privateKeyPath === key?.path;
      });

      if (isKeyInUse) {
        alert('This key is currently in use by at least one host and cannot be deleted.');
        return;
      }

      privateKeys = await removeAndReloadPrivateKey(id);
    }
  }

  function handleEditPrivateKey(privateKey) {
    privateKeyForm = { ...privateKey };
  }

  function resetPrivateKeyForm() {
    privateKeyForm = {
      id: null,
      name: '',
      path: '',
    };
  }
</script>

<div class="form-section">
  <h3 class="text-lg font-semibold mb-3">
    {privateKeyForm.id ? 'Edit Private Key' : 'Add New Private Key'}
  </h3>

  <form on:submit|preventDefault={handleSavePrivateKey}>
    <div class="form-group">
      <label for="privateKey-name">Name *</label>
      <input
        id="privateKey-name"
        type="text"
        bind:value={privateKeyForm.name}
        placeholder="e.g., My Personal Key"
        required
      />
    </div>

    <div class="form-group">
      <label for="privateKey-path">Key Path *</label>
      <div class="flex gap-2">
        <input
          id="privateKey-path"
          type="text"
          bind:value={privateKeyForm.path}
          placeholder="~/.ssh/id_ed25519"
          class="flex-1"
          required
        />
        <button type="button" class="btn-sm btn-secondary" on:click={handleSelectKeyFile}>
          Browse
        </button>
      </div>
    </div>

    <div class="form-actions">
      <button type="submit" class="btn-primary">
        {privateKeyForm.id ? 'Update' : 'Create'}
      </button>
      {#if privateKeyForm.id}
        <button type="button" class="btn-secondary" on:click={resetPrivateKeyForm}>
          Cancel
        </button>
      {/if}
    </div>
  </form>

  <!-- Private Keys List -->
  <div class="list-section">
    <h3 class="text-lg font-semibold mb-3 mt-6">Your Saved Keys</h3>
    {#if privateKeys.length === 0}
      <p class="text-gray-500 dark:text-gray-400 text-sm">No private keys saved yet</p>
    {:else}
      <div class="list">
        {#each privateKeys as pk (pk.id)}
          <div class="list-item">
            <div class="item-info">
              <div class="item-name">{pk.name}</div>
              <div class="item-preview">{pk.path}</div>
            </div>
            <div class="item-actions">
              <button
                type="button"
                class="action-btn edit-btn"
                on:click={() => handleEditPrivateKey(pk)}
                title="Edit"
              >
                ✎
              </button>
              <button
                type="button"
                class="action-btn delete-btn"
                on:click={() => handleDeletePrivateKey(pk.id)}
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

  input {
    @apply w-full px-3 py-2 border border-gray-600 rounded-md bg-gray-900 text-gray-300;
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

  .btn-sm {
    @apply px-2 py-1 text-sm rounded-md transition-colors font-medium;
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
