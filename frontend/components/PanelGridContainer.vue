<template>
  <div class="grid h-full w-full grid-cols-9 gap-[0.5rem]">
    <GridPanelItem
      v-for="index in 45"
      :key="index - 1"
      :is-open="isOpen[index - 1]"
      :probablity="getProbability(index - 1)"
      :entropy="getEntropy(index - 1)"
      @click="handleClick(index - 1)"
    />
  </div>
</template>

<script setup lang="ts">
/**
 * Props for PanelGridContainer component.
 * @type {Object}
 * @property {InferenceResult | null} inferenceResult - The inference result containing probability and entropy data.
 */
const props = defineProps({
  inferenceResult: {
    type: Object as () => InferenceResult | null,
    default: null,
  },
})

/**
 * Computes the probability for a given index.
 * @param {number} i - The index of the panel.
 * @returns {[number, Color] | null} The probability and its color or null if no data is available.
 */
const getProbability = (i: number): [number, Color] | null => {
  const { row, col } = indexToRowCol(i)
  if (props.inferenceResult !== null) {
    return props.inferenceResult.probabilities.data[row][col]
  }
  return null
}

/**
 * Computes the entropy for a given index.
 * @param {number} i - The index of the panel.
 * @returns {[number, Color] | null} The entropy and its color or null if no data is available.
 */
const getEntropy = (i: number): [number, Color] | null => {
  const { row, col } = indexToRowCol(i)
  if (props.inferenceResult !== null) {
    return props.inferenceResult.entropy.data[row][col]
  }
  return null
}

/**
 * Converts a flat index to a row and column index.
 * @param {number} index - The flat index.
 * @returns {Object} An object containing row and column indices.
 */
const indexToRowCol = (index: number) => {
  const row = Math.floor(index / gridStore.size.cols)
  const col = index % gridStore.size.cols
  return { row, col }
}

/**
 * Store for grid data.
 */
const gridStore = useGridStore()

/**
 * Computed property that determines if a panel is open.
 * @type {boolean[]}
 */
const isOpen = computed(() => gridStore.mask.flat())

/**
 * Handles click event on a panel item.
 * Flips the mask state of the grid at the given index.
 * @param {number} i - The index of the panel.
 */
const handleClick = (i: number) => {
  const { row, col } = indexToRowCol(i)
  gridStore.flipMask(col, row)
}
</script>
