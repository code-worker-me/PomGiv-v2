<script setup>
defineProps({
  direction: {
    type: String,
    default: "up",
    validator: (val) => ["up", "down"].includes(val),
  },
});

defineEmits(["click"]);
</script>

<template>
  <button 
    type="button"
    @click="$emit('click')"
    class="flex justify-center outline-none active:outline-none hover:text-white py-1 transition-colors w-full cursor-pointer focus:outline-none"
  >
    <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <path :d="direction === 'up' ? 'm18 15-6-6-6 6' : 'm6 9 6 6 6-6'" />
    </svg>
  </button>
</template>
