<script>
  export let checked = false;
</script>

<label class="relative inline-flex items-center cursor-pointer">
  <input type="checkbox" bind:checked on:change class="sr-only peer">
  <div class="w-11 h-6 bg-gray-700 rounded-full peer peer-focus:ring-2 peer-focus:ring-blue-500 peer-focus:ring-offset-2 peer-focus:ring-offset-gray-900 peer-checked:bg-blue-600"></div>
  <span class="absolute left-0.5 top-0.5 w-5 h-5 bg-white rounded-full transition-transform peer-checked:translate-x-full"></span>
</label>

<style>
    span {
        transition: transform 0.2s ease-in-out;
    }
</style>
