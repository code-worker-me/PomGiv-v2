<script setup>
import { ref, computed } from "vue";
import EditTimerModal from "./components/EditTimerModal.vue";

// State Timer Utama
const hours = ref(0);
const minutes = ref(25);
const seconds = ref(0);
const isRunning = ref(false);

// Format waktu 00:00:00
const timeDisplay = computed(() => {
  const pad = (n) => String(n).padStart(2, "0");
  return `${pad(hours.value)}:${pad(minutes.value)}:${pad(seconds.value)}`;
});

// State Modal Edit Timer
const showEditModal = ref(false);

function handleSaveTimer(newTime) {
  hours.value = newTime.hours;
  minutes.value = newTime.minutes;
  seconds.value = newTime.seconds;
  showEditModal.value = false;
}
</script>

<template>
  <main class="min-h-screen w-full bg-[#1c1826] text-white flex flex-col justify-between items-center p-6 select-none font-sans relative">
    <!-- Top Action Icon -->
    <header class="w-full flex justify-end">
      <!-- Tombol Statistik Belajar -->
      <button 
        type="button"
        title="Statistik Belajar"
        class="p-2.5 rounded-lg text-neutral-400 hover:text-white hover:bg-white/10 active:scale-95 transition-all"
      >
        <!-- Bar Chart / Statistics Icon -->
        <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M3 3v18h18" />
          <path d="M18 17V9" />
          <path d="M13 17V5" />
          <path d="M8 17v-3" />
        </svg>
      </button>
    </header>

    <!-- Center Section: Circular Timer Ring (Klik untuk edit) -->
    <section class="flex flex-col items-center justify-center my-auto">
      <div 
        @click="showEditModal = true"
        title="Klik untuk mengubah timer"
        class="group relative w-72 h-72 sm:w-84 sm:h-84 md:w-96 md:h-96 rounded-full border-[6px] border-[#2c2438] hover:border-[#433657] flex flex-col items-center justify-center cursor-pointer transition-colors"
      >
        <!-- Timer Text -->
        <span class="text-6xl sm:text-7xl font-light tracking-tight text-neutral-100 tabular-nums group-hover:scale-105 transition-transform">
          {{ timeDisplay }}
        </span>
      </div>
    </section>

    <!-- Bottom Controls -->
    <footer class="flex items-center justify-center gap-5 pb-6">
      <!-- Play / Pause Button -->
      <button
        type="button"
        aria-label="Start Timer"
        class="w-16 h-16 rounded-full bg-[#d8b4e2] hover:bg-[#cb9ed6] active:scale-95 transition-all flex items-center justify-center shadow-lg"
      >
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-7 h-7 text-black translate-x-0.5">
          <path d="M5 3.868v16.264a1 1 0 0 0 1.545.841l12.443-8.132a1 1 0 0 0 0-1.682L6.545 3.027A1 1 0 0 0 5 3.868z" />
        </svg>
      </button>

      <!-- Reset / Restart Button -->
      <button
        type="button"
        aria-label="Reset Timer"
        class="w-16 h-16 rounded-full bg-[#2c2438] hover:bg-[#382f48] active:scale-95 transition-all flex items-center justify-center text-[#9f94b0] hover:text-white"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
          <path stroke-linecap="round" stroke-linejoin="round" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
        </svg>
      </button>
    </footer>

    <!-- MODAL EDIT TIMER (Component) -->
    <EditTimerModal
      v-if="showEditModal"
      :initial-hours="hours"
      :initial-minutes="minutes"
      :initial-seconds="seconds"
      @save="handleSaveTimer"
      @close="showEditModal = false"
    />
  </main>
</template>



