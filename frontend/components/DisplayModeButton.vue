<template>
  <button
    class="flex h-10 w-24 items-center justify-center rounded-full px-2 py-1 text-xs font-bold text-gray-700 transition-colors duration-300 outline-none select-none"
    :class="
      displayMode == DisplayMode.Probability
        ? 'bg-amber-300 hover:bg-amber-400 active:bg-amber-500'
        : 'bg-green-300 hover:bg-green-400 active:bg-green-500'
    "
    @click="handleDisplayModeChange"
  >
    {{ displayMode === "probability" ? "Probability" : "Entropy" }}
  </button>
</template>

<script setup lang="ts">
const displayModeStore = useDisplayModeStore()
const displayMode = computed(() => displayModeStore.mode)

/**
 * Handles button click event.
 * Switches the display mode between probability and entropy.
 */
const handleDisplayModeChange = () => {
  displayModeStore.setMode(
    displayMode.value == DisplayMode.Probability
      ? DisplayMode.Entropy
      : DisplayMode.Probability,
  )
}
</script>
