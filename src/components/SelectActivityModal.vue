<script setup>
const props = defineProps({
  activities: {
    type: Array,
    required: true,
  },
  durationSeconds: {
    type: Number,
    required: true,
  },
});

const emit = defineEmits(["select", "skip", "close"]);

function formatMinutes(seconds) {
  const mins = Math.floor(seconds / 60);
  const h = Math.floor(mins / 60);
  const m = mins % 60;
  if (h > 0 && m > 0) return `${h}j ${m}m`;
  if (h > 0) return `${h} jam`;
  return `${m} menit`;
}
</script>

<template>
  <div 
    class="fixed inset-0 z-50 bg-black/70 backdrop-blur-[2px] flex items-center justify-center p-4 select-none"
    @click.self="emit('close')"
  >
    <div class="bg-[#1f1f23] border border-neutral-800 rounded-xl w-full max-w-sm p-5 shadow-2xl flex flex-col gap-4 text-neutral-100 animate-in fade-in zoom-in-95 duration-150">
      <!-- Header Modal -->
      <div class="flex items-center justify-between pb-2 border-b border-neutral-800">
        <div>
          <h2 class="text-base font-semibold text-neutral-100 flex items-center gap-2">
            <!-- Check Circle Icon -->
            <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 text-[#d8b4e2]" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" />
              <polyline points="22 4 12 14.01 9 11.01" />
            </svg>
            Pilih Aktivitas Sesi Ini
          </h2>
          <p class="text-xs text-neutral-400 mt-0.5">
            Durasi: <strong class="text-[#d8b4e2]">{{ formatMinutes(durationSeconds) }}</strong>
          </p>
        </div>

        <button 
          type="button"
          @click="emit('close')"
          title="Tutup"
          class="text-neutral-400 hover:text-white p-1 rounded-md hover:bg-neutral-800 transition-colors cursor-pointer"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- Keterangan -->
      <p class="text-xs text-neutral-300">
        Beberapa aktivitas tercentang. Pilih aktivitas yang ingin dicatat ke <span class="text-[#d8b4e2] font-medium">Activity Summary</span>:
      </p>

      <!-- Daftar Aktivitas yang Tercentang -->
      <div class="flex flex-col gap-2 max-h-56 overflow-y-auto pr-1">
        <button
          v-for="item in activities"
          :key="item.id"
          type="button"
          @click="emit('select', item.title)"
          class="flex items-center justify-between p-3 rounded-lg bg-[#282233] hover:bg-[#382c48] border border-neutral-800 hover:border-[#d8b4e2]/50 text-left transition-all cursor-pointer group active:scale-98"
        >
          <span class="text-sm font-medium text-neutral-200 group-hover:text-white line-clamp-2">
            {{ item.title }}
          </span>
          <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 text-neutral-400 group-hover:text-[#d8b4e2] shrink-0 ml-2" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
            <polyline points="9 18 15 12 9 6" />
          </svg>
        </button>
      </div>

      <!-- Action Footer (Lewati / Tanpa Aktivitas) -->
      <div class="pt-2 border-t border-neutral-800 flex justify-end">
        <button
          type="button"
          @click="emit('skip')"
          class="text-xs text-neutral-400 hover:text-neutral-200 py-1.5 px-3 rounded hover:bg-neutral-800 transition-colors cursor-pointer"
        >
          Simpan tanpa nama aktivitas
        </button>
      </div>
    </div>
  </div>
</template>
