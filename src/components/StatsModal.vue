<script setup>
import { ref, computed, watch, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const emit = defineEmits(["close"]);

const monthNames = [
  "Januari", "Februari", "Maret", "April", "Mei", "Juni",
  "Juli", "Agustus", "September", "Oktober", "November", "Desember"
];

// State Data Statistik Dinamis dari SQLite
const yearNumber = ref(new Date().getFullYear());
const monthNumber = ref(new Date().getMonth() + 1);
const todayDate = ref(new Date().getDate());
const todayMinutes = ref(0);
const totalMonthlyMinutes = ref(0);
const monthlyData = ref([]);
const selectedDay = ref(null);
const isLoading = ref(true);

// State Activity Summary untuk hari terpilih
const activitySummaries = ref([]);
const isLoadingActivities = ref(false);

const currentMonthName = computed(() => {
  return `${monthNames[monthNumber.value - 1]} ${yearNumber.value}`;
});

const shortMonthName = computed(() => {
  return monthNames[monthNumber.value - 1]?.substring(0, 3) || "";
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

// Format durasi detik untuk Activity Summary: (1h 5m) atau (25m)
function formatActivityDuration(seconds) {
  const totalMins = Math.floor(seconds / 60);
  const h = Math.floor(totalMins / 60);
  const m = totalMins % 60;
  if (h > 0 && m > 0) return `${h}h ${m}m`;
  if (h > 0) return `${h}h`;
  return `${m}m`;
}

// Mengambil data ringkasan aktivitas harian dari SQLite
async function fetchDailyActivities(day) {
  if (!day) return;
  isLoadingActivities.value = true;
  try {
    const list = await invoke("get_daily_activity_summary", {
      year: yearNumber.value,
      month: monthNumber.value,
      day: day,
    });
    activitySummaries.value = list || [];
  } catch (err) {
    console.error("Gagal mengambil ringkasan aktivitas harian:", err);
    activitySummaries.value = [];
  } finally {
    isLoadingActivities.value = false;
  }
}

// Pantau perubahan selectedDay untuk memuat activity summary
watch(selectedDay, (newVal) => {
  if (newVal) {
    fetchDailyActivities(newVal.day);
  }
});

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

  const colorClasses = {
    0: "bg-[#282233] text-neutral-400 hover:bg-[#342c43]",
    1: "bg-[#543b6e] text-purple-100 hover:bg-[#634682]",
    2: "bg-[#804ca8] text-purple-50 hover:bg-[#9256c0]",
    3: "bg-[#b370da] text-black font-semibold hover:bg-[#c27fe9]",
    4: "bg-[#d8b4e2] text-black font-bold hover:bg-[#e4c7ed]",
  };

  return [
    colorClasses[level],
    isSelected ? "opacity-100 scale-105 z-10" : "opacity-90 hover:opacity-100",
  ];
}

// Muat data statistik riil dari database SQLite di Rust
onMounted(async () => {
  try {
    const res = await invoke("get_monthly_study_stats");
    if (res) {
      yearNumber.value = res.year;
      monthNumber.value = res.month;
      todayDate.value = res.today_day;
      todayMinutes.value = res.today_minutes;
      totalMonthlyMinutes.value = res.total_month_minutes;
      monthlyData.value = res.daily_stats;
      const defaultDay = res.daily_stats.find((d) => d.day === res.today_day) || res.daily_stats[0];
      selectedDay.value = defaultDay;
      if (defaultDay) {
        fetchDailyActivities(defaultDay.day);
      }
    }
  } catch (err) {
    console.error("Gagal mengambil statistik belajar dari SQLite:", err);
  } finally {
    isLoading.value = false;
  }
});
</script>

<template>
  <div 
    class="fixed inset-0 z-50 bg-black/60 backdrop-blur-[2px] flex items-center justify-center p-3 select-none"
    @click.self="emit('close')"
  >
    <div class="bg-[#1f1f23] border border-neutral-800/80 rounded-xl w-full max-w-97.5 max-h-[92vh] overflow-y-auto p-5 shadow-2xl flex flex-col gap-3.5 text-neutral-100 animate-in fade-in zoom-in-95 duration-150">
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
          <span class="text-[10px] text-neutral-400 mt-0.5">Tgl {{ todayDate }} {{ shortMonthName }}</span>
        </div>

        <div class="bg-[#18181b] border border-neutral-800/80 rounded-lg p-3 flex flex-col">
          <span class="text-[11px] font-medium text-neutral-400 uppercase tracking-wider">Bulan Ini</span>
          <span class="text-xl font-bold text-neutral-100 mt-0.5">{{ formatDuration(totalMonthlyMinutes) }}</span>
          <span class="text-[10px] text-neutral-400 mt-0.5">{{ monthlyData.length }} hari total</span>
        </div>
      </div>

      <!-- Detail Tanggal yang Dipilih -->
      <div class="bg-[#18181b] border border-neutral-800/80 rounded-lg px-3 py-2 flex items-center justify-between text-xs">
        <span class="text-neutral-300">
          Tanggal <strong class="text-white">{{ selectedDay?.day }} {{ shortMonthName }}</strong>:
        </span>
        <span class="font-semibold text-[#d8b4e2]">
          {{ selectedDay?.minutes > 0 ? `${selectedDay.minutes} menit (${formatDuration(selectedDay.minutes)})` : 'Tidak ada aktivitas' }}
        </span>
      </div>

      <!-- Activity Summary (Daily Log) untuk Hari yang Dipilih -->
      <div class="bg-[#18181b] border border-neutral-800/80 rounded-lg p-3 flex flex-col gap-2">
        <div class="flex items-center justify-between border-b border-neutral-800/60 pb-1.5">
          <span class="text-[11px] font-semibold text-neutral-300 uppercase tracking-wider flex items-center gap-1.5">
            <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5 text-[#d8b4e2]" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
              <polyline points="14 2 14 8 20 8" />
              <line x1="16" y1="13" x2="8" y2="13" />
              <line x1="16" y1="17" x2="8" y2="17" />
              <polyline points="10 9 9 9 8 9" />
            </svg>
            Activity Summary
          </span>
          <span class="text-[10px] text-neutral-400">Tgl {{ selectedDay?.day }} {{ shortMonthName }}</span>
        </div>

        <div v-if="isLoadingActivities" class="text-xs text-neutral-500 py-1 italic">
          Memuat aktivitas...
        </div>

        <div v-else-if="activitySummaries.length > 0" class="flex flex-col gap-1 max-h-28 overflow-y-auto pr-1">
          <div
            v-for="(item, idx) in activitySummaries"
            :key="idx"
            class="text-xs text-neutral-200 flex items-start gap-1.5 py-0.5"
          >
            <span class="text-[#d8b4e2] font-semibold shrink-0">- ({{ formatActivityDuration(item.duration_seconds) }})</span>
            <span class="break-words">{{ item.title }}</span>
          </div>
        </div>

        <div v-else class="text-[11px] text-neutral-500 italic py-0.5">
          Tidak ada catatan aktivitas khusus pada tanggal ini.
        </div>
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
