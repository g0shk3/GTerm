<script>
  import { createEventDispatcher, onMount } from 'svelte';

  const dispatch = createEventDispatcher();

  function handleKeydown(event) {
    if (event.key === 'Enter') {
      dispatch('yes');
    } else if (event.key === 'Escape') {
      dispatch('no');
    }
  }

  onMount(() => {
    // Focus the 'Yes' button by default
    const yesButton = document.querySelector('.confirm-btn-yes');
    if (yesButton) {
      yesButton.focus();
    }
  });
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="modal-backdrop">
  <div class="modal-content" role="dialog" aria-modal="true" aria-labelledby="modal-title">
    <h2 id="modal-title" class="modal-title">Are you sure?</h2>
    <p class="modal-body">Do you really want to close this tab?</p>
    <div class="modal-actions">
      <button class="modal-btn confirm-btn-no" on:click={() => dispatch('no')}>No</button>
      <button class="modal-btn confirm-btn-yes" on:click={() => dispatch('yes')}>Yes</button>
    </div>
  </div>
</div>

<style>
  .modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.6);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 100;
  }

  .modal-content {
    background-color: #1f2937; /* gray-800 */
    color: #f3f4f6; /* gray-100 */
    padding: 2rem;
    border-radius: 0.5rem;
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
    width: 90%;
    max-width: 400px;
    text-align: center;
    border: 1px solid #374151; /* gray-700 */
  }

  .modal-title {
    font-size: 1.5rem;
    font-weight: 600;
    margin-bottom: 0.5rem;
  }

  .modal-body {
    margin-bottom: 1.5rem;
    color: #d1d5db; /* gray-300 */
  }

  .modal-actions {
    display: flex;
    justify-content: center;
    gap: 1rem;
  }

  .modal-btn {
    padding: 0.5rem 1.5rem;
    border-radius: 0.375rem;
    border: 1px solid transparent;
    font-weight: 500;
    cursor: pointer;
    transition: background-color 0.2s, border-color 0.2s;
  }

  .confirm-btn-no {
    background-color: #374151; /* gray-700 */
    color: #f3f4f6; /* gray-100 */
  }

  .confirm-btn-no:hover {
    background-color: #4b5563; /* gray-600 */
  }

  .confirm-btn-yes {
    background-color: #2563eb; /* blue-600 */
    color: white;
  }

  .confirm-btn-yes:hover {
    background-color: #1d4ed8; /* blue-700 */
  }

  .confirm-btn-yes:focus {
    outline: 2px solid #fbbf24; /* amber-400 */
    outline-offset: 2px;
  }
</style>
