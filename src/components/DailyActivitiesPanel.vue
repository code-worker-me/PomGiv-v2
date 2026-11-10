<script setup>
import { ref, computed } from "vue";

const props = defineProps({
  activities: {
    type: Array,
    default: () => [],
  },
});

const emit = defineEmits(["add", "toggle", "delete"]);

const isOpen = ref(false);
const newTitle = ref("");

const activeCount = computed(() => {
  return props.activities.filter((a) => a.is_checked).length;
});

function handleAdd() {
  const trimmed = newTitle.value.trim();
  if (!trimmed) return;
  emit("add", trimmed);
  newTitle.value = "";
}

function handleKeyDown(e) {
  if (e.key === "Enter") {
    e.preventDefault();
    handleAdd();
  }
}
</script>

<template>
  <div class="w-full max-w-sm flex flex-col items-center">
    <!-- Tombol Header Collapsible -->
    <button
      type="button"
      @click="isOpen = !isOpen"
      class="flex items-center justify-between gap-3 px-4 py-2 rounded-full bg-[#241e30] hover:bg-[#2d253d] border border-neutral-800 hover:border-neutral-700 transition-all text-xs font-medium text-neutral-300 hover:text-white cursor-pointer shadow-sm group select-none"
      title="Buka / tutup daftar aktivitas harian"
    >
      <div class="flex items-center gap-2">
        <!-- Checklist Icon -->
        <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 text-[#d8b4e2]" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M9 11l3 3L22 4" />
          <path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11" />
        </svg>
        <span>Aktivitas Belajar</span>
        <span 
          v-if="activeCount > 0" 
          class="px-1.5 py-0.2 rounded-full text-[10px] bg-[#d8b4e2] text-black font-semibold"
        >
          {{ activeCount }} aktif
        </span>
      </div>

      <!-- Chevron Icon -->
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="w-3.5 h-3.5 text-neutral-400 group-hover:text-white transition-transform duration-200"
        :class="{ 'rotate-180': isOpen }"
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
        stroke-width="2"
      >
        <polyline points="6 9 12 15 18 9" />
      </svg>
    </button>

    <!-- Konten Collapsible Panel -->
    <div
      v-if="isOpen"
      class="w-full mt-3 bg-[#1e1929]/95 border border-neutral-800/80 rounded-xl p-3.5 flex flex-col gap-3 shadow-xl backdrop-blur-sm animate-in fade-in slide-in-from-top-2 duration-150"
    >
      <!-- Form Input Tambah Aktivitas -->
      <div class="flex items-center gap-1.5">
        <input
          v-model="newTitle"
          type="text"
          placeholder="Misal: Selesai dengan Golang..."
          @keydown="handleKeyDown"
          maxlength="100"
          class="flex-1 bg-[#15121d] border border-neutral-800 rounded-lg px-3 py-1.5 text-xs text-neutral-100 placeholder-neutral-500 focus:outline-none focus:border-[#d8b4e2] transition-colors"
        />
        <button
          type="button"
          @click="handleAdd"
          title="Tambah aktivitas"
          class="px-3 py-1.5 rounded-lg bg-[#d8b4e2] hover:bg-[#cb9ed6] text-black font-semibold text-xs active:scale-95 transition-all cursor-pointer shrink-0"
        >
          + Tambah
        </button>
      </div>

      <!-- Keterangan Cepat -->
      <p class="text-[10px] text-neutral-400 px-0.5">
        Centang aktivitas yang sedang Anda kerjakan agar otomatis tercatat ke <span class="text-[#d8b4e2]">Activity Summary</span> saat sesi selesai.
      </p>

      <!-- Daftar Aktivitas -->
      <div class="flex flex-col gap-1.5 max-h-44 overflow-y-auto pr-1">
        <div
          v-for="item in activities"
          :key="item.id"
          class="flex items-center justify-between gap-2 p-2 rounded-lg bg-[#272134] hover:bg-[#302840] border border-neutral-800/60 transition-colors group"
        >
          <!-- Checkbox & Judul -->
          <label class="flex items-center gap-2.5 flex-1 min-w-0 cursor-pointer select-none">
            <input
              type="checkbox"
              :checked="item.is_checked"
              @change="emit('toggle', item)"
              class="w-4 h-4 rounded border-neutral-600 accent-[#d8b4e2] cursor-pointer"
            />
            <span
              class="text-xs truncate transition-all"
              :class="item.is_checked ? 'text-neutral-100 font-medium' : 'text-neutral-400'"
            >
              {{ item.title }}
            </span>
          </label>

          <!-- Tombol Hapus -->
          <button
            type="button"
            @click="emit('delete', item.id)"
            title="Hapus aktivitas"
            class="text-neutral-500 hover:text-rose-400 p-1 rounded hover:bg-neutral-800/80 transition-colors cursor-pointer shrink-0"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
              <path stroke-linecap="round" stroke-linejoin="round" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
            </svg>
          </button>
        </div>

        <!-- State Kosong -->
        <div
          v-if="activities.length === 0"
          class="text-center py-4 text-xs text-neutral-500 italic"
        >
          Belum ada aktivitas. Ketik di atas untuk menambahkan!
        </div>
      </div>
    </div>
  </div>
</template>
