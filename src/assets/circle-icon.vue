<script setup>
defineProps({
  circleRadius: {
    type: Number,
    required: true
  },
  circleCircumference: {
    type: Number,
    required: true
  },
  progressOffset: {
    type: Number,
    required: true
  }
})
</script>

<template>
  <svg class="absolute inset-0 w-full h-full -rotate-90 pointer-events-none" viewBox="0 0 200 200">
    <!-- Background Track Circle -->
    <circle
      cx="100"
      cy="100"
      :r="circleRadius"
      fill="transparent"
      stroke="#2c2438"
      stroke-width="6"
      class="group-hover:stroke-[#433657] transition-colors"
    />
    <!-- Foreground Animated Progress Circle -->
    <circle
      cx="100"
      cy="100"
      :r="circleRadius"
      fill="transparent"
      stroke="#d8b4e2"
      stroke-width="6"
      stroke-linecap="round"
      :stroke-dasharray="circleCircumference"
      :stroke-dashoffset="progressOffset"
      class="transition-[stroke-dashoffset] duration-1000 ease-linear"
    />
  </svg>
</template>
