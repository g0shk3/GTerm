<script>
  import { onMount, createEventDispatcher } from 'svelte';
  import { getHosts, saveHost, deleteHost } from '../stores/hosts';
  import { open } from '@tauri-apps/plugin-dialog';
  import { invoke } from '@tauri-apps/api/core';

  const dispatch = createEventDispatcher();

  let hosts = [];
  let selectedHost = null;
  let editMode = false;
  let keyTypeWarning = '';

  let form = {
    id: null,
    name: '',
    host: '',
    port: 22,
    username: '',
    privateKeyPath: '',
    passphrase: '',
  };

  onMount(async () => {
    hosts = await getHosts();
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
      form.privateKeyPath = selected.path;
      await checkKeyType(selected.path);
    }
  }

  async function checkKeyType(path) {
    try {
      const keyType = await invoke('get_private_key_type', { path });
      if (keyType === 'rsa') {
        keyTypeWarning = '⚠️ RSA ключът е остарял. Препоръчваме ed25519.';
      } else {
        keyTypeWarning = '';
      }
    } catch (error) {
      console.error('Failed to check key type:', error);
      keyTypeWarning = '';
    }
  }

  async function handleGenerateKey() {
    try {
      const homeDir = await invoke('get_home_dir');
      const keyPath = `${homeDir}/.ssh/gterm_ed25519`;

      const publicKey = await invoke('generate_keypair', { outputPath: keyPath });

      form.privateKeyPath = keyPath;
      keyTypeWarning = '';

      // Copy public key to clipboard
      await navigator.clipboard.writeText(publicKey);
      alert('Нов ed25519 ключ е генериран!\nПубличният ключ е копиран в clipboard.');
    } catch (error) {
      alert('Грешка при генериране на ключ: ' + error);
    }
  }

  async function handleSave() {
    if (!form.name || !form.host || !form.username || !form.privateKeyPath) {
      alert('Моля, попълнете всички задължителни полета');
      return;
    }

    const host = {
      ...form,
      id: form.id || `host-${Date.now()}`,
    };

    hosts = await saveHost(host);
    resetForm();
  }

  async function handleDelete(id) {
    if (confirm('Сигурни ли сте, че искате да изтриете този хост?')) {
      hosts = await deleteHost(id);
      if (selectedHost?.id === id) {
        resetForm();
      }
    }
  }

  function handleEdit(host) {
    form = { ...host };
    editMode = true;
    selectedHost = host;
    checkKeyType(host.privateKeyPath);
  }

  function handleConnect(host) {
    dispatch('connect', host);
  }

  function resetForm() {
    form = {
      id: null,
      name: '',
      host: '',
      port: 22,
      username: '',
      privateKeyPath: '',
      passphrase: '',
    };
    editMode = false;
    selectedHost = null;
    keyTypeWarning = '';
  }

  function handleClose() {
    dispatch('close');
  }
</script>

<div class="modal-overlay" on:click={handleClose} on:keydown={(e) => e.key === 'Escape' && handleClose()} role="dialog" tabindex="-1">
  <div class="modal-content" on:click|stopPropagation on:keydown|stopPropagation role="document">
    <div class="modal-header">
      <h2 class="text-xl font-bold">SSH Connections</h2>
      <button class="close-btn" on:click={handleClose}>×</button>
    </div>

    <div class="modal-body">
      <!-- Saved Hosts List -->
      <div class="hosts-section">
        <h3 class="text-lg font-semibold mb-3">Saved Hosts</h3>
        {#if hosts.length === 0}
          <p class="text-gray-500 dark:text-gray-400 text-sm">No saved hosts yet</p>
        {:else}
          <div class="hosts-list">
            {#each hosts as host (host.id)}
              <div class="host-item">
                <div class="host-info">
                  <div class="font-medium">{host.name}</div>
                  <div class="text-sm text-gray-600 dark:text-gray-400">
                    {host.username}@{host.host}:{host.port}
                  </div>
                </div>
                <div class="host-actions">
                  <button class="btn-sm btn-primary" on:click={() => handleConnect(host)}>
                    Connect
                  </button>
                  <button class="btn-sm btn-secondary" on:click={() => handleEdit(host)}>
                    Edit
                  </button>
                  <button class="btn-sm btn-danger" on:click={() => handleDelete(host.id)}>
                    Delete
                  </button>
                </div>
              </div>
            {/each}
          </div>
        {/if}
      </div>

      <!-- Add/Edit Form -->
      <div class="form-section">
        <h3 class="text-lg font-semibold mb-3">
          {editMode ? 'Edit Host' : 'Add New Host'}
        </h3>

        <form on:submit|preventDefault={handleSave}>
          <div class="form-group">
            <label for="name">Name *</label>
            <input
              id="name"
              type="text"
              bind:value={form.name}
              placeholder="My Server"
              required
            />
          </div>

          <div class="form-row">
            <div class="form-group flex-1">
              <label for="host">Host *</label>
              <input
                id="host"
                type="text"
                bind:value={form.host}
                placeholder="192.168.1.100"
                required
              />
            </div>

            <div class="form-group" style="width: 100px;">
              <label for="port">Port *</label>
              <input
                id="port"
                type="number"
                bind:value={form.port}
                min="1"
                max="65535"
                required
              />
            </div>
          </div>

          <div class="form-group">
            <label for="username">Username *</label>
            <input
              id="username"
              type="text"
              bind:value={form.username}
              placeholder="root"
              required
            />
          </div>

          <div class="form-group">
            <label for="privateKey">Private Key *</label>
            <div class="flex gap-2">
              <input
                id="privateKey"
                type="text"
                bind:value={form.privateKeyPath}
                placeholder="~/.ssh/id_ed25519"
                class="flex-1"
                required
              />
              <button type="button" class="btn-sm btn-secondary" on:click={handleSelectKeyFile}>
                Browse
              </button>
              <button type="button" class="btn-sm btn-secondary" on:click={handleGenerateKey}>
                Generate
              </button>
            </div>
            {#if keyTypeWarning}
              <div class="warning-message">{keyTypeWarning}</div>
            {/if}
          </div>

          <div class="form-group">
            <label for="passphrase">Passphrase (optional)</label>
            <input
              id="passphrase"
              type="password"
              bind:value={form.passphrase}
              placeholder="Leave empty if no passphrase"
            />
          </div>

          <div class="form-actions">
            <button type="submit" class="btn-primary">
              {editMode ? 'Update' : 'Save'}
            </button>
            {#if editMode}
              <button type="button" class="btn-secondary" on:click={resetForm}>
                Cancel
              </button>
            {/if}
          </div>
        </form>
      </div>
    </div>
  </div>
</div>

<style>
  .modal-overlay {
    @apply fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50;
  }

  .modal-content {
    @apply bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-4xl w-full max-h-[90vh] overflow-hidden flex flex-col;
    margin: 20px;
  }

  .modal-header {
    @apply flex items-center justify-between p-4 border-b border-gray-300 dark:border-gray-700;
  }

  .close-btn {
    @apply text-3xl leading-none hover:text-red-500 transition-colors;
  }

  .modal-body {
    @apply p-4 overflow-y-auto flex-1 grid grid-cols-2 gap-6;
  }

  .hosts-section,
  .form-section {
    @apply flex flex-col;
  }

  .hosts-list {
    @apply space-y-2;
  }

  .host-item {
    @apply p-3 bg-gray-100 dark:bg-gray-700 rounded-md flex items-center justify-between;
  }

  .host-info {
    @apply flex-1;
  }

  .host-actions {
    @apply flex gap-2;
  }

  .form-group {
    @apply mb-4;
  }

  .form-row {
    @apply flex gap-4;
  }

  label {
    @apply block text-sm font-medium mb-1;
  }

  input {
    @apply w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100;
  }

  .form-actions {
    @apply flex gap-2 mt-6;
  }

  .btn-primary {
    @apply px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors font-medium;
  }

  .btn-secondary {
    @apply px-4 py-2 bg-gray-300 dark:bg-gray-600 text-gray-900 dark:text-gray-100 rounded-md hover:bg-gray-400 dark:hover:bg-gray-500 transition-colors font-medium;
  }

  .btn-danger {
    @apply px-4 py-2 bg-red-600 text-white rounded-md hover:bg-red-700 transition-colors font-medium;
  }

  .btn-sm {
    @apply px-2 py-1 text-sm rounded-md transition-colors font-medium;
  }

  .warning-message {
    @apply mt-2 text-sm text-orange-600 dark:text-orange-400;
  }
</style>
