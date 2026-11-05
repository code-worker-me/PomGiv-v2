<script setup>
defineProps({
  timeDisplay: {
    type: String,
    required: true,
  },
  isRunning: {
    type: Boolean,
    default: false,
  },
  showNotificationToast: {
    type: Boolean,
    default: false,
  },
  notificationMinutes: {
    type: Number,
    default: 0,
  },
});

const emit = defineEmits([
  "toggle-timer",
  "reset",
  "toggle-mini-mode",
  "dismiss-notification",
]);
</script>

<template>
  <main
    data-tauri-drag-region
    class="h-screen w-full bg-[#1c1826] text-white flex items-center justify-between px-3.5 select-none font-sans cursor-move overflow-hidden border border-[#2c2438] relative"
  >
    <!-- Overlay Notifikasi di Mode Mini jika aktif -->
    <div
      v-if="showNotificationToast"
      class="absolute inset-0 bg-[#251e33] z-50 flex items-center justify-between px-3 border border-[#d8b4e2]/40"
      data-tauri-drag-region
    >
      <div class="flex items-center gap-2 min-w-0" data-tauri-drag-region>
        <span class="text-xs font-semibold text-emerald-400">Selesai! 🎉</span>
        <span class="text-[11px] text-neutral-300 truncate">{{ notificationMinutes }}m</span>
      </div>
      <button
        type="button"
        @click="emit('dismiss-notification')"
        class="px-2.5 py-1 rounded bg-[#382c4a] hover:bg-[#47385e] active:scale-95 text-xs font-medium text-neutral-200 hover:text-white cursor-pointer shrink-0 transition-colors"
      >
        Dismiss
      </button>
    </div>

    <!-- Left: Pin Status Indicator + Timer Text -->
    <div class="flex items-center gap-2.5" data-tauri-drag-region>
      <div class="w-2.5 h-2.5 rounded-full bg-emerald-400 animate-pulse" title="Always on Top aktif"></div>
      <span class="text-2xl font-light tracking-tight text-neutral-100 tabular-nums">
        {{ timeDisplay }}
      </span>
    </div>

    <!-- Right: Mini Action Buttons -->
    <div class="flex items-center gap-1.5" @mousedown.stop>
      <!-- Play / Pause Button -->
      <button
        type="button"
        @click="emit('toggle-timer')"
        :title="isRunning ? 'Pause' : 'Start'"
        class="w-7.5 h-7.5 rounded-full bg-[#d8b4e2] hover:bg-[#cb9ed6] active:scale-90 transition-all flex items-center justify-center shadow cursor-pointer text-black"
      >
        <!-- Pause Icon -->
        <svg v-if="isRunning" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-3.5 h-3.5">
          <path fill-rule="evenodd" d="M6.75 5.25a.75.75 0 0 1 .75.75v12a.75.75 0 0 1-1.5 0v-12a.75.75 0 0 1 .75-.75Zm10.5 0a.75.75 0 0 1 .75.75v12a.75.75 0 0 1-1.5 0v-12a.75.75 0 0 1 .75-.75Z" clip-rule="evenodd" />
        </svg>
        <!-- Play Icon -->
        <svg v-else xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-3.5 h-3.5 translate-x-0.5">
          <path d="M5 3.868v16.264a1 1 0 0 0 1.545.841l12.443-8.132a1 1 0 0 0 0-1.682L6.545 3.027A1 1 0 0 0 5 3.868z" />
        </svg>
      </button>

      <!-- Reset Button -->
      <button
        type="button"
        @click="emit('reset')"
        title="Reset Timer"
        class="w-7.5 h-7.5 rounded-full bg-[#2c2438] hover:bg-[#382f48] active:scale-90 transition-all flex items-center justify-center text-[#9f94b0] hover:text-white cursor-pointer"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
          <path stroke-linecap="round" stroke-linejoin="round" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
        </svg>
      </button>

      <!-- Return to Normal Mode Button -->
      <button
        type="button"
        @click="emit('toggle-mini-mode')"
        title="Kembali ke Mode Normal"
        class="w-7.5 h-7.5 rounded-full bg-[#2c2438] hover:bg-[#382f48] active:scale-90 transition-all flex items-center justify-center text-[#9f94b0] hover:text-white cursor-pointer"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="15 3 21 3 21 9" />
          <polyline points="9 21 3 21 3 15" />
          <line x1="21" x2="14" y1="3" y2="10" />
          <line x1="3" x2="10" y1="21" y2="14" />
        </svg>
      </button>
    </div>
  </main>
</template>
