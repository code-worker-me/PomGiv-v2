<script setup>
import { ref, computed, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import EditTimerModal from "./components/EditTimerModal.vue";
import StatsModal from "./components/StatsModal.vue";
import { playChimeSound, stopChimeSound } from "./utils/audio.js";

// State Timer Utama (default 25 menit)
const hours = ref(0);
const minutes = ref(25);
const seconds = ref(0);

// State Waktu Awal Tersimpan (untuk reset)
const savedHours = ref(0);
const savedMinutes = ref(25);
const savedSeconds = ref(0);

const isRunning = ref(false);
let timerInterval = null;

// Format waktu 00:00:00
const timeDisplay = computed(() => {
  const pad = (n) => String(n).padStart(2, "0");
  return `${pad(hours.value)}:${pad(minutes.value)}:${pad(seconds.value)}`;
});

// Perhitungan Detik & Animasi SVG Circular Progress Ring
const currentTotalSeconds = computed(() => hours.value * 3600 + minutes.value * 60 + seconds.value);
const baseTotalSeconds = computed(() => {
  const base = savedHours.value * 3600 + savedMinutes.value * 60 + savedSeconds.value;
  return base > 0 ? base : 1500;
});

const circleRadius = 88;
const circleCircumference = 2 * Math.PI * circleRadius; // ~552.92

const progressOffset = computed(() => {
  if (baseTotalSeconds.value <= 0) return 0;
  const ratio = currentTotalSeconds.value / baseTotalSeconds.value;
  return circleCircumference * (1 - Math.max(0, Math.min(1, ratio)));
});

// State Modal Edit Timer & Statistik
const showEditModal = ref(false);
const showStatsModal = ref(false);

// State Mode Mini / Floating Window (Always on Top)
const isMiniMode = ref(false);

async function toggleMiniMode() {
  try {
    const nextState = !isMiniMode.value;
    await invoke("set_mini_mode", { enabled: nextState });
    isMiniMode.value = nextState;
  } catch (err) {
    console.error("Gagal mengubah mode mini:", err);
  }
}

// Ambil data timer terakhir dari database SQLite saat aplikasi terbuka
onMounted(async () => {
  try {
    const data = await invoke("get_timer_settings");
    if (data) {
      hours.value = data.hours;
      minutes.value = data.minutes;
      seconds.value = data.seconds;
      savedHours.value = data.hours;
      savedMinutes.value = data.minutes;
      savedSeconds.value = data.seconds;
    }
  } catch (err) {
    console.error("Gagal mengambil data timer dari SQLite:", err);
  }
});

// Bersihkan interval saat komponen dilepas
onUnmounted(() => {
  if (timerInterval) {
    clearInterval(timerInterval);
  }
});

// State Waktu Sesi Berjalan (dalam detik)
const currentSessionDurationSeconds = ref(0);

// State Notifikasi Toast In-App dengan Tombol Dismiss
const showNotificationToast = ref(false);
const notificationMinutes = ref(0);
let notificationTimeout = null;

function dismissNotification() {
  showNotificationToast.value = false;
  if (notificationTimeout) {
    clearTimeout(notificationTimeout);
    notificationTimeout = null;
  }
  stopChimeSound();
}

// Fungsi Ketika Waktu Timer Habis
async function handleTimerCompleted() {
  clearInterval(timerInterval);
  timerInterval = null;
  isRunning.value = false;

  // Bunyikan dering lonceng / chime berprioritas tinggi & audio ducking
  playChimeSound();

  const duration = currentSessionDurationSeconds.value;
  currentSessionDurationSeconds.value = 0;

  // Validasi ketat: HANYA simpan ke database dan munculkan notifikasi jika durasi LEBIH DARI 1 MENIT (> 60 detik)
  if (duration > 60) {
    const mins = Math.floor(duration / 60);
    notificationMinutes.value = mins;
    showNotificationToast.value = true;

    // Auto-dismiss toast setelah 12 detik jika tidak ditekan manual
    if (notificationTimeout) clearTimeout(notificationTimeout);
    notificationTimeout = setTimeout(() => {
      showNotificationToast.value = false;
      notificationTimeout = null;
    }, 12000);

    try {
      await invoke("complete_study_session", { durationSeconds: duration });
    } catch (err) {
      console.error("Gagal mencatat sesi belajar dan notifikasi:", err);
    }
  }
}


// Fungsi Play / Pause Countdown Timer
function toggleTimer() {
  if (isRunning.value) {
    // Jeda timer (Pause)
    clearInterval(timerInterval);
    timerInterval = null;
    isRunning.value = false;
  } else {
    // Mulai timer (Play)
    // Jika waktu saat ini 00:00:00, kembalikan ke waktu awal tersimpan
    if (hours.value === 0 && minutes.value === 0 && seconds.value === 0) {
      hours.value = savedHours.value;
      minutes.value = savedMinutes.value;
      seconds.value = savedSeconds.value;
    }

    // Jika waktu masih 00:00:00, jangan jalankan
    if (hours.value === 0 && minutes.value === 0 && seconds.value === 0) {
      return;
    }

    // Catat durasi awal sesi (jika belum tercatat dari resume pause)
    if (!currentSessionDurationSeconds.value || currentSessionDurationSeconds.value === 0) {
      currentSessionDurationSeconds.value = hours.value * 3600 + minutes.value * 60 + seconds.value;
    }

    isRunning.value = true;
    timerInterval = setInterval(() => {
      if (seconds.value > 0) {
        seconds.value--;
      } else if (minutes.value > 0) {
        minutes.value--;
        seconds.value = 59;
      } else if (hours.value > 0) {
        hours.value--;
        minutes.value = 59;
        seconds.value = 59;
      } else {
        // Waktu selesai
        handleTimerCompleted();
      }
    }, 1000);
  }
}

// Fungsi Reset Timer ke waktu awal
function handleReset() {
  if (timerInterval) {
    clearInterval(timerInterval);
    timerInterval = null;
  }
  isRunning.value = false;
  currentSessionDurationSeconds.value = 0;
  hours.value = savedHours.value;
  minutes.value = savedMinutes.value;
  seconds.value = savedSeconds.value;
}

// Simpan timer ke database SQLite di Rust saat user klik Save
async function handleSaveTimer(newTime) {
  // Hentikan timer jika sedang berjalan
  if (timerInterval) {
    clearInterval(timerInterval);
    timerInterval = null;
  }
  isRunning.value = false;
  currentSessionDurationSeconds.value = 0;


  hours.value = newTime.hours;
  minutes.value = newTime.minutes;
  seconds.value = newTime.seconds;
  savedHours.value = newTime.hours;
  savedMinutes.value = newTime.minutes;
  savedSeconds.value = newTime.seconds;
  showEditModal.value = false;

  try {
    await invoke("save_timer_settings", {
      hours: newTime.hours,
      minutes: newTime.minutes,
      seconds: newTime.seconds,
    });
  } catch (err) {
    console.error("Gagal menyimpan data timer ke SQLite:", err);
  }
}
</script>

<template>
  <!-- Mini Mode Layout (Always on Top) -->
  <main 
    v-if="isMiniMode" 
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
        @click="dismissNotification"
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
        @click="toggleTimer"
        :title="isRunning ? 'Pause' : 'Start'"
        class="w-7.5 h-7.5 rounded-full bg-[#d8b4e2] hover:bg-[#cb9ed6] active:scale-90 transition-all flex items-center justify-center shadow cursor-pointer text-black"
      >
        <svg v-if="isRunning" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-4 h-4">
          <path fill-rule="evenodd" d="M6.75 5.25a.75.75 0 0 1 .75.75v12a.75.75 0 0 1-1.5 0v-12a.75.75 0 0 1 .75-.75Zm10.5 0a.75.75 0 0 1 .75.75v12a.75.75 0 0 1-1.5 0v-12a.75.75 0 0 1 .75-.75Z" clip-rule="evenodd" />
        </svg>
        <svg v-else xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-4 h-4 translate-x-0.5">
          <path d="M5 3.868v16.264a1 1 0 0 0 1.545.841l12.443-8.132a1 1 0 0 0 0-1.682L6.545 3.027A1 1 0 0 0 5 3.868z" />
        </svg>
      </button>

      <!-- Reset Button -->
      <button
        type="button"
        @click="handleReset"
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
        @click="toggleMiniMode"
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

  <!-- Normal Mode Layout -->
  <main v-else class="min-h-screen w-full bg-[#1c1826] text-white flex flex-col justify-between items-center p-6 select-none font-sans relative">
    <!-- In-App Notification Toast (dengan Tombol Dismiss) -->
    <transition
      enter-active-class="transition duration-300 ease-out"
      enter-from-class="opacity-0 -translate-y-4 scale-95"
      enter-to-class="opacity-100 translate-y-0 scale-100"
      leave-active-class="transition duration-200 ease-in"
      leave-from-class="opacity-100 translate-y-0 scale-100"
      leave-to-class="opacity-0 -translate-y-2 scale-95"
    >
      <div 
        v-if="showNotificationToast" 
        class="absolute top-4 left-4 right-4 z-50 bg-[#251e33] border border-[#d8b4e2]/30 shadow-2xl rounded-xl p-3 flex items-center justify-between gap-3 text-white backdrop-blur-md"
      >
        <div class="flex items-center gap-3 min-w-0">
          <div class="w-8 h-8 rounded-full bg-[#d8b4e2]/20 text-[#d8b4e2] flex items-center justify-center shrink-0">
            <!-- Bell Icon -->
            <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M6 8a6 6 0 0 1 12 0c0 7 3 9 3 9H3s3-2 3-9" />
              <path d="M10.3 21a1.94 1.94 0 0 0 3.4 0" />
            </svg>
          </div>
          <div class="flex flex-col min-w-0">
            <span class="text-xs font-semibold text-neutral-100 leading-tight truncate">Waktu Selesai! 🎉</span>
            <span class="text-[11px] text-[#b3a8c5] leading-tight truncate">Sesi belajar {{ notificationMinutes }} menit selesai</span>
          </div>
        </div>

        <!-- Tombol Dismiss -->
        <button
          type="button"
          @click="dismissNotification"
          class="shrink-0 px-3 py-1.5 rounded-lg bg-[#382c4a] hover:bg-[#47385e] active:scale-95 text-xs font-medium text-neutral-200 hover:text-white border border-white/5 transition-all cursor-pointer flex items-center gap-1.5"
        >
          <span>Dismiss</span>
          <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5 text-neutral-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="18" y1="6" x2="6" y2="18" />
            <line x1="6" y1="6" x2="18" y2="18" />
          </svg>
        </button>
      </div>
    </transition>
    <!-- Top Action Icons -->
    <header class="w-full flex justify-end gap-1">
      <!-- Tombol Mode Mini / Floating (Always on Top) -->
      <button 
        type="button"
        @click="toggleMiniMode"
        title="Mode Mini (Always on Top)"
        class="p-2.5 rounded-lg text-neutral-400 hover:text-white hover:bg-white/10 active:scale-95 transition-all cursor-pointer"
      >
        <!-- Picture in Picture / Float Icon -->
        <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <rect width="18" height="18" x="3" y="3" rx="2" />
          <rect width="7" height="6" x="12" y="13" rx="1" fill="currentColor" fill-opacity="0.2" />
        </svg>
      </button>

      <!-- Tombol Statistik Belajar -->
      <button 
        type="button"
        @click="showStatsModal = true"
        title="Statistik Belajar"
        class="p-2.5 rounded-lg text-neutral-400 hover:text-white hover:bg-white/10 active:scale-95 transition-all cursor-pointer"
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
        class="group relative w-72 h-72 sm:w-84 sm:h-84 md:w-96 md:h-96 flex flex-col items-center justify-center cursor-pointer select-none"
      >
        <!-- SVG Circular Progress Ring -->
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

        <!-- Timer Text -->
        <span class="relative z-10 text-6xl sm:text-7xl font-light tracking-tight text-neutral-100 tabular-nums group-hover:scale-105 transition-transform">
          {{ timeDisplay }}
        </span>
      </div>
    </section>

    <!-- Bottom Controls -->
    <footer class="flex items-center justify-center gap-5 pb-6">
      <!-- Play / Pause Button -->
      <button
        type="button"
        @click="toggleTimer"
        :aria-label="isRunning ? 'Pause Timer' : 'Start Timer'"
        class="w-16 h-16 rounded-full bg-[#d8b4e2] hover:bg-[#cb9ed6] active:scale-95 transition-all flex items-center justify-center shadow-lg cursor-pointer"
      >
        <!-- Pause Icon (Saat timer aktif) -->
        <svg v-if="isRunning" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-7 h-7 text-black">
          <path fill-rule="evenodd" d="M6.75 5.25a.75.75 0 0 1 .75.75v12a.75.75 0 0 1-1.5 0v-12a.75.75 0 0 1 .75-.75Zm10.5 0a.75.75 0 0 1 .75.75v12a.75.75 0 0 1-1.5 0v-12a.75.75 0 0 1 .75-.75Z" clip-rule="evenodd" />
        </svg>
        <!-- Play Icon (Saat timer berhenti / pause) -->
        <svg v-else xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-7 h-7 text-black translate-x-0.5">
          <path d="M5 3.868v16.264a1 1 0 0 0 1.545.841l12.443-8.132a1 1 0 0 0 0-1.682L6.545 3.027A1 1 0 0 0 5 3.868z" />
        </svg>
      </button>

      <!-- Reset / Restart Button -->
      <button
        type="button"
        @click="handleReset"
        aria-label="Reset Timer"
        class="w-16 h-16 rounded-full bg-[#2c2438] hover:bg-[#382f48] active:scale-95 transition-all flex items-center justify-center text-[#9f94b0] hover:text-white cursor-pointer"
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

    <!-- MODAL STATISTIK BELAJAR (Component) -->
    <StatsModal
      v-if="showStatsModal"
      @close="showStatsModal = false"
    />
  </main>
</template>




