declare const Hls: any;

interface HlsInstance {
  destroy(): void;
  loadSource(url: string): void;
  attachMedia(video: HTMLVideoElement): void;
  audioTrack: number;
  currentLevel: number;
  media?: HTMLVideoElement;
  _native?: boolean;
  _video?: HTMLVideoElement;
}

const instances = new Map<string, HlsInstance>();
let counter = 0;

export function hlsAttach(videoId: string, streamUrl: string): string {
  const video = document.getElementById(videoId) as HTMLVideoElement | null;
  if (!video) return "";
  const handle = "hls_" + (++counter);

  if (typeof Hls !== 'undefined' && Hls.isSupported()) {
    const hls = new Hls({ enableWorker: true });
    hls.loadSource(streamUrl);
    hls.attachMedia(video);
    instances.set(handle, hls);
  } else if (video.canPlayType('application/vnd.apple.mpegurl')) {
    video.src = streamUrl;
    instances.set(handle, {
      destroy: () => { video.src = ""; },
      loadSource: (url: string) => { video.src = url; },
      attachMedia: () => {},
      audioTrack: 0,
      currentLevel: -1,
      _native: true,
      _video: video,
    });
  }
  return handle;
}

export function hlsDestroy(handle: string): void {
  const inst = instances.get(handle);
  if (inst) {
    inst.destroy();
    instances.delete(handle);
  }
}

export function hlsSetAudioTrack(handle: string, trackIndex: number): void {
  const inst = instances.get(handle);
  if (inst && !inst._native) {
    inst.audioTrack = trackIndex;
  }
}

export function hlsSetQuality(handle: string, level: number): void {
  const inst = instances.get(handle);
  if (inst && !inst._native) {
    inst.currentLevel = level;
  }
}

export function hlsSeek(handle: string, seconds: number): void {
  const inst = instances.get(handle);
  if (!inst) return;
  const video = inst._native ? inst._video : inst.media;
  if (video) video.currentTime = seconds;
}

export function hlsCurrentTime(handle: string): number {
  const inst = instances.get(handle);
  if (!inst) return 0;
  const video = inst._native ? inst._video : inst.media;
  return video ? video.currentTime : 0;
}

export function hlsDuration(handle: string): number {
  const inst = instances.get(handle);
  if (!inst) return 0;
  const video = inst._native ? inst._video : inst.media;
  return video ? video.duration || 0 : 0;
}
