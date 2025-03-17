<template>
  <button
    class="btn w-full max-w-100 cursor-pointer rounded-full bg-blue-500 px-4 py-2 font-bold text-white select-none hover:bg-blue-700 active:bg-blue-800"
    :class="{ 'cursor-wait opacity-50': waiting }"
    :disabled="waiting"
    @click="$emit('estimate')"
  >
    <span
      v-if="waiting"
      class="mr-2 h-5 w-5 border-b-2 border-white"
    />
    <span>{{ waiting ? "Processing..." : "Estimate" }}</span>
  </button>
</template>

<script setup lang="ts">
defineProps<{
  waiting: boolean
}>()

defineEmits(['estimate'])
</script>
