export interface SoundCue {
  id: string;
  name: string;
  sub: string;
  color: string;
  duration: number; // in seconds
  fadeTime: number; // in milliseconds
  category: "TM Avbrytelser" | "Humor latter" | "Applaus" | "Stemning";
  isPlaying: boolean;
  progress: number; // 0 to 100
  elapsed: number; // elapsed time in seconds
}

export interface PreflightItem {
  id: string;
  label: string;
  status: "success" | "warning" | "pending";
  description: string;
}

export interface SimulationState {
  isLinked: boolean;
  masterVolume: number; // 0 to 100
  latency: number; // in ms, e.g., 0 for local, ~5 for Tailscale remote
  activeDevice: "mac" | "iphone";
  isMuted: boolean;
  preflightPassed: boolean;
}
