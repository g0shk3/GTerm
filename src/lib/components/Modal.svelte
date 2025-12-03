<script>
  import { createEventDispatcher } from 'svelte';

  export let title = '';
  export let size = 'large'; // 'small' | 'medium' | 'large' | 'xlarge'
  export let showCloseButton = true;

  const dispatch = createEventDispatcher();

  function handleBackdropClick() {
    dispatch('close');
  }

  function handleKeydown(e) {
    if (e.key === 'Escape') {
      dispatch('close');
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="modal-backdrop" on:click={handleBackdropClick}>
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div class="modal-container modal-{size}" on:click|stopPropagation>
    {#if title || showCloseButton}
      <div class="modal-header">
        {#if title}
          <h2 class="modal-title">{title}</h2>
        {/if}
        {#if showCloseButton}
          <button on:click={() => dispatch('close')} class="close-button" aria-label="Close">
            <svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M4 4l12 12M16 4L4 16"/>
            </svg>
          </button>
        {/if}
      </div>
    {/if}

    <div class="modal-body">
      <slot />
    </div>
  </div>
</div>

<style>
  .modal-backdrop {
    @apply fixed inset-0 z-40;
    @apply flex items-center justify-center;
    @apply bg-black bg-opacity-60;
    @apply backdrop-blur-sm;
    animation: fadeIn 0.15s ease-out;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  .modal-container {
    @apply bg-gray-800 rounded-xl shadow-2xl;
    @apply flex flex-col;
    @apply border border-gray-700;
    max-height: 90vh;
    animation: slideUp 0.2s ease-out;
    will-change: transform, opacity;
  }

  @keyframes slideUp {
    from {
      opacity: 0;
      transform: translateY(20px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .modal-small {
    width: 90%;
    max-width: 480px;
  }

  .modal-medium {
    width: 90%;
    max-width: 640px;
  }

  .modal-large {
    width: 90%;
    max-width: 800px;
  }

  .modal-xlarge {
    width: 90%;
    max-width: 1024px;
  }

  .modal-header {
    @apply flex items-center justify-between;
    @apply px-6 py-4;
    @apply border-b border-gray-700;
    flex-shrink: 0;
  }

  .modal-title {
    @apply text-xl font-semibold text-gray-100;
    @apply m-0;
  }

  .close-button {
    @apply p-1.5 rounded-lg;
    @apply text-gray-400 hover:text-gray-200;
    @apply hover:bg-gray-700;
    @apply transition-colors;
    @apply flex items-center justify-center;
  }

  .modal-body {
    @apply flex-1 overflow-auto;
    @apply p-6;
  }

  /* Custom scrollbar */
  .modal-body::-webkit-scrollbar {
    width: 8px;
  }

  .modal-body::-webkit-scrollbar-track {
    @apply bg-transparent;
  }

  .modal-body::-webkit-scrollbar-thumb {
    @apply bg-gray-600 rounded-full;
  }

  .modal-body::-webkit-scrollbar-thumb:hover {
    @apply bg-gray-500;
  }
</style>
