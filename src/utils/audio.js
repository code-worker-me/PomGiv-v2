// Global state untuk audio manager
let activeCtx = null;
let activeOscillators = [];
let duckedElements = [];
let chimeTimeoutId = null;

/**
 * Meredam (duck) audio lain yang mungkin sedang aktif di dalam webview
 */
function duckOtherAudio() {
  duckedElements = [];
  try {
    const mediaElements = document.querySelectorAll("audio, video");
    mediaElements.forEach((el) => {
      if (!el.paused && el.volume > 0) {
        duckedElements.push({
          element: el,
          originalVolume: el.volume,
        });
        // Turunkan volume ke 15% dari volume asli agar suara notifikasi lebih dominan
        el.volume = Math.max(0, el.volume * 0.15);
      }
    });
  } catch (err) {
    console.warn("Gagal melakukan audio ducking:", err);
  }
}

/**
 * Mengembalikan volume audio yang sebelumnya diredam
 */
function restoreOtherAudio() {
  if (duckedElements.length > 0) {
    duckedElements.forEach(({ element, originalVolume }) => {
      try {
        element.volume = originalVolume;
      } catch (err) {
        // Abaikan jika elemen sudah di-unmount
      }
    });
    duckedElements = [];
  }
}

/**
 * Menghentikan suara dering chime seketika (misalnya saat tombol Dismiss ditekan)
 */
export function stopChimeSound() {
  if (chimeTimeoutId) {
    clearTimeout(chimeTimeoutId);
    chimeTimeoutId = null;
  }

  // Hentikan semua oscillator yang masih berbunyi
  if (activeOscillators.length > 0) {
    activeOscillators.forEach((osc) => {
      try {
        osc.stop();
        osc.disconnect();
      } catch (err) {
        // Sudah berhenti
      }
    });
    activeOscillators = [];
  }

  if (activeCtx) {
    try {
      activeCtx.close();
    } catch (err) {
      // Abaikan
    }
    activeCtx = null;
  }

  // Kembalikan volume audio lain
  restoreOtherAudio();
}

/**
 * Memainkan suara chime dengan prioritas tertinggi & audio ducking
 */
export function playChimeSound() {
  // Jika ada chime sebelumnya yang belum selesai, hentikan dulu
  stopChimeSound();

  try {
    const AudioContext = window.AudioContext || window.webkitAudioContext;
    if (!AudioContext) return;

    activeCtx = new AudioContext();
    
    // Pastikan AudioContext dalam status running
    if (activeCtx.state === "suspended") {
      activeCtx.resume();
    }

    // 1. Prioritaskan suara notifikasi: redam audio lain di app
    duckOtherAudio();

    // 2. Mainkan nada lonceng bertingkat (C6, E6, G6, C7)
    const tones = [
      { freq: 1046.50, time: 0 },    // C6
      { freq: 1318.51, time: 0.15 }, // E6
      { freq: 1567.98, time: 0.30 }, // G6
      { freq: 2093.00, time: 0.45 }, // C7
    ];

    const ctx = activeCtx;
    activeOscillators = [];

    tones.forEach(({ freq, time }) => {
      const osc = ctx.createOscillator();
      const gain = ctx.createGain();

      osc.type = "sine";
      osc.frequency.setValueAtTime(freq, ctx.currentTime + time);

      // Volume prioritas tinggi & decay lembut
      gain.gain.setValueAtTime(0, ctx.currentTime + time);
      gain.gain.linearRampToValueAtTime(0.35, ctx.currentTime + time + 0.02);
      gain.gain.exponentialRampToValueAtTime(0.0001, ctx.currentTime + time + 1.4);

      osc.connect(gain);
      gain.connect(ctx.destination);

      osc.start(ctx.currentTime + time);
      osc.stop(ctx.currentTime + time + 1.4);

      activeOscillators.push(osc);
    });

    // Otomatis kembalikan volume setelah lonceng selesai berdering (durasi ~1.9 detik)
    chimeTimeoutId = setTimeout(() => {
      restoreOtherAudio();
      activeOscillators = [];
      activeCtx = null;
      chimeTimeoutId = null;
    }, 1900);

  } catch (err) {
    console.warn("Gagal memutar audio chime berprioritas:", err);
    restoreOtherAudio();
  }
}
