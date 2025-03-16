<template>
  <button
    class="btn w-full max-w-100 cursor-pointer rounded-full bg-blue-500 px-4 py-2 font-bold text-white hover:bg-blue-700 active:bg-blue-800"
    :class="{ 'opacity-50 cursor-wait': waiting }"
    :disabled="waiting"
    @click="$emit('estimate')"
  >
    <span
      v-if="waiting"
      class="animate-spin h-5 w-5 border-b-2 border-white mr-2"
    />
    <span>{{ waiting ? 'Processing...' : 'Estimate' }}</span>
  </button>
</template>

<script setup lang="ts">
defineProps<{
  waiting: boolean
}>()

defineEmits(['estimate'])
</script>
