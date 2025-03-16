<template>
  <div
    class="flex aspect-square w-full cursor-pointer items-center justify-center transition-colors duration-300"
    :class="[panelData !== null ? '' : isOpen ? 'bg-gray-100' : 'bg-gray-500']"
    :style="{ backgroundColor }"
    @click="$emit('click')"
  >
    <span
      v-if="panelData !== null"
      class="text-base font-bold"
      :style="{ color: textColor }"
    >{{ label }}</span>
  </div>
</template>

<script setup lang="ts">
/**
 * A panel item for the grid.
 * It can show the probability or entropy of a rectangle.
 * @param {boolean} isOpen - Whether the panel is open or not.
 * @param {object} probablity - Probability of the rectangle.
 * @param {object} entropy - Entropy of the rectangle.
 */
const props = defineProps({
  isOpen: {
    type: Boolean,
    required: true,
  },
  probablity: {
    type: Object as () => [number, Color] | null,
    default: null,
  },
  entropy: {
    type: Object as () => [number, Color] | null,
    default: null,
  },
})

/**
 * The current display mode.
 * @type {DisplayMode}
 */
const displayModeStore = useDisplayModeStore()

/**
 * The displayed data.
 * It is either the probability or the entropy.
 * @type {[number, Color] | null}
 */
const panelData = computed<[number, Color] | null>(() => {
  if (displayModeStore.mode == DisplayMode.Entropy) {
    return props.entropy
  }
  else {
    return props.probablity
  }
})

/**
 * The text color.
 * It is either black or white based on the brightness of the background color.
 * @type {string}
 */
const textColor = computed(() => {
  if (panelData.value !== null) {
    const rgb = panelData.value[1]
    const brightness = Math.round(
      (rgb[0] * 299 + rgb[1] * 587 + rgb[2] * 114) / 1000,
    )
    const color = brightness > 125 ? 'black' : 'white'
    return color
  }
  else {
    return 'black'
  }
})

/**
 * The background color.
 * It is the same as the color of the panel data.
 * @type {string}
 */
const backgroundColor = computed(() => {
  if (panelData.value !== null) {
    return `rgb(${panelData.value[1][0]}, ${panelData.value[1][1]}, ${panelData.value[1][2]})`
  }
  return ''
})

/**
 * The label of the panel.
 * It is the value of the panel data.
 * @type {string}
 */
const label = computed(() => {
  if (panelData.value !== null) {
    return panelData.value[0].toFixed(2)
  }
  return ''
})

/**
 * Emits a click event to the parent.
 * @function
 */
defineEmits(['click'])
</script>
