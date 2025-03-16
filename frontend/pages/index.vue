<template>
  <div class="grid grid-rows-[auto,1fr] gap-4 p-4">
    <DisplayModeButton />
    <PanelGridContainer :inference-result="estimates" />
    <HiddenItemSetting
      v-for="i in 4"
      :key="i"
      :setting-index="i - 1"
    />
    <div class="flex justify-center space-x-2">
      <EstimateButton
        :waiting="isShowWaitIndicator"
        @estimate="requestInference"
      />
      <ResetButton />
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * The estimates of the inference result.
 */
const estimates = ref<InferenceResult | null>(null)

/**
 * Whether the waiting indicator is shown.
 * It is true when the inference result is being requested from the server.
 */
const isShowWaitIndicator = ref(false)

/**
 * The grid store.
 */
const gridStore = useGridStore()

/**
 * The item store.
 */
const itemStore = useItemStore()

/**
 * Requests the inference result from the server.
 */
const requestInference = async () => {
  isShowWaitIndicator.value = true

  // Get the current grid mask and item settings
  const grid: Grid<boolean> = {
    rows: gridStore.size.rows,
    cols: gridStore.size.cols,
    data: gridStore.mask,
  }
  const param = {
    mask: grid,
    rectangles: itemStore.toRectangleArray(),
  }

  // Send the request
  const data: InferenceResult = await $fetch('/estimate', {
    method: 'POST',
    body: param,
  })
  estimates.value = data

  isShowWaitIndicator.value = false
}

/**
 * Listens to the grid store and resets the estimates when all cells are masked.
 */
useGridStore().$subscribe((_) => {
  if (gridStore.isAllMasked()) {
    estimates.value = null
  }
})
</script>
