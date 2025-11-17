<script>
  import { createEventDispatcher, onMount } from 'svelte';

  export let searchAddon;

  let searchTerm = '';
  let searchInput;

  const dispatch = createEventDispatcher();

  function handleSearch(direction) {
    if (!searchTerm) return;
    if (direction === 'next') {
      searchAddon.findNext(searchTerm);
    } else {
      searchAddon.findPrevious(searchTerm);
    }
  }

  function handleKeyDown(event) {
    if (event.key === 'Enter') {
      handleSearch('next');
    }
    if (event.key === 'Escape') {
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
    searchInput.focus();
  });
</script>

<div class="search-bar-wrapper" on:keydown={handleKeyDown}>
  <input
    type="text"
    bind:this={searchInput}
    bind:value={searchTerm}
    placeholder="Search"
    class="search-input"
  />
  <button on:click={() => handleSearch('previous')} class="search-button">↑</button>
  <button on:click={() => handleSearch('next')} class="search-button">→</button>
  <button on:click={handleClose} class="search-button">×</button>
</div>

<style>
  .search-bar-wrapper {
    position: absolute;
    top: 0;
    right: 0;
    z-index: 10;
    background-color: #1f2937; /* gray-800 */
    padding: 4px;
    border-radius: 0 0 0 5px;
    display: flex;
    align-items: center;
    border-left: 1px solid #374151; /* gray-700 */
    border-bottom: 1px solid #374151; /* gray-700 */
  }

  .search-input {
    background-color: #111827; /* gray-900 */
    border: 1px solid #374151; /* gray-700 */
    color: #ffffff;
    padding: 2px 4px;
    width: 180px;
  }

  .search-input:focus {
    outline: none;
    border-color: #4b5563; /* gray-600 */
  }

  .search-button {
    background-color: transparent;
    border: none;
    color: #ffffff;
    cursor: pointer;
    padding: 0 8px;
    font-size: 16px;
    line-height: 1;
  }

  .search-button:hover {
    background-color: #374151; /* gray-700 */
  }

  .search-button:disabled {
    color: #6b7280; /* gray-500 */
    cursor: default;
  }
</style>
