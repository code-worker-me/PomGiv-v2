// Helper Web Audio API untuk membunyikan suara chime/lonceng saat waktu habis
export function playChimeSound() {
  try {
    const AudioContext = window.AudioContext || window.webkitAudioContext;
    if (!AudioContext) return;
    const ctx = new AudioContext();

    // Nada lonceng bertingkat yang menenangkan (C6, E6, G6, C7)
    const tones = [
      { freq: 1046.50, time: 0 },    // C6
      { freq: 1318.51, time: 0.15 }, // E6
      { freq: 1567.98, time: 0.30 }, // G6
      { freq: 2093.00, time: 0.45 }, // C7
    ];

    tones.forEach(({ freq, time }) => {
      const osc = ctx.createOscillator();
      const gain = ctx.createGain();

      osc.type = "sine";
      osc.frequency.setValueAtTime(freq, ctx.currentTime + time);

      // Volume & exponential decay lembut
      gain.gain.setValueAtTime(0, ctx.currentTime + time);
      gain.gain.linearRampToValueAtTime(0.25, ctx.currentTime + time + 0.02);
      gain.gain.exponentialRampToValueAtTime(0.0001, ctx.currentTime + time + 1.2);

      osc.connect(gain);
      gain.connect(ctx.destination);

      osc.start(ctx.currentTime + time);
      osc.stop(ctx.currentTime + time + 1.2);
    });
  } catch (err) {
    console.warn("Gagal memutar audio chime:", err);
  }
}
