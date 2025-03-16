<template>
  <div class="flex items-center gap-2">
    <div
      class="mr-2 grid gap-1"
      :style="{ gridTemplateColumns: `repeat(${itemSize.rows}, 1fr)` }"
    >
      <div
        v-for="index in itemSize.rows * itemSize.cols"
        :key="index"
        class="h-4 w-4 rounded-sm bg-gray-300 transition-colors duration-500 dark:bg-teal-800"
      />
    </div>
    <select
      v-model="selectedItemSizePresetIndex"
      class="rounded-full border border-gray-300 bg-white px-2 py-1 transition-colors duration-500 outline-none dark:border-gray-500 dark:bg-slate-700 dark:text-white"
      @change="handleSelectChange($event, 'itemSize')"
    >
      <option
        v-for="(preset, index) in presets"
        :key="index"
        :value="index"
      >
        {{ preset.toLabel() }}
      </option>
    </select>
    <select
      v-model="selectedNumItems"
      class="rounded-full border border-gray-300 bg-white px-2 py-1 transition-colors duration-500 outline-none dark:border-gray-500 dark:bg-slate-700 dark:text-white"
      @change="handleSelectChange($event, 'numItems')"
    >
      <option
        v-for="num in numItemOptions"
        :key="num"
        :value="num"
      >
        {{ num }}
      </option>
    </select>
  </div>
</template>

<script setup lang="ts">
/**
 * A select component for choosing the number of items.
 */
const props = defineProps<{
  /**
   * The index of the setting.
   */
  settingIndex: number
}>()

/**
 * The options for the number of items.
 */
const numItemOptions = Array.from({ length: 7 }).map((_, i) => i)

/**
 * The presets for the item size.
 */
const presets = [
  new ItemSize(2, 1),
  new ItemSize(3, 1),
  new ItemSize(3, 2),
  new ItemSize(3, 3),
]

/**
 * The item store.
 */
const itemStore = useItemStore()

/**
 * The selected item size preset index.
 */
const selectedItemSizePresetIndex = ref(0)

/**
 * The selected number of items.
 */
const selectedNumItems = ref(0)

/**
 * The item size.
 */
const itemSize = computed(() => presets[selectedItemSizePresetIndex.value])

/**
 * Handles the select change event.
 * @param {Event} event - The event.
 * @param {'itemSize' | 'numItems'} type - The type of the select.
 */
const handleSelectChange = (event: Event, type: 'itemSize' | 'numItems') => {
  const value = (event.target as HTMLSelectElement).value
  const size = type === 'itemSize' ? presets[Number(value)] : undefined
  const count = type === 'numItems' ? Number(value) : undefined

  if (size) {
    itemStore.setSize(props.settingIndex, size)
  }
  if (count) {
    itemStore.setCount(props.settingIndex, count)
  }
}
</script>
