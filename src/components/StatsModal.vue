<script setup>
import { ref, computed } from "vue";

const emit = defineEmits(["close"]);

// Data Dummy Hardcoded Bulan Oktober (1 - 31)
const currentMonthName = "Oktober 2026";
const todayDate = 11;

const monthlyData = ref([
  { day: 1, minutes: 45 },
  { day: 2, minutes: 75 },
  { day: 3, minutes: 0 },
  { day: 4, minutes: 25 },
  { day: 5, minutes: 110 },
  { day: 6, minutes: 60 },
  { day: 7, minutes: 0 },
  { day: 8, minutes: 30 },
  { day: 9, minutes: 90 },
  { day: 10, minutes: 135 },
  { day: 11, minutes: 85 }, // Hari ini
  { day: 12, minutes: 0 },
  { day: 13, minutes: 0 },
  { day: 14, minutes: 0 },
  { day: 15, minutes: 0 },
  { day: 16, minutes: 0 },
  { day: 17, minutes: 0 },
  { day: 18, minutes: 0 },
  { day: 19, minutes: 0 },
  { day: 20, minutes: 0 },
  { day: 21, minutes: 0 },
  { day: 22, minutes: 0 },
  { day: 23, minutes: 0 },
  { day: 24, minutes: 0 },
  { day: 25, minutes: 0 },
  { day: 26, minutes: 0 },
  { day: 27, minutes: 0 },
  { day: 28, minutes: 0 },
  { day: 29, minutes: 0 },
  { day: 30, minutes: 0 },
  { day: 31, minutes: 0 },
]);

// Tanggal yang sedang dipilih untuk melihat detail (default hari ini)
const selectedDay = ref(monthlyData.value.find((d) => d.day === todayDate));

// Hitung total menit hari ini
const todayMinutes = computed(() => {
  const item = monthlyData.value.find((d) => d.day === todayDate);
  return item ? item.minutes : 0;
});

// Hitung total menit bulan ini
const totalMonthlyMinutes = computed(() => {
  return monthlyData.value.reduce((acc, curr) => acc + curr.minutes, 0);
});

// Format menit menjadi jam & menit
function formatDuration(minutes) {
  if (!minutes || minutes <= 0) return "0 mnt";
  const h = Math.floor(minutes / 60);
  const m = minutes % 60;
  if (h > 0 && m > 0) return `${h}j ${m}m`;
  if (h > 0) return `${h} jam`;
  return `${m} menit`;
}

// Menentukan tingkat warna gradasi berdasarkan menit (0 - 4)
function getColorLevel(minutes) {
  if (!minutes || minutes === 0) return 0;
  if (minutes <= 30) return 1;
  if (minutes <= 60) return 2;
  if (minutes <= 90) return 3;
  return 4;
}

// Class warna sesuai level heatmap
function getBoxClass(dayItem) {
  const level = getColorLevel(dayItem.minutes);
  const isSelected = selectedDay.value?.day === dayItem.day;
  const isToday = dayItem.day === todayDate;

  const colorClasses = {
    0: "bg-[#282233] text-neutral-400 hover:bg-[#342c43]",
    1: "bg-[#543b6e] text-purple-100 hover:bg-[#634682]",
    2: "bg-[#804ca8] text-purple-50 hover:bg-[#9256c0]",
    3: "bg-[#b370da] text-black font-semibold hover:bg-[#c27fe9]",
    4: "bg-[#d8b4e2] text-black font-bold hover:bg-[#e4c7ed]",
  };

  return [
    colorClasses[level],
    isSelected ? "opacity-100" : "opacity-90 hover:opacity-100",
  ];
}
</script>

<template>
  <div 
    class="fixed inset-0 z-50 bg-black/60 backdrop-blur-[2px] flex items-center justify-center p-3 select-none"
    @click.self="emit('close')"
  >
    <div class="bg-[#1f1f23] border border-neutral-800/80 rounded-xl w-full max-w-[390px] p-5 shadow-2xl flex flex-col gap-4 text-neutral-100 animate-in fade-in zoom-in-95 duration-150">
      <!-- Header Modal -->
      <div class="flex items-center justify-between pb-1 border-b border-neutral-800">
        <div>
          <h2 class="text-base font-semibold tracking-wide text-neutral-100 flex items-center gap-2">
            <!-- Stats Icon -->
            <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 text-[#d8b4e2]" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M3 3v18h18" />
              <path d="M18 17V9" />
              <path d="M13 17V5" />
              <path d="M8 17v-3" />
            </svg>
            Statistik Belajar
          </h2>
          <p class="text-xs text-neutral-400 mt-0.5">{{ currentMonthName }}</p>
        </div>

        <button 
          type="button"
          @click="emit('close')"
          title="Tutup"
          class="text-neutral-400 hover:text-white p-1.5 rounded-md hover:bg-neutral-800 transition-colors cursor-pointer"
        >
          <!-- X Icon -->
          <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- Ringkasan Cepat: Hari Ini & Bulan Ini -->
      <div class="grid grid-cols-2 gap-2.5">
        <div class="bg-[#18181b] border border-neutral-800/80 rounded-lg p-3 flex flex-col">
          <span class="text-[11px] font-medium text-neutral-400 uppercase tracking-wider">Hari Ini</span>
          <span class="text-xl font-bold text-[#d8b4e2] mt-0.5">{{ formatDuration(todayMinutes) }}</span>
          <span class="text-[10px] text-neutral-400 mt-0.5">Tgl {{ todayDate }} Okt</span>
        </div>

        <div class="bg-[#18181b] border border-neutral-800/80 rounded-lg p-3 flex flex-col">
          <span class="text-[11px] font-medium text-neutral-400 uppercase tracking-wider">Bulan Ini</span>
          <span class="text-xl font-bold text-neutral-100 mt-0.5">{{ formatDuration(totalMonthlyMinutes) }}</span>
          <span class="text-[10px] text-neutral-400 mt-0.5">31 hari total</span>
        </div>
      </div>

      <!-- Detail Tanggal yang Dipilih -->
      <div class="bg-[#18181b] border border-neutral-800/80 rounded-lg px-3 py-2 flex items-center justify-between text-xs">
        <span class="text-neutral-300">
          Tanggal <strong class="text-white">{{ selectedDay?.day }} Okt</strong>:
        </span>
        <span class="font-semibold text-[#d8b4e2]">
          {{ selectedDay?.minutes > 0 ? `${selectedDay.minutes} menit (${formatDuration(selectedDay.minutes)})` : 'Tidak ada aktivitas' }}
        </span>
      </div>

      <!-- Kalender Heatmap Grid (~31 Kotak) -->
      <div class="flex flex-col gap-1.5 bg-[#18181b] border border-neutral-800/80 rounded-lg p-3">
        <!-- Label Nama Hari -->
        <div class="grid grid-cols-7 gap-1.5 text-center text-[10px] text-neutral-400 font-medium">
          <span>Min</span>
          <span>Sen</span>
          <span>Sel</span>
          <span>Rab</span>
          <span>Kam</span>
          <span>Jum</span>
          <span>Sab</span>
        </div>

        <!-- Kotak-kotak Heatmap per Tanggal -->
        <div class="grid grid-cols-7 gap-1.5">
          <button
            v-for="item in monthlyData"
            :key="item.day"
            type="button"
            @click="selectedDay = item"
            :title="`Tgl ${item.day}: ${item.minutes} menit`"
            :class="[
              'aspect-square rounded flex outline-none items-center justify-center text-[11px] transition-all cursor-pointer relative',
              getBoxClass(item)
            ]"
          >
            {{ item.day }}
          </button>
        </div>
      </div>

      <!-- Footer Legenda Gradasi -->
      <div class="flex items-center justify-between text-[11px] text-neutral-400 pt-1">
        <span>Aktivitas Belajar</span>
        <div class="flex items-center gap-1.5">
          <span>Kurang</span>
          <div class="w-3 h-3 rounded-xs bg-[#282233]" title="0 mnt"></div>
          <div class="w-3 h-3 rounded-xs bg-[#543b6e]" title="1-30 mnt"></div>
          <div class="w-3 h-3 rounded-xs bg-[#804ca8]" title="31-60 mnt"></div>
          <div class="w-3 h-3 rounded-xs bg-[#b370da]" title="61-90 mnt"></div>
          <div class="w-3 h-3 rounded-xs bg-[#d8b4e2]" title=">90 mnt"></div>
          <span>Banyak</span>
        </div>
      </div>
    </div>
  </div>
</template>
