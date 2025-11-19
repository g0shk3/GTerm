<script>
  import { onMount, createEventDispatcher } from 'svelte';
  import { getHosts, addAndReloadHost, removeAndReloadHost } from '../stores/hosts';
  import { getSnippets, addAndReloadSnippet, removeAndReloadSnippet } from '../stores/snippets';
  import { getPrivateKeys, addAndReloadPrivateKey, removeAndReloadPrivateKey } from '../stores/privateKeys';
  import { open } from '@tauri-apps/plugin-dialog';
  import { invoke } from '@tauri-apps/api/core';

  const dispatch = createEventDispatcher();

  export let editingHost = null;

  let hosts = [];
  let snippets = [];
  let privateKeys = [];
  let selectedHost = null;
  let editMode = false;
  let keyTypeWarning = '';
  let activeTab = 'hosts'; // 'hosts', 'snippets', or 'privateKeys'

  let form = {
    id: null,
    name: '',
    host: '',
    port: 22,
    username: '',
    privateKeyId: null, // Changed from privateKeyPath
    passphrase: '',
    snippetId: null,
  };

  let snippetForm = {
    id: null,
    name: '',
    content: '',
  };

  let privateKeyForm = {
    id: null,
    name: '',
    path: '',
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

    // Handle Escape key to close modal
    const handleKeydown = (e) => {
      if (e.key === 'Escape') {
        handleClose();
      }
    };
    document.addEventListener('keydown', handleKeydown);

    return () => {
      document.removeEventListener('keydown', handleKeydown);
    };
  });

  async function handleSelectKeyFile(formObject = privateKeyForm) {
    const selected = await open({
      multiple: false,
      directory: false,
      filters: [{
        name: 'SSH Keys',
        extensions: ['', 'pem', 'pub', 'key']
      }]
    });

    if (selected) {
        formObject.path = selected.path;
    }
  }

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

  async function handleGenerateKey(formObject = privateKeyForm) {
    try {
      const homeDir = await invoke('get_home_dir');
      const keyPath = `${homeDir}/.ssh/gterm_ed25519_${Date.now()}`;

      const publicKey = await invoke('generate_keypair', { outputPath: keyPath });

      formObject.path = keyPath;
      
      await navigator.clipboard.writeText(publicKey);
      alert('New ed25519 key generated successfully!\nPublic key has been copied to clipboard.');
    } catch (error) {
      alert('Error generating key: ' + error);
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
      privateKeyPath: privateKeyPath, // Ensure path is included
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
      if (form.snippetId === id) {
        form.snippetId = null;
      }
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
      if (form.privateKeyId === id) {
        form.privateKeyId = null;
      }
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

  function handleClose() {
    dispatch('close');
    setTimeout(() => {
      window.dispatchEvent(new CustomEvent('tabSwitched'));
    }, 50);
  }
</script>

<div class="modal-overlay" on:click={handleClose} role="presentation">
  <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
  <div class="modal-content" on:click|stopPropagation on:keydown|stopPropagation role="dialog" tabindex="-1">
    <div class="modal-header">
      <div class="header-tabs">
        <button
          class="header-tab"
          class:active={activeTab === 'hosts'}
          on:click={() => activeTab = 'hosts'}
        >
          SSH Connections
        </button>
        <button
          class="header-tab"
          class:active={activeTab === 'privateKeys'}
          on:click={() => activeTab = 'privateKeys'}
        >
          Private Keys
        </button>
        <button
          class="header-tab"
          class:active={activeTab === 'snippets'}
          on:click={() => activeTab = 'snippets'}
        >
          Snippets
        </button>
      </div>
      <button class="close-btn" on:click={handleClose}>×</button>
    </div>

    <div class="modal-body">
      <!-- HOSTS TAB -->
      {#if activeTab === 'hosts'}
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
              <button type="button" class="btn-secondary" on:click={handleClose}>
                Cancel
              </button>
            {/if}
          </div>
        </form>
      </div>
      {/if}

      <!-- PRIVATE KEYS TAB -->
      {#if activeTab === 'privateKeys'}
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
              <button type="button" class="btn-sm btn-secondary" on:click={() => handleSelectKeyFile(privateKeyForm)}>
                Browse
              </button>
              <button type="button" class="btn-sm btn-secondary" on:click={() => handleGenerateKey(privateKeyForm)}>
                Generate
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
        <div class="snippets-list-section">
          <h3 class="text-lg font-semibold mb-3 mt-6">Your Saved Keys</h3>
          {#if privateKeys.length === 0}
            <p class="text-gray-500 dark:text-gray-400 text-sm">No private keys saved yet</p>
          {:else}
            <div class="snippets-list">
              {#each privateKeys as pk (pk.id)}
                <div class="snippet-item">
                  <div class="snippet-info">
                    <div class="snippet-name">{pk.name}</div>
                    <div class="snippet-preview">{pk.path}</div>
                  </div>
                  <div class="snippet-actions">
                    <button
                      type="button"
                      class="snippet-btn snippet-edit-btn"
                      on:click={() => handleEditPrivateKey(pk)}
                      title="Edit"
                    >
                      ✎
                    </button>
                    <button
                      type="button"
                      class="snippet-btn snippet-delete-btn"
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
      {/if}

      <!-- SNIPPETS TAB -->
      {#if activeTab === 'snippets'}
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
            />
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
        <div class="snippets-list-section">
          <h3 class="text-lg font-semibold mb-3 mt-6">Your Snippets</h3>
          {#if snippets.length === 0}
            <p class="text-gray-500 dark:text-gray-400 text-sm">No snippets created yet</p>
          {:else}
            <div class="snippets-list">
              {#each snippets as snippet (snippet.id)}
                <div class="snippet-item">
                  <div class="snippet-info">
                    <div class="snippet-name">{snippet.name}</div>
                    <div class="snippet-preview">{snippet.content}</div>
                  </div>
                  <div class="snippet-actions">
                    <button
                      type="button"
                      class="snippet-btn snippet-edit-btn"
                      on:click={() => handleEditSnippet(snippet)}
                      title="Edit"
                    >
                      ✎
                    </button>
                    <button
                      type="button"
                      class="snippet-btn snippet-delete-btn"
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
      {/if}
    </div>
  </div>
</div>

<style>
  .modal-overlay {
    @apply fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50;
  }

  .modal-content {
    @apply bg-white dark:bg-gray-800 rounded-xl shadow-2xl max-w-lg w-full max-h-[85vh] overflow-hidden flex flex-col;
    margin: 20px;
  }

  .modal-header {
    @apply flex items-center justify-between px-6 py-4 border-b border-gray-200 dark:border-gray-700 gap-4;
  }

  .close-btn {
    @apply text-2xl leading-none text-gray-600 dark:text-gray-400 hover:text-red-500 transition-colors font-light;
  }

  .modal-body {
    @apply p-6 overflow-y-auto flex-1;
  }

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
    @apply block text-sm font-medium mb-1 text-gray-700 dark:text-gray-300;
  }

  input, select {
    @apply w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100;
  }

  .form-actions {
    @apply flex gap-3 mt-6 pt-4 border-t border-gray-200 dark:border-gray-700;
  }

  .btn-primary {
    @apply px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors font-medium;
  }

  .btn-secondary {
    @apply px-4 py-2 bg-gray-300 dark:bg-gray-600 text-gray-900 dark:text-gray-100 rounded-md hover:bg-gray-400 dark:hover:bg-gray-500 transition-colors font-medium;
  }

  .btn-sm {
    @apply px-2 py-1 text-sm rounded-md transition-colors font-medium;
  }

  .warning-message {
    @apply mt-2 text-sm text-orange-600 dark:text-orange-400;
  }

  /* Header Tabs */
  .header-tabs {
    @apply flex gap-4;
  }

  .header-tab {
    @apply px-3 py-2 font-medium text-gray-700 dark:text-gray-300;
    @apply border-b-2 border-transparent;
    @apply hover:text-gray-900 dark:hover:text-white;
    @apply transition-colors cursor-pointer;
  }

  .header-tab.active {
    @apply text-blue-600 dark:text-blue-400;
    @apply border-blue-600 dark:border-blue-400;
  }


  /* Textarea */
  textarea {
    @apply w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 font-mono text-sm;
  }

  /* Snippets List Section */
  .snippets-list-section {
    @apply mt-6;
  }

  .snippets-list {
    @apply space-y-3;
  }

  .snippet-item {
    @apply flex items-start justify-between gap-4 p-3 rounded-lg border border-gray-200 dark:border-gray-700;
    @apply bg-gray-50 dark:bg-gray-900/50 hover:bg-gray-100 dark:hover:bg-gray-900 transition-colors;
  }

  .snippet-info {
    @apply flex-1 min-w-0;
  }

  .snippet-name {
    @apply text-sm font-semibold text-gray-900 dark:text-white truncate;
  }

  .snippet-preview {
    @apply text-xs text-gray-600 dark:text-gray-400 mt-1 truncate;
  }

  .snippet-actions {
    @apply flex gap-2 flex-shrink-0;
  }

  .snippet-btn {
    @apply w-8 h-8 flex items-center justify-center rounded-md transition-colors;
    @apply text-base leading-none;
  }

  .snippet-edit-btn {
    @apply bg-blue-100 dark:bg-blue-900 text-blue-600 dark:text-blue-300;
    @apply hover:bg-blue-200 dark:hover:bg-blue-800;
  }

  .snippet-delete-btn {
    @apply bg-red-100 dark:bg-red-900 text-red-600 dark:text-red-300;
    @apply hover:bg-red-200 dark:hover:bg-red-800;
  }
</style>
