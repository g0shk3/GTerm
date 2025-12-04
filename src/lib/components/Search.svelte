<script>
  import { createEventDispatcher, onMount } from 'svelte';
  import { settings } from '../stores/settings.js';

  export let searchAddon;
  export let containerClassName = '';

  let searchTerm = '';
  let searchInput;
  let useRegex = false;

  const dispatch = createEventDispatcher();

  function handleSearch(direction) {
    if (!searchTerm) return;

    const searchOptions = {
      regex: useRegex,
      decorations: {
        matchBackground: '#4b5563', // gray-600 - бледо маркирани, не изпъкват много
        matchOverviewRuler: '#6b7280', // gray-500
        activeMatchBackground: '#60a5fa', // blue-400 - светло синьо за текущия
        activeMatchColorOverviewRuler: '#60a5fa', // blue-400
      }
    };

    if (direction === 'next') {
      searchAddon.findNext(searchTerm, searchOptions);
    } else {
      searchAddon.findPrevious(searchTerm, searchOptions);
    }

    // HACK: Refocus the input after a search action because xterm.js tends
    // to steal focus back to the terminal.
    setTimeout(() => {
      searchInput.focus();
    }, 0);
  }

  function handleKeyDown(event) {
    if (event.key === 'Enter') {
      event.preventDefault();
      event.stopPropagation();
      // Use search direction from settings
      const direction = $settings.searchDirection === 'bottomToTop' ? 'previous' : 'next';
      handleSearch(direction);
    }
    if (event.key === 'Escape') {
      event.preventDefault();
      handleClose();
    }
  }

  function handleClose() {
    searchAddon.clearDecorations();
    dispatch('close');
    // Refocus terminal after search closes
    setTimeout(() => {
      window.dispatchEvent(new CustomEvent('tabSwitched'));
    }, 50);
  }

  onMount(() => {
    // Focus the input field when the component is mounted
    setTimeout(() => {
      searchInput.focus();
    }, 0);
  });
</script>

<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
<div class="search-bar-wrapper {containerClassName}" on:keydown={handleKeyDown} role="search">
  <input
    type="text"
    bind:this={searchInput}
    bind:value={searchTerm}
    placeholder="Search"
    class="search-input"
    autocomplete="off"
    spellcheck="false"
    on:click|stopPropagation
    on:focus|stopPropagation
    on:mousedown|stopPropagation
  />
  <button
    on:click={() => { useRegex = !useRegex; handleSearch($settings.searchDirection === 'bottomToTop' ? 'previous' : 'next'); }}
    class="search-button {useRegex ? 'active' : ''}"
    title="Use Regex"
  >
    .*
  </button>
  <button on:click={() => handleSearch('previous')} class="search-button" title="Previous (↑)">
    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
      <path fill-rule="evenodd" d="M14.707 12.707a1 1 0 01-1.414 0L10 9.414l-3.293 3.293a1 1 0 01-1.414-1.414l4-4a1 1 0 011.414 0l4 4a1 1 0 010 1.414z" clip-rule="evenodd" />
    </svg>
  </button>
  <button on:click={() => handleSearch('next')} class="search-button" title="Next (↓)">
    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
      <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
    </svg>
  </button>
  <button on:click={handleClose} class="search-button" title="Close (Esc)">
    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
      <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
    </svg>
  </button>
</div>

<style>
  .search-bar-wrapper {
    @apply absolute top-3 right-5 z-10 flex items-center gap-1;
    @apply bg-gray-900 bg-opacity-80 backdrop-blur-sm;
    @apply border border-gray-700 rounded-lg shadow-2xl;
    @apply p-1.5;
    transition: all 0.2s ease-in-out;
  }

  .search-input {
    @apply bg-gray-800 border border-gray-600;
    @apply text-white text-sm rounded-md;
    @apply px-3 py-1.5;
    @apply w-64;
    @apply focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent;
  }

  .search-button {
    @apply flex items-center justify-center;
    @apply w-9 h-9;
    @apply bg-transparent border-0;
    @apply text-gray-400 rounded-md;
    @apply cursor-pointer;
    @apply transition-colors duration-150;
  }

  .search-button:hover {
    @apply bg-gray-700 text-white;
  }

  .search-button:disabled {
    @apply text-gray-600 cursor-not-allowed;
  }

  .search-button.active {
    @apply bg-blue-600 text-white;
  }
</style>
