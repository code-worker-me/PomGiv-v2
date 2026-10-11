<script setup>
import { ref, onMounted, onUnmounted } from "vue";
import TimeAdjustButton from "./TimeAdjustButton.vue";

const props = defineProps({
  initialHours: {
    type: Number,
    default: 0,
  },
  initialMinutes: {
    type: Number,
    default: 25,
  },
  initialSeconds: {
    type: Number,
    default: 0,
  },
});

const emit = defineEmits(["save", "close"]);

const activeUnit = ref("hours"); // 'hours' | 'minutes' | 'seconds'
const tempHours = ref(props.initialHours);
const tempMinutes = ref(props.initialMinutes);
const tempSeconds = ref(props.initialSeconds);

// Flag untuk pengetikan digit pertama pada unit aktif
const isFirstDigit = ref(true);

function selectUnit(unit) {
  activeUnit.value = unit;
  isFirstDigit.value = true;
}

function adjustUnit(unit, delta) {
  isFirstDigit.value = true;
  if (unit === "hours") {
    tempHours.value = Math.max(0, Math.min(99, tempHours.value + delta));
  } else if (unit === "minutes") {
    tempMinutes.value = (tempMinutes.value + delta + 60) % 60;
  } else if (unit === "seconds") {
    tempSeconds.value = (tempSeconds.value + delta + 60) % 60;
  }
}

function resetTempTimer() {
  tempHours.value = 0;
  tempMinutes.value = 0;
  tempSeconds.value = 0;
  isFirstDigit.value = true;
}

function pad(val) {
  return String(val).padStart(2, "0");
}

function handleSave() {
  emit("save", {
    hours: tempHours.value,
    minutes: tempMinutes.value,
    seconds: tempSeconds.value,
  });
}

function handleClose() {
  emit("close");
}

// Handler input keyboard
function handleKeyDown(e) {
  // 1. Input Angka 0-9
  if (/^[0-9]$/.test(e.key)) {
    e.preventDefault();
    const digit = parseInt(e.key, 10);

    if (activeUnit.value === "hours") {
      if (isFirstDigit.value) {
        tempHours.value = digit;
        isFirstDigit.value = false;
      } else {
        tempHours.value = Math.min(99, tempHours.value * 10 + digit);
        activeUnit.value = "minutes";
        isFirstDigit.value = true;
      }
    } else if (activeUnit.value === "minutes") {
      if (isFirstDigit.value) {
        tempMinutes.value = digit;
        isFirstDigit.value = false;
      } else {
        tempMinutes.value = Math.min(59, tempMinutes.value * 10 + digit);
        activeUnit.value = "seconds";
        isFirstDigit.value = true;
      }
    } else if (activeUnit.value === "seconds") {
      if (isFirstDigit.value) {
        tempSeconds.value = digit;
        isFirstDigit.value = false;
      } else {
        tempSeconds.value = Math.min(59, tempSeconds.value * 10 + digit);
        isFirstDigit.value = true;
      }
    }
    return;
  }

  // 2. Navigasi Antar Kolom (Panah Kiri / Kanan atau Tab)
  if (e.key === "ArrowLeft") {
    e.preventDefault();
    if (activeUnit.value === "seconds") selectUnit("minutes");
    else if (activeUnit.value === "minutes") selectUnit("hours");
  } else if (e.key === "ArrowRight") {
    e.preventDefault();
    if (activeUnit.value === "hours") selectUnit("minutes");
    else if (activeUnit.value === "minutes") selectUnit("seconds");
  } else if (e.key === "Tab") {
    e.preventDefault();
    if (e.shiftKey) {
      if (activeUnit.value === "seconds") selectUnit("minutes");
      else if (activeUnit.value === "minutes") selectUnit("hours");
    } else {
      if (activeUnit.value === "hours") selectUnit("minutes");
      else if (activeUnit.value === "minutes") selectUnit("seconds");
    }
  } 
  // 3. Tambah / Kurang Waktu (Panah Atas / Bawah)
  else if (e.key === "ArrowUp") {
    e.preventDefault();
    adjustUnit(activeUnit.value, 1);
  } else if (e.key === "ArrowDown") {
    e.preventDefault();
    adjustUnit(activeUnit.value, -1);
  } 
  // 4. Backspace (Reset kolom aktif ke 0)
  else if (e.key === "Backspace") {
    e.preventDefault();
    if (activeUnit.value === "hours") tempHours.value = 0;
    else if (activeUnit.value === "minutes") tempMinutes.value = 0;
    else if (activeUnit.value === "seconds") tempSeconds.value = 0;
    isFirstDigit.value = true;
  } 
  // 5. Enter (Simpan) dan Escape (Tutup)
  else if (e.key === "Enter") {
    e.preventDefault();
    handleSave();
  } else if (e.key === "Escape") {
    e.preventDefault();
    handleClose();
  }
}

onMounted(() => {
  window.addEventListener("keydown", handleKeyDown);
});

onUnmounted(() => {
  window.removeEventListener("keydown", handleKeyDown);
});
</script>

<template>
  <div 
    class="fixed inset-0 z-50 bg-black/60 backdrop-blur-[2px] flex items-center justify-center p-4 select-none"
    @click.self="handleClose"
  >
    <div class="bg-[#1f1f22] border border-neutral-800/80 rounded-xl w-full max-w-sm p-6 shadow-2xl flex flex-col gap-6 text-neutral-100 animate-in fade-in zoom-in-95 duration-150">
      <!-- Header Modal -->
      <div class="flex items-center justify-between">
        <h2 class="text-lg font-semibold tracking-wide text-neutral-100">Edit timer</h2>
        <button 
          type="button"
          @click="resetTempTimer"
          title="Reset timer ke 00:00:00"
          class="text-rose-400 hover:text-rose-300 p-1 rounded-md hover:bg-rose-500/10 transition-colors cursor-pointer"
        >
          <!-- Trash Icon -->
          <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.8">
            <path stroke-linecap="round" stroke-linejoin="round" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
          </svg>
        </button>
      </div>

      <!-- Time Picker Columns (HH : MM : SS) -->
      <div class="flex flex-col items-center gap-1.5">
        <!-- Chevron Up Buttons (Menggunakan TimeAdjustButton) -->
        <div class="grid grid-cols-[1fr_auto_1fr_auto_1fr] items-center w-full text-neutral-400">
          <TimeAdjustButton direction="up" @click="adjustUnit('hours', 1)" />
          <div class="w-4"></div>
          <TimeAdjustButton direction="up" @click="adjustUnit('minutes', 1)" />
          <div class="w-4"></div>
          <TimeAdjustButton direction="up" @click="adjustUnit('seconds', 1)" />
        </div>

        <!-- Time Display Box with Active Column Highlighting -->
        <div class="w-full bg-[#18181a] border border-neutral-800 rounded-lg grid grid-cols-[1fr_auto_1fr_auto_1fr] items-center overflow-hidden">
          <!-- Hours Column -->
          <button
            type="button"
            @click="selectUnit('hours')"
            :class="[
              'py-4 text-3xl font-bold text-center transition-colors relative cursor-pointer',
              activeUnit === 'hours' ? 'bg-[#29292e] text-white' : 'text-neutral-400 hover:text-neutral-200'
            ]"
          >
            {{ pad(tempHours) }}
            <div v-if="activeUnit === 'hours'" class="absolute bottom-0 left-0 right-0 h-0.5 bg-[#d8b4e2]"></div>
          </button>

          <!-- Colon Separator -->
          <span class="text-2xl font-bold text-neutral-400 select-none px-0.5">:</span>

          <!-- Minutes Column -->
          <button
            type="button"
            @click="selectUnit('minutes')"
            :class="[
              'py-4 text-3xl font-bold text-center transition-colors relative cursor-pointer',
              activeUnit === 'minutes' ? 'bg-[#29292e] text-white' : 'text-neutral-400 hover:text-neutral-200'
            ]"
          >
            {{ pad(tempMinutes) }}
            <div v-if="activeUnit === 'minutes'" class="absolute bottom-0 left-0 right-0 h-0.5 bg-[#d8b4e2]"></div>
          </button>

          <!-- Colon Separator -->
          <span class="text-2xl font-bold text-neutral-400 select-none px-0.5">:</span>

          <!-- Seconds Column -->
          <button
            type="button"
            @click="selectUnit('seconds')"
            :class="[
              'py-4 text-3xl font-bold text-center transition-colors relative cursor-pointer',
              activeUnit === 'seconds' ? 'bg-[#29292e] text-white' : 'text-neutral-400 hover:text-neutral-200'
            ]"
          >
            {{ pad(tempSeconds) }}
            <div v-if="activeUnit === 'seconds'" class="absolute bottom-0 left-0 right-0 h-0.5 bg-[#d8b4e2]"></div>
          </button>
        </div>

        <!-- Chevron Down Buttons (Menggunakan TimeAdjustButton) -->
        <div class="grid grid-cols-[1fr_auto_1fr_auto_1fr] items-center w-full text-neutral-400">
          <TimeAdjustButton direction="down" @click="adjustUnit('hours', -1)" />
          <div class="w-4"></div>
          <TimeAdjustButton direction="down" @click="adjustUnit('minutes', -1)" />
          <div class="w-4"></div>
          <TimeAdjustButton direction="down" @click="adjustUnit('seconds', -1)" />
        </div>
      </div>

      <!-- Footer Action Buttons -->
      <div class="grid grid-cols-2 gap-3 pt-2">
        <!-- Save Button -->
        <button
          type="button"
          @click="handleSave"
          class="flex items-center justify-center gap-2 py-2.5 px-4 rounded-md bg-[#d8b4e2] text-black font-medium text-sm hover:bg-[#cba0d6] active:scale-98 transition-all shadow-md cursor-pointer"
        >
          <!-- Save / Floppy Disk Icon -->
          <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
            <path stroke-linecap="round" stroke-linejoin="round" d="M8 7H5a2 2 0 00-2 2v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-3m-1 4l-3 3m0 0l-3-3m3 3V4" />
          </svg>
          Save
        </button>

        <!-- Cancel Button -->
        <button
          type="button"
          @click="handleClose"
          class="flex items-center justify-center gap-2 py-2.5 px-4 rounded-md bg-[#29292e] text-neutral-200 font-medium text-sm hover:bg-[#34343b] active:scale-98 transition-all cursor-pointer"
        >
          <!-- Cancel / X Icon -->
          <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
          </svg>
          Cancel
        </button>
      </div>
    </div>
  </div>
</template>

