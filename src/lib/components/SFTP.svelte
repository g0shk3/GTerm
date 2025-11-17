<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { open, save } from '@tauri-apps/plugin-dialog';

  export let pane;

  let currentPath = '/home';
  let files = [];
  let loading = false;
  let error = '';
  let selectedFile = null;

  onMount(async () => {
    await loadDirectory(currentPath);
  });

  async function loadDirectory(path) {
    loading = true;
    error = '';
    try {
      files = await invoke('sftp_list_directory', {
        sessionId: pane.sessionId,
        path: path,
      });
      currentPath = path;
    } catch (err) {
      error = `Failed to load directory: ${err}`;
      console.error(err);
    } finally {
      loading = false;
    }
  }

  async function handleFileClick(file) {
    if (file.is_dir) {
      await loadDirectory(file.path);
    } else {
      selectedFile = file;
    }
  }

  async function handleDownload(file) {
    try {
      const savePath = await save({
        defaultPath: file.name,
        filters: [{
          name: 'All Files',
          extensions: ['*']
        }]
      });

      if (savePath) {
        await invoke('sftp_download', {
          sessionId: pane.sessionId,
          remotePath: file.path,
          localPath: savePath,
        });
        alert('File downloaded successfully!');
      }
    } catch (err) {
      alert(`Failed to download: ${err}`);
    }
  }

  async function handleUpload() {
    try {
      const selected = await open({
        multiple: false,
        directory: false,
      });

      if (selected) {
        const fileName = selected.name || selected.path.split('/').pop();
        const remotePath = `${currentPath}/${fileName}`;

        await invoke('sftp_upload', {
          sessionId: pane.sessionId,
          localPath: selected.path,
          remotePath: remotePath,
        });

        alert('File uploaded successfully!');
        await loadDirectory(currentPath);
      }
    } catch (err) {
      alert(`Failed to upload: ${err}`);
    }
  }

  function goUp() {
    const parts = currentPath.split('/').filter(p => p);
    if (parts.length > 0) {
      parts.pop();
      const newPath = '/' + parts.join('/');
      loadDirectory(newPath || '/');
    }
  }

  function formatSize(bytes) {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i];
  }

  function formatDate(timestamp) {
    const date = new Date(timestamp * 1000);
    return date.toLocaleString();
  }

  function formatPermissions(mode) {
    const perms = ['---', '--x', '-w-', '-wx', 'r--', 'r-x', 'rw-', 'rwx'];
    const owner = perms[(mode >> 6) & 7];
    const group = perms[(mode >> 3) & 7];
    const others = perms[mode & 7];
    return owner + group + others;
  }
</script>

<div class="sftp-container">
  <!-- Header -->
  <div class="sftp-header">
    <div class="breadcrumb">
      <button class="btn-icon" on:click={goUp} disabled={currentPath === '/'} title="Go up">
        ⬆️
      </button>
      <span class="path">{currentPath}</span>
      <button class="btn-icon" on:click={() => loadDirectory(currentPath)} title="Refresh">
        🔄
      </button>
    </div>

    <div class="actions">
      <button class="btn-primary" on:click={handleUpload}>
        📤 Upload File
      </button>
    </div>
  </div>

  <!-- Content -->
  <div class="sftp-content">
    {#if loading}
      <div class="loading-container">
        <div class="spinner"></div>
        <div>Loading...</div>
      </div>
    {:else if error}
      <div class="error-container">
        <div class="error-text">{error}</div>
        <button class="btn-primary" on:click={() => loadDirectory(currentPath)}>
          Retry
        </button>
      </div>
    {:else}
      <table class="file-table">
        <thead>
          <tr>
            <th>Name</th>
            <th>Size</th>
            <th>Modified</th>
            <th>Permissions</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          {#each files as file (file.path)}
            <tr
              class="file-row"
              class:selected={selectedFile?.path === file.path}
              on:click={() => handleFileClick(file)}
              on:keydown={(e) => e.key === 'Enter' && handleFileClick(file)}
              role="button"
              tabindex="0"
            >
              <td class="file-name">
                <span class="file-icon">
                  {file.is_dir ? '📁' : '📄'}
                </span>
                {file.name}
              </td>
              <td class="file-size">
                {file.is_dir ? '-' : formatSize(file.size)}
              </td>
              <td class="file-date">
                {formatDate(file.modified)}
              </td>
              <td class="file-perms">
                {formatPermissions(file.permissions)}
              </td>
              <td class="file-actions">
                {#if !file.is_dir}
                  <button
                    class="btn-sm"
                    on:click|stopPropagation={() => handleDownload(file)}
                  >
                    Download
                  </button>
                {/if}
              </td>
            </tr>
          {/each}
        </tbody>
      </table>

      {#if files.length === 0}
        <div class="empty-container">
          <div class="empty-text">This directory is empty</div>
        </div>
      {/if}
    {/if}
  </div>
</div>

<style>
  .sftp-container {
    @apply flex flex-col w-full h-full bg-white dark:bg-gray-900;
  }

  .sftp-header {
    @apply flex items-center justify-between p-4 border-b border-gray-300 dark:border-gray-700;
  }

  .breadcrumb {
    @apply flex items-center gap-2 flex-1;
  }

  .path {
    @apply text-sm font-mono bg-gray-100 dark:bg-gray-800 px-3 py-1 rounded-md;
  }

  .actions {
    @apply flex gap-2;
  }

  .btn-icon {
    @apply p-2 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed;
  }

  .btn-primary {
    @apply px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors text-sm font-medium;
  }

  .btn-sm {
    @apply px-2 py-1 bg-gray-200 dark:bg-gray-700 rounded-md hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors text-xs;
  }

  .sftp-content {
    @apply flex-1 overflow-auto p-4;
  }

  .file-table {
    @apply w-full border-collapse text-sm;
  }

  .file-table thead {
    @apply bg-gray-100 dark:bg-gray-800 sticky top-0;
  }

  .file-table th {
    @apply text-left p-3 font-semibold;
  }

  .file-row {
    @apply border-b border-gray-200 dark:border-gray-800 hover:bg-gray-50 dark:hover:bg-gray-800 cursor-pointer transition-colors;
  }

  .file-row.selected {
    @apply bg-blue-50 dark:bg-blue-900;
  }

  .file-row td {
    @apply p-3;
  }

  .file-name {
    @apply flex items-center gap-2;
  }

  .file-icon {
    @apply text-lg;
  }

  .file-size,
  .file-date,
  .file-perms {
    @apply text-gray-600 dark:text-gray-400;
  }

  .file-perms {
    @apply font-mono text-xs;
  }

  .loading-container,
  .error-container,
  .empty-container {
    @apply flex flex-col items-center justify-center h-64;
  }

  .spinner {
    @apply w-12 h-12 border-4 border-blue-600 border-t-transparent rounded-full animate-spin mb-4;
  }

  .error-text,
  .empty-text {
    @apply text-gray-600 dark:text-gray-400 mb-4;
  }

  .error-text {
    @apply text-red-500;
  }
</style>