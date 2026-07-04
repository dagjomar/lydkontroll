// Web Audio API Sound Synthesizer for Lydkontroll

class AudioSynthesizer {
  private ctx: AudioContext | null = null;
  private masterGain: GainNode | null = null;
  private activeSources: Map<
    string,
    {
      nodes: AudioNode[];
      timer: number;
      interval?: number;
    }
  > = new Map();
  private volume: number = 0.8; // default volume

  init() {
    if (this.ctx) return;
    try {
      const AudioContextClass =
        window.AudioContext || (window as any).webkitAudioContext;
      this.ctx = new AudioContextClass();
      this.masterGain = this.ctx.createGain();
      this.masterGain.gain.setValueAtTime(this.volume, this.ctx.currentTime);
      this.masterGain.connect(this.ctx.destination);
    } catch (e) {
      console.error("Failed to initialize Web Audio API", e);
    }
  }

  setMasterVolume(vol: number) {
    this.volume = vol / 100;
    if (this.masterGain && this.ctx) {
      this.masterGain.gain.setValueAtTime(this.volume, this.ctx.currentTime);
    }
  }

  stopAll() {
    this.activeSources.forEach((source, id) => {
      this.stopCue(id);
    });
  }

  stopCue(id: string) {
    const active = this.activeSources.get(id);
    if (active) {
      // Fade out smoothly over 150ms before stopping
      if (this.ctx) {
        const now = this.ctx.currentTime;
        active.nodes.forEach((node) => {
          if (node instanceof GainNode) {
            node.gain.cancelScheduledValues(now);
            node.gain.setValueAtTime(node.gain.value, now);
            node.gain.exponentialRampToValueAtTime(0.001, now + 0.15);
          }
        });
        setTimeout(() => {
          active.nodes.forEach((node) => {
            if ("stop" in node) {
              try {
                (node as any).stop();
              } catch (e) {}
            }
          });
        }, 160);
      }
      window.clearInterval(active.interval);
      window.clearTimeout(active.timer);
      this.activeSources.delete(id);
    }
  }

  startCue(
    id: string,
    onUpdate: (elapsed: number) => void,
    onEnded: () => void,
  ) {
    this.init();
    if (!this.ctx || !this.masterGain) {
      onEnded();
      return;
    }

    // Resume context if suspended (browser security)
    if (this.ctx.state === "suspended") {
      this.ctx.resume();
    }

    // Stop existing if playing
    this.stopCue(id);

    const now = this.ctx.currentTime;
    const nodesToCleanup: AudioNode[] = [];
    let duration = 1.5; // default

    if (id === "glass_tap") {
      duration = 2.0;
      // High pitched crystal ding
      const osc1 = this.ctx.createOscillator();
      const osc2 = this.ctx.createOscillator();
      const gainNode = this.ctx.createGain();

      osc1.type = "sine";
      osc1.frequency.setValueAtTime(2200, now); // Primary high frequency

      osc2.type = "sine";
      osc2.frequency.setValueAtTime(3412, now); // Secondary harmonic to simulate glass tension

      gainNode.gain.setValueAtTime(0.3, now);
      // Sharp crystal strike and smooth long ringing release
      gainNode.gain.exponentialRampToValueAtTime(0.001, now + duration);

      osc1.connect(gainNode);
      osc2.connect(gainNode);
      gainNode.connect(this.masterGain);

      osc1.start(now);
      osc2.start(now);

      nodesToCleanup.push(osc1, osc2, gainNode);
    } else if (id === "ding_dong") {
      duration = 2.2;
      // Two-tone door bell
      const osc = this.ctx.createOscillator();
      const gainNode = this.ctx.createGain();

      osc.type = "sine";
      // "Ding" - C5 (523.25Hz)
      osc.frequency.setValueAtTime(523.25, now);
      // "Dong" - G4 (392.00Hz) after 0.6 seconds
      osc.frequency.setValueAtTime(523.25, now + 0.5);
      osc.frequency.exponentialRampToValueAtTime(392.0, now + 0.6);

      gainNode.gain.setValueAtTime(0.4, now);
      gainNode.gain.setValueAtTime(0.4, now + 0.5);
      gainNode.gain.exponentialRampToValueAtTime(0.001, now + duration);

      osc.connect(gainNode);
      gainNode.connect(this.masterGain);

      osc.start(now);

      nodesToCleanup.push(osc, gainNode);
    } else if (id === "fanfare") {
      duration = 3.0;
      // Synthesized brass arpeggio: C4, G4, C5, E5, G5
      const notes = [
        { freq: 261.63, start: 0, length: 1.5 }, // C4
        { freq: 392.0, start: 0.15, length: 1.35 }, // G4
        { freq: 523.25, start: 0.3, length: 1.2 }, // C5
        { freq: 659.25, start: 0.45, length: 1.05 }, // E5
        { freq: 783.99, start: 0.6, length: 2.2 }, // G5 (sustained)
      ];

      const masterGroupGain = this.ctx.createGain();
      masterGroupGain.gain.setValueAtTime(0.2, now);
      masterGroupGain.gain.exponentialRampToValueAtTime(0.001, now + duration);
      masterGroupGain.connect(this.masterGain);
      nodesToCleanup.push(masterGroupGain);

      notes.forEach((note) => {
        if (!this.ctx) return;
        const o1 = this.ctx.createOscillator();
        const o2 = this.ctx.createOscillator(); // Detuned to fatten the brass sound
        const filter = this.ctx.createBiquadFilter();
        const g = this.ctx.createGain();

        o1.type = "sawtooth";
        o1.frequency.setValueAtTime(note.freq, now + note.start);

        o2.type = "triangle";
        o2.frequency.setValueAtTime(note.freq + 2, now + note.start);

        // Filter gives it a trumpeting swell
        filter.type = "lowpass";
        filter.frequency.setValueAtTime(300, now + note.start);
        filter.frequency.exponentialRampToValueAtTime(
          2000,
          now + note.start + 0.1,
        );
        filter.Q.setValueAtTime(4, now + note.start);

        g.gain.setValueAtTime(0.001, now + note.start);
        g.gain.linearRampToValueAtTime(0.3, now + note.start + 0.05);
        g.gain.exponentialRampToValueAtTime(
          0.001,
          now + note.start + note.length,
        );

        o1.connect(filter);
        o2.connect(filter);
        filter.connect(g);
        g.connect(masterGroupGain);

        o1.start(now + note.start);
        o2.start(now + note.start);

        nodesToCleanup.push(o1, o2, filter, g);
      });
    } else if (id === "laugh") {
      duration = 2.5;
      // Bubbly cartoon comic synth boing
      const osc = this.ctx.createOscillator();
      const filter = this.ctx.createBiquadFilter();
      const gainNode = this.ctx.createGain();

      osc.type = "triangle";
      osc.frequency.setValueAtTime(150, now);

      // Pitch sweeps up and down mimicking a chuckling pattern
      for (let i = 0; i < 6; i++) {
        const stepTime = now + i * 0.35;
        osc.frequency.setValueAtTime(220 + (i % 2 === 0 ? 120 : 0), stepTime);
        osc.frequency.exponentialRampToValueAtTime(
          140 - i * 10,
          stepTime + 0.25,
        );
      }

      filter.type = "peaking";
      filter.frequency.setValueAtTime(400, now);
      filter.Q.setValueAtTime(5, now);

      gainNode.gain.setValueAtTime(0.35, now);
      gainNode.gain.exponentialRampToValueAtTime(0.001, now + duration);

      osc.connect(filter);
      filter.connect(gainNode);
      gainNode.connect(this.masterGain);

      osc.start(now);
      nodesToCleanup.push(osc, filter, gainNode);
    } else if (id === "applause") {
      duration = 5.0; // Longer sound

      // Generate standard white noise
      const bufferSize = this.ctx.sampleRate * duration;
      const buffer = this.ctx.createBuffer(1, bufferSize, this.ctx.sampleRate);
      const data = buffer.getChannelData(0);

      for (let i = 0; i < bufferSize; i++) {
        data[i] = Math.random() * 2 - 1;
      }

      const noiseSource = this.ctx.createBufferSource();
      noiseSource.buffer = buffer;

      // Filter noise to resemble hands clapping and audience wash
      const filter = this.ctx.createBiquadFilter();
      filter.type = "bandpass";
      filter.frequency.setValueAtTime(800, now);
      filter.Q.setValueAtTime(0.8, now);

      // Volume envelope to fade-in audience then fade-out at the end
      const gainNode = this.ctx.createGain();
      gainNode.gain.setValueAtTime(0.001, now);
      gainNode.gain.linearRampToValueAtTime(0.25, now + 0.8); // Fade in over 0.8s
      gainNode.gain.setValueAtTime(0.25, now + 3.8);
      gainNode.gain.exponentialRampToValueAtTime(0.001, now + duration); // Fade out

      // Low frequency oscillator (LFO) to modulate amplitude to create dynamic "claps"
      const lfo = this.ctx.createOscillator();
      const lfoGain = this.ctx.createGain();
      lfo.frequency.setValueAtTime(6.5, now); // 6.5 claps per second modulation
      lfo.type = "triangle";

      lfoGain.gain.setValueAtTime(0.12, now); // intensity of claps modulation

      lfo.connect(lfoGain);
      lfoGain.connect(gainNode.gain); // modulate volume

      noiseSource.connect(filter);
      filter.connect(gainNode);
      gainNode.connect(this.masterGain);

      lfo.start(now);
      noiseSource.start(now);

      nodesToCleanup.push(noiseSource, filter, gainNode, lfo, lfoGain);
    }

    // Set up reporting and timers
    let elapsed = 0;
    const interval = window.setInterval(() => {
      elapsed += 0.05;
      if (elapsed >= duration) {
        window.clearInterval(interval);
      } else {
        onUpdate(elapsed);
      }
    }, 50);

    const timer = window.setTimeout(() => {
      window.clearInterval(interval);
      this.activeSources.delete(id);
      onEnded();
    }, duration * 1000);

    this.activeSources.set(id, {
      nodes: nodesToCleanup,
      timer,
      interval,
    });
  }
}

export const soundSynth = new AudioSynthesizer();
