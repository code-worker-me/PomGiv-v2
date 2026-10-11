<script setup>
import { ref } from "vue";

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

function adjustUnit(unit, delta) {
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
          class="text-rose-400 hover:text-rose-300 p-1 rounded-md hover:bg-rose-500/10 transition-colors"
        >
          <!-- Trash Icon -->
          <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.8">
            <path stroke-linecap="round" stroke-linejoin="round" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
          </svg>
        </button>
      </div>

      <!-- Time Picker Columns (HH : MM : SS) -->
      <div class="flex flex-col items-center gap-1.5">
        <!-- Chevron Up Buttons -->
        <div class="grid grid-cols-[1fr_auto_1fr_auto_1fr] items-center w-full text-neutral-400">
          <button 
            type="button"
            @click="adjustUnit('hours', 1)" 
            class="flex justify-center hover:text-white py-1 transition-colors"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="m18 15-6-6-6 6"/></svg>
          </button>
          <div class="w-4"></div>
          <button 
            type="button"
            @click="adjustUnit('minutes', 1)" 
            class="flex justify-center hover:text-white py-1 transition-colors"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="m18 15-6-6-6 6"/></svg>
          </button>
          <div class="w-4"></div>
          <button 
            type="button"
            @click="adjustUnit('seconds', 1)" 
            class="flex justify-center hover:text-white py-1 transition-colors"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="m18 15-6-6-6 6"/></svg>
          </button>
        </div>

        <!-- Time Display Box with Active Column Highlighting -->
        <div class="w-full bg-[#18181a] border border-neutral-800 rounded-lg grid grid-cols-[1fr_auto_1fr_auto_1fr] items-center overflow-hidden">
          <!-- Hours Column -->
          <button
            type="button"
            @click="activeUnit = 'hours'"
            :class="[
              'py-4 text-3xl font-bold text-center transition-colors relative',
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
            @click="activeUnit = 'minutes'"
            :class="[
              'py-4 text-3xl font-bold text-center transition-colors relative',
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
            @click="activeUnit = 'seconds'"
            :class="[
              'py-4 text-3xl font-bold text-center transition-colors relative',
              activeUnit === 'seconds' ? 'bg-[#29292e] text-white' : 'text-neutral-400 hover:text-neutral-200'
            ]"
          >
            {{ pad(tempSeconds) }}
            <div v-if="activeUnit === 'seconds'" class="absolute bottom-0 left-0 right-0 h-0.5 bg-[#d8b4e2]"></div>
          </button>
        </div>

        <!-- Chevron Down Buttons -->
        <div class="grid grid-cols-[1fr_auto_1fr_auto_1fr] items-center w-full text-neutral-400">
          <button 
            type="button"
            @click="adjustUnit('hours', -1)" 
            class="flex justify-center hover:text-white py-1 transition-colors"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="m6 9 6 6 6-6"/></svg>
          </button>
          <div class="w-4"></div>
          <button 
            type="button"
            @click="adjustUnit('minutes', -1)" 
            class="flex justify-center hover:text-white py-1 transition-colors"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="m6 9 6 6 6-6"/></svg>
          </button>
          <div class="w-4"></div>
          <button 
            type="button"
            @click="adjustUnit('seconds', -1)" 
            class="flex justify-center hover:text-white py-1 transition-colors"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="m6 9 6 6 6-6"/></svg>
          </button>
        </div>
      </div>

      <!-- Footer Action Buttons -->
      <div class="grid grid-cols-2 gap-3 pt-2">
        <!-- Save Button -->
        <button
          type="button"
          @click="handleSave"
          class="flex items-center justify-center gap-2 py-2.5 px-4 rounded-md bg-[#d8b4e2] text-black font-medium text-sm hover:bg-[#cba0d6] active:scale-98 transition-all shadow-md"
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
          class="flex items-center justify-center gap-2 py-2.5 px-4 rounded-md bg-[#29292e] text-neutral-200 font-medium text-sm hover:bg-[#34343b] active:scale-98 transition-all"
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
