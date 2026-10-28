<script setup>
defineProps({
  type: {
    type: String,
      default: 'button'
  },
  title: {
    type: String,
      default: "isi konten berikut"
    }
})

defineEmits(['click'])
</script>

<template>
<button
    :type="type"
    :title="title"
    @click="$emit('click', $event)"
    class="p-2.5 rounded-lg text-neutral-400 hover:text-white hover:bg-white/10 active:scale-95 transition-all cursor-pointer"
>
    <slot />
</button>
</template>
