<script>
  import { onMount, createEventDispatcher } from 'svelte';
  import { getHosts, addAndReloadHost, removeAndReloadHost } from '../stores/hosts';
  import { getSnippets } from '../stores/snippets';
  import { getPrivateKeys } from '../stores/privateKeys';
  import { invoke } from '@tauri-apps/api/core';

  const dispatch = createEventDispatcher();

  export let editingHost = null;

  let hosts = [];
  let snippets = [];
  let privateKeys = [];
  let selectedHost = null;
  let editMode = false;
  let keyTypeWarning = '';

  let form = {
    id: null,
    name: '',
    host: '',
    port: 22,
    username: '',
    privateKeyId: null,
    passphrase: '',
    snippetId: null,
  };

  onMount(async () => {
    hosts = await getHosts();
    snippets = await getSnippets();
    privateKeys = await getPrivateKeys();

    // If editing an existing host, populate the form
    if (editingHost) {
      // Find the corresponding private key ID from the path
      const key = privateKeys.find(pk => pk.path === editingHost.privateKeyPath);
      form = {
        ...editingHost,
        privateKeyId: key ? key.id : null,
      };
      editMode = true;
      if (editingHost.privateKeyPath) {
        checkKeyType(editingHost.privateKeyPath);
      }
    }
  });

  async function checkKeyType(path) {
    if (!path) {
      keyTypeWarning = '';
      return;
    }
    try {
      const keyType = await invoke('get_private_key_type', { path });
      if (keyType === 'rsa') {
        keyTypeWarning = '⚠️ Warning: RSA keys are outdated. We recommend using ed25519.';
      } else {
        keyTypeWarning = '';
      }
    } catch (error) {
      console.error('Failed to check key type:', error);
      keyTypeWarning = '⚠️ Could not read key type. Ensure the path is correct.';
    }
  }

  async function handleSave() {
    if (!form.name || !form.host || !form.username) {
      alert('Please fill in Name, Host, and Username fields.');
      return;
    }

    // Find the private key path from the ID
    const selectedKey = privateKeys.find(pk => pk.id === form.privateKeyId);
    const privateKeyPath = selectedKey ? selectedKey.path : null;

    const host = {
      ...form,
      id: form.id || `host-${Date.now()}`,
      privateKeyPath: privateKeyPath,
    };
    delete host.privateKeyId; // Clean up the ID from the final host object

    hosts = await addAndReloadHost(host);

    if (editMode) {
      dispatch('close');
    } else {
      dispatch('connect', host);
    }

    resetForm();
  }

  async function handleDelete(id) {
    if (confirm('Are you sure you want to delete this host?')) {
      hosts = await removeAndReloadHost(id);
      if (selectedHost?.id === id) {
        resetForm();
      }
    }
  }

  function handleEdit(host) {
    const key = privateKeys.find(pk => pk.path === host.privateKeyPath);
    form = {
      ...host,
      privateKeyId: key ? key.id : null,
    };
    editMode = true;
    selectedHost = host;
    if (host.privateKeyPath) {
      checkKeyType(host.privateKeyPath);
    }
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
      privateKeyId: null,
      passphrase: '',
      snippetId: null,
    };
    editMode = false;
    selectedHost = null;
    keyTypeWarning = '';
  }
</script>

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
      <label for="privateKey">Private Key (optional)</label>
      <select id="privateKey" bind:value={form.privateKeyId} on:change={(e) => {
        const selectedKey = privateKeys.find(pk => pk.id === e.target.value);
        checkKeyType(selectedKey ? selectedKey.path : null);
      }}>
        <option value={null}>None</option>
        {#each privateKeys as pk (pk.id)}
          <option value={pk.id}>{pk.name}</option>
        {/each}
      </select>
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

    <div class="form-group">
      <label for="snippet">Snippet</label>
      <select id="snippet" bind:value={form.snippetId}>
        <option value={null}>None</option>
        {#each snippets as snippet (snippet.id)}
          <option value={snippet.id}>{snippet.name}</option>
        {/each}
      </select>
    </div>

    <div class="form-actions">
      <button type="submit" class="btn-primary">
        {editMode ? 'Update' : 'Save'}
      </button>
      {#if editMode}
        <button type="button" class="btn-secondary" on:click={() => dispatch('close')}>
          Cancel
        </button>
      {/if}
    </div>
  </form>
</div>

<style>
  .form-section {
    @apply flex flex-col;
  }

  .form-group {
    @apply mb-4;
  }

  .form-row {
    @apply flex gap-4;
  }

  label {
    @apply block text-sm font-medium mb-1 text-gray-300;
  }

  input, select {
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

  .warning-message {
    @apply mt-2 text-sm text-orange-600 dark:text-orange-400;
  }
</style>
