<script>
  import { onMount, onDestroy } from 'svelte';
  import { get, derived } from 'svelte/store';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { writeText } from '@tauri-apps/plugin-clipboard-manager';
  import { Terminal as XTerm } from '@xterm/xterm';
  import { FitAddon } from '@xterm/addon-fit';
  import { WebLinksAddon } from '@xterm/addon-web-links';
  import { SearchAddon } from '@xterm/addon-search';
  import '@xterm/xterm/css/xterm.css';
  import { tabs, closeTab, closePane, updatePaneConnection, activeTabId } from '../stores/tabs';
  import { getSnippets } from '../stores/snippets';
  import { settings } from '../stores/settings';
  import Search from './Search.svelte';

  export let pane;
  export let tabId;

  let terminalElement;
  let terminal;
  let fitAddon;
  let searchAddon;
  let webLinksAddon;
  const unlistenPromises = [];
  let connecting = true;
  let errorMessage = '';
  let showSearch = false;
  let executingSnippet = false;
  let isDestroyed = false;
  let resizeAnimationFrameId = null;

  const activePaneId = derived(tabs, $tabs => {
    const currentTab = $tabs.find(t => t.id === tabId);
    return currentTab ? currentTab.activePaneId : null;
  });

  // Handle focusing the terminal
  $: if (terminal && $activeTabId === tabId && $activePaneId === pane.id) {
    // Use a small timeout to ensure the terminal is visible and ready,
    // especially after layout changes like closing a pane or switching tabs.
    setTimeout(() => {
      terminal.focus();
    }, 50);
  }

  // Helper function to check if terminal is visible
  function isTerminalVisible() {
    if (!terminalElement) return false;
    const rect = terminalElement.getBoundingClientRect();
    return rect.width > 0 && rect.height > 0;
  }

  onMount(async () => {
    // Initialize xterm.js
    terminal = new XTerm({
      cursorBlink: true,
      fontSize: 14,
      fontFamily: 'Menlo, Monaco, "Courier New", monospace',
      scrollback: $settings.scrollback,
      fastScrollModifier: 'shift',
      fastScrollSensitivity: 5,
      rows: 24, // Set initial rows
      cols: 80, // Set initial cols
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
      windowsMode: false,
      convertEol: false,
      screenReaderMode: false,
      allowTransparency: false,
      drawBoldTextInBrightColors: true,
      rightClickSelectsWord: true,
      smoothScrollDuration: 0, // Disable smooth scrolling for better performance
    });

    fitAddon = new FitAddon();
    searchAddon = new SearchAddon();
    webLinksAddon = new WebLinksAddon();
    terminal.loadAddon(fitAddon);
    terminal.loadAddon(webLinksAddon);
    terminal.loadAddon(searchAddon);

    terminal.open(terminalElement);

    // Only fit if terminal is visible (prevents issues when creating multiple tabs quickly)
    if (isTerminalVisible()) {
      fitAddon.fit();
    }

    // Copy on select
    terminal.onSelectionChange(async () => {
      if (get(settings).autoCopyOnSelect) {
        const selection = terminal.getSelection();
        if (selection) {
          await writeText(selection);
        }
      }
    });

    // Add find shortcut
    terminal.attachCustomKeyEventHandler((event) => {
      // Use event.code for layout-independent shortcuts
      if (event.code === 'KeyF' && (event.metaKey || event.ctrlKey) && event.type === 'keydown') {
        event.preventDefault();
        showSearch = true;
        return false; // Prevent event from being processed further
      }
      if (event.key === 'Escape' && showSearch) {
        event.preventDefault();
        showSearch = false;
        searchAddon.clearDecorations();
        return false;
      }
      // Add close on Ctrl+D if connection is closed
      if (event.code === 'KeyD' && event.ctrlKey && event.type === 'keydown') {
        const currentTabs = get(tabs);
        const currentTab = currentTabs.find(t => t.id === tabId);
        if (!currentTab) return true;

        const currentPane = currentTab.panes.find(p => p.id === pane.id);

        if (currentPane && !currentPane.connected) {
          event.preventDefault();

          if (currentTab.panes.length > 1) {
            closePane(tabId, pane.id);
          } else {
            closeTab(tabId);
          }
          return false;
        }
      }
      return true;
    });

    // Handle user input
    terminal.onData(async (data) => {
      if (isDestroyed) return;
      // Block user input while executing snippet to prevent race condition
      if (executingSnippet) {
        return;
      }

      try {
        const connectionType = pane.host?.type || 'ssh';
        const command = connectionType === 'local' ? 'local_send_input' : 'ssh_send_input';
        await invoke(command, {
          sessionId: pane.sessionId,
          data: data,
        });
      } catch (error) {
        console.error('[Terminal] Failed to send input:', error);
      }
    });

    // Handle terminal resize
    terminal.onResize(async ({ cols, rows }) => {
      if (isDestroyed) return;
      try {
        const connectionType = pane.host?.type || 'ssh';
        const command = connectionType === 'local' ? 'local_resize' : 'ssh_resize';
        await invoke(command, {
          sessionId: pane.sessionId,
          cols,
          rows,
        });
      } catch (error) {
        console.error('[Terminal] Failed to resize:', error);
      }
    });

    // Determine event channel prefix based on connection type
    const connectionType = pane.host?.type || 'ssh';
    const eventPrefix = connectionType === 'local' ? 'terminal' : 'ssh';

    // Listen for terminal output
    unlistenPromises.push(listen(`${eventPrefix}-output:${pane.sessionId}`, (event) => {
      // event.payload is number[]
      if (terminal) {
        terminal.write(new Uint8Array(event.payload));
      }
    }));

    // Listen for connection closed
    unlistenPromises.push(listen(`${eventPrefix}-closed:${pane.sessionId}`, () => {
      terminal.write('\r\n\x1b[31mConnection closed\x1b[0m\r\n');
      updatePaneConnection(tabId, pane.id, false);
    }));

    // Listen for errors
    unlistenPromises.push(listen(`${eventPrefix}-error:${pane.sessionId}`, (event) => {
      terminal.write(`\r\n\x1b[31mError: ${event.payload}\x1b[0m\r\n`);
      updatePaneConnection(tabId, pane.id, false);
    }));

    // Window resize handler
    window.addEventListener('resize', handleResize);

    // Clear terminal handler
    window.addEventListener('clearTerminal', handleClearTerminal);

    // Tab switched handler - refocus terminal
    window.addEventListener('tabSwitched', handleTabSwitched);

    // Connect based on type
    if (connectionType === 'local') {
      await connectLocal();
    } else {
      await connectSSH();
    }
  });

  async function connectSSH() {
    if (isDestroyed) return;
    try {
      connecting = true;
      errorMessage = '';

      await invoke('ssh_connect', {
        sessionId: pane.sessionId,
        host: pane.host.host,
        port: pane.host.port,
        username: pane.host.username,
        privateKeyPath: pane.host.privateKeyPath,
        passphrase: pane.host.passphrase || null,
      });

      connecting = false;
      updatePaneConnection(tabId, pane.id, true);

      // Fit terminal after connection
      setTimeout(async () => {
        if (isDestroyed) return;
        // Only fit if terminal is visible
        if (isTerminalVisible()) {
          // First fit the terminal to the container
          fitAddon.fit();

          // Wait for the next frame to ensure fit has completed
          await new Promise(resolve => requestAnimationFrame(resolve));

          // Now get the actual terminal dimensions after fit
          const { cols, rows } = terminal;

          // Manually trigger a resize event to sync backend with actual terminal size
          try {
            await invoke('ssh_resize', {
              sessionId: pane.sessionId,
              cols,
              rows,
            });
          } catch (error) {
            console.error('Failed to send initial resize:', error);
          }
          // Focus terminal so user can start typing immediately
          if ($activeTabId === tabId && $activePaneId === pane.id) {
            terminal.focus();
          }
        }
      }, 150);

      // Execute snippet if assigned
      if (pane.host.snippetId) {
        await executeSnippet(pane.host.snippetId);
      }
    } catch (error) {
      connecting = false;
      errorMessage = `Failed to connect: ${error}`;
      terminal.write(`\r\n\x1b[31m${errorMessage}\x1b[0m\r\n`);
      updatePaneConnection(tabId, pane.id, false);
    }
  }

  async function connectLocal() {
    if (isDestroyed) return;
    try {
      connecting = true;
      errorMessage = '';

      await invoke('local_connect', {
        sessionId: pane.sessionId,
        shell: pane.host.shell || null,
        cwd: pane.host.cwd || null,
      });

      connecting = false;
      updatePaneConnection(tabId, pane.id, true);

      // Fit terminal after connection
      setTimeout(async () => {
        if (isDestroyed) return;
        // Only fit if terminal is visible
        if (isTerminalVisible()) {
          // First fit the terminal to the container
          fitAddon.fit();

          // Wait for the next frame to ensure fit has completed
          await new Promise(resolve => requestAnimationFrame(resolve));

          // Now get the actual terminal dimensions after fit
          const { cols, rows } = terminal;

          // Manually trigger a resize event to sync backend with actual terminal size
          try {
            await invoke('local_resize', {
              sessionId: pane.sessionId,
              cols,
              rows,
            });
          } catch (error) {
            console.error('Failed to send initial resize:', error);
          }
          // Focus terminal so user can start typing immediately
          if ($activeTabId === tabId && $activePaneId === pane.id) {
            terminal.focus();
          }
        }
      }, 150);

      // Execute snippet if assigned
      if (pane.host.snippetId) {
        await executeSnippet(pane.host.snippetId);
      }
    } catch (error) {
      connecting = false;
      errorMessage = `Failed to start local terminal: ${error}`;
      terminal.write(`\r\n\x1b[31m${errorMessage}\x1b[0m\r\n`);
      updatePaneConnection(tabId, pane.id, false);
    }
  }

  async function executeSnippet(snippetId) {
    if (isDestroyed) return;
    try {
      // Set flag to block user input during snippet execution
      executingSnippet = true;

      const snippets = await getSnippets();
      const snippet = snippets.find(s => s.id === snippetId);

      if (snippet && snippet.content) {
        // Add a small delay to ensure terminal is ready (reduced from 500ms for better UX)
        await new Promise(resolve => setTimeout(resolve, 200));

        const connectionType = pane.host?.type || 'ssh';
        const command = connectionType === 'local' ? 'local_send_input' : 'ssh_send_input';

        // Send the snippet command with Enter
        await invoke(command, {
          sessionId: pane.sessionId,
          data: snippet.content + '\n',
        });
      }
    } catch (error) {
      console.error('Failed to execute snippet:', error);
    } finally {
      // Always clear the flag, even if there was an error
      executingSnippet = false;
    }
  }

  function handleResize() {
    // Only resize if this terminal is in the active tab
    if ($activeTabId !== tabId) {
      return;
    }

    if (fitAddon && terminal && isTerminalVisible()) {
      // Use requestAnimationFrame to ensure DOM has been updated
      if (resizeAnimationFrameId) {
        cancelAnimationFrame(resizeAnimationFrameId);
      }
      resizeAnimationFrameId = requestAnimationFrame(() => {
        try {
          if (isTerminalVisible()) {
            fitAddon.fit();
          }
        } catch (error) {
          console.error('Failed to fit terminal:', error);
        }
      });
    }
    // Refocus terminal after resize (e.g., when sidebar toggles)
    if (terminal && $activeTabId === tabId && $activePaneId === pane.id) {
      setTimeout(() => {
        terminal.focus();
      }, 50);
    }
  }

  function handleClearTerminal(event) {
    if (terminal) {
      const { detail } = event;
      // If the event is for this terminal, clear it
      if (detail && detail.tabId === tabId && detail.paneId === pane.id) {
        terminal.clear();
      }
    }
  }

  async function handleTabSwitched() {
    // Refocus and resize terminal when switching to this tab
    if (terminal && fitAddon && $activeTabId === tabId) {
      // This ensures the terminal is resized correctly when it becomes visible,
      // as fitAddon cannot work on hidden elements.
      // Use multiple animation frames to ensure DOM is fully updated
      requestAnimationFrame(() => {
        requestAnimationFrame(async () => {
          if (isTerminalVisible() && !isDestroyed) {
            // Force a fit even if previously fitted
            try {
              fitAddon.fit();

              // Wait a tick for fit to complete
              await new Promise(resolve => setTimeout(resolve, 10));

              // Manually send resize to backend to ensure it's synced
              const { cols, rows } = terminal;
              const connectionType = pane.host?.type || 'ssh';
              const command = connectionType === 'local' ? 'local_resize' : 'ssh_resize';

              try {
                await invoke(command, {
                  sessionId: pane.sessionId,
                  cols,
                  rows,
                });
                console.log(`[Terminal] Resized on tab switch: ${cols}x${rows}`);
              } catch (error) {
                console.error('[Terminal] Failed to send resize on tab switch:', error);
              }

              // Focus the terminal
              if ($activePaneId === pane.id) {
                terminal.focus();
              }
            } catch (error) {
              console.error('Failed to fit terminal on tab switch:', error);
            }
          }
        });
      });
    }
  }

  onDestroy(async () => {
    isDestroyed = true;

    // Cancel any pending animation frames
    if (resizeAnimationFrameId) {
      cancelAnimationFrame(resizeAnimationFrameId);
    }

    window.removeEventListener('resize', handleResize);
    window.removeEventListener('clearTerminal', handleClearTerminal);
    window.removeEventListener('tabSwitched', handleTabSwitched);

    // This is the fix: wait for the listen promises to resolve
    // and then call the returned unlisten functions. This is more robust
    // than Promise.all as it won't fail if one promise rejects.
    unlistenPromises.forEach(p => {
      p.then(unlistenFn => {
        if (unlistenFn) {
          unlistenFn();
        }
      }).catch(err => {
        console.error("A listener promise failed to resolve for cleanup:", err);
      });
    });

    try {
      const connectionType = pane.host?.type || 'ssh';
      const command = connectionType === 'local' ? 'local_disconnect' : 'ssh_disconnect';
      await invoke(command, { sessionId: pane.sessionId });
    } catch (error) {
      console.error('Failed to disconnect:', error);
    }

    // Dispose addons before the terminal itself
    if (fitAddon) {
      fitAddon.dispose();
    }
    if (searchAddon) {
      searchAddon.dispose();
    }
    if (webLinksAddon) {
      webLinksAddon.dispose();
    }

    if (terminal) {
      terminal.dispose();
    }
  });
</script>

<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
<div
  class="terminal-wrapper"
  on:click={() => terminal?.focus()}
  on:keydown={(e) => {
    if (e.key === 'Enter') {
      e.preventDefault();
      terminal?.focus();
    } else if (e.key === ' ') {
      terminal?.focus();
    }
  }}
  tabindex="0"
  role="application"
  aria-label="Terminal"
>
  {#if showSearch}
    <Search {searchAddon} on:close={() => showSearch = false} />
  {/if}

  {#if connecting}
    <div class="connecting-overlay">
      <div class="connecting-spinner"></div>
      <div class="connecting-text">
        {#if pane.host?.type === 'local'}
          Starting local terminal...
        {:else}
          Connecting to {pane.host.host}...
        {/if}
      </div>
    </div>
  {/if}

  {#if errorMessage}
    <div class="error-overlay">
      <div class="error-text">{errorMessage}</div>
      <button class="btn-retry" on:click={pane.host?.type === 'local' ? connectLocal : connectSSH}>Retry</button>
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