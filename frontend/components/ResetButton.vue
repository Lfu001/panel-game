<template>
  <button
    class="cursor-pointer rounded-full bg-red-500 px-4 py-2 text-white outline-none select-none hover:bg-red-700"
    @click="resetGrid"
  >
    Reset
  </button>
</template>

<script setup lang="ts">
/**
 * A button that resets the grid mask to its original state.
 * Uses the `useGridStore` composable to get the grid store.
 * @see {@link useGridStore}
 */
const gridStore = useGridStore()

/**
 * Resets the grid mask to its original state.
 */
const resetGrid = () => {
  gridStore.reset()
}
</script>
