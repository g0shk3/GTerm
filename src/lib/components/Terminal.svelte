<script>
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { Terminal as XTerm } from '@xterm/xterm';
  import { FitAddon } from '@xterm/addon-fit';
  import { WebLinksAddon } from '@xterm/addon-web-links';
  import '@xterm/xterm/css/xterm.css';
  import { updateTabConnection, activeTabId } from '../stores/tabs';
  import { getSnippets } from '../stores/snippets';

  export let tab;

  let terminalElement;
  let terminal;
  let fitAddon;
  let unlistenOutput;
  let unlistenClosed;
  let unlistenError;
  let connecting = true;
  let errorMessage = '';

  onMount(async () => {
    // Initialize xterm.js
    terminal = new XTerm({
      cursorBlink: true,
      fontSize: 14,
      fontFamily: '"SF Mono", Monaco, Menlo, "Courier New", monospace',
      theme: {
        background: '#1f2937', // gray-800 - same as sidebar and welcome screen
        foreground: '#ffffff',
        cursor: '#d0d0d0',
        selection: 'rgba(255, 255, 255, 0.3)',
        black: '#000000',
        red: '#ff5555',
        green: '#50fa7b',
        yellow: '#f1fa8c',
        blue: '#bd93f9',
        magenta: '#ff79c6',
        cyan: '#8be9fd',
        white: '#bbbbbb',
        brightBlack: '#555555',
        brightRed: '#ff5555',
        brightGreen: '#50fa7b',
        brightYellow: '#f1fa8c',
        brightBlue: '#bd93f9',
        brightMagenta: '#ff79c6',
        brightCyan: '#8be9fd',
        brightWhite: '#ffffff',
      },
      allowProposedApi: true,
    });

    fitAddon = new FitAddon();
    terminal.loadAddon(fitAddon);
    terminal.loadAddon(new WebLinksAddon());

    terminal.open(terminalElement);
    fitAddon.fit();

    // Handle user input
    terminal.onData(async (data) => {
      try {
        await invoke('ssh_send_input', {
          sessionId: tab.sessionId,
          data: data,
        });
      } catch (error) {
        console.error('Failed to send input:', error);
      }
    });

    // Handle terminal resize
    terminal.onResize(async ({ cols, rows }) => {
      try {
        await invoke('ssh_resize', {
          sessionId: tab.sessionId,
          cols,
          rows,
        });
      } catch (error) {
        console.error('Failed to resize:', error);
      }
    });

    // Listen for SSH output
    unlistenOutput = await listen(`ssh-output:${tab.sessionId}`, (event) => {
      terminal.write(event.payload);
    });

    // Listen for SSH closed
    unlistenClosed = await listen(`ssh-closed:${tab.sessionId}`, () => {
      terminal.write('\r\n\x1b[31mConnection closed\x1b[0m\r\n');
      updateTabConnection(tab.id, false);
    });

    // Listen for SSH errors
    unlistenError = await listen(`ssh-error:${tab.sessionId}`, (event) => {
      terminal.write(`\r\n\x1b[31mError: ${event.payload}\x1b[0m\r\n`);
      updateTabConnection(tab.id, false);
    });

    // Window resize handler
    window.addEventListener('resize', handleResize);

    // Clear terminal handler
    window.addEventListener('clearTerminal', handleClearTerminal);

    // Connect to SSH
    await connectSSH();
  });

  async function connectSSH() {
    try {
      connecting = true;
      errorMessage = '';

      await invoke('ssh_connect', {
        sessionId: tab.sessionId,
        host: tab.host.host,
        port: tab.host.port,
        username: tab.host.username,
        privateKeyPath: tab.host.privateKeyPath,
        passphrase: tab.host.passphrase || null,
      });

      connecting = false;
      updateTabConnection(tab.id, true);

      // Fit terminal after connection
      setTimeout(() => {
        fitAddon.fit();
        // Focus terminal so user can start typing immediately
        terminal.focus();
      }, 100);

      // Execute snippet if assigned
      if (tab.host.snippetId) {
        await executeSnippet(tab.host.snippetId);
      }
    } catch (error) {
      connecting = false;
      errorMessage = `Failed to connect: ${error}`;
      terminal.write(`\r\n\x1b[31m${errorMessage}\x1b[0m\r\n`);
      updateTabConnection(tab.id, false);
    }
  }

  async function executeSnippet(snippetId) {
    try {
      const snippets = await getSnippets();
      const snippet = snippets.find(s => s.id === snippetId);

      if (snippet && snippet.content) {
        // Add a small delay to ensure terminal is ready
        await new Promise(resolve => setTimeout(resolve, 500));

        // Send the snippet command with Enter
        await invoke('ssh_send_input', {
          sessionId: tab.sessionId,
          data: snippet.content + '\n',
        });
      }
    } catch (error) {
      console.error('Failed to execute snippet:', error);
    }
  }

  function handleResize() {
    if (fitAddon) {
      fitAddon.fit();
    }
  }

  function handleClearTerminal() {
    if (terminal) {
      terminal.clear();
    }
  }

  onDestroy(async () => {
    window.removeEventListener('resize', handleResize);
    window.removeEventListener('clearTerminal', handleClearTerminal);

    if (unlistenOutput) await unlistenOutput();
    if (unlistenClosed) await unlistenClosed();
    if (unlistenError) await unlistenError();

    try {
      await invoke('ssh_disconnect', { sessionId: tab.sessionId });
    } catch (error) {
      console.error('Failed to disconnect:', error);
    }

    if (terminal) {
      terminal.dispose();
    }
  });

  // Focus terminal when this tab becomes active
  $: if (terminal && $activeTabId === tab.id) {
    terminal.focus();
  }
</script>

<div class="terminal-wrapper">
  {#if connecting}
    <div class="connecting-overlay">
      <div class="connecting-spinner"></div>
      <div class="connecting-text">Connecting to {tab.host.host}...</div>
    </div>
  {/if}

  {#if errorMessage}
    <div class="error-overlay">
      <div class="error-text">{errorMessage}</div>
      <button class="btn-retry" on:click={connectSSH}>Retry</button>
    </div>
  {/if}

  <div class="terminal-container" bind:this={terminalElement}></div>
</div>

<style>
  .terminal-wrapper {
    @apply relative w-full h-full;
  }

  .terminal-container {
    @apply w-full h-full;
  }

  .connecting-overlay,
  .error-overlay {
    @apply absolute inset-0 flex flex-col items-center justify-center z-10;
    background-color: rgba(31, 41, 55, 0.95); /* gray-800 with opacity */
  }

  .connecting-spinner {
    @apply w-12 h-12 border-4 border-blue-600 border-t-transparent rounded-full animate-spin mb-4;
  }

  .connecting-text,
  .error-text {
    @apply text-white text-lg;
  }

  .error-text {
    @apply text-red-500 mb-4;
  }

  .btn-retry {
    @apply px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors;
  }
</style>
