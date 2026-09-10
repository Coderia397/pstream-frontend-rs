declare const YT: any;

interface YouTubePlayerInstance {
  destroy(): void;
  playVideo(): void;
  pauseVideo(): void;
  stopVideo(): void;
  mute(): void;
  unMute(): void;
  isMuted(): boolean;
  seekTo(seconds: number, allowSeekAhead: boolean): void;
  getCurrentTime(): number;
  getDuration(): number;
  getPlayerState(): number;
}

const DEFAULT_CROP: Record<string, number> = { card: 1.35, hero: 1.15, modal: 1.35, clips: 2.2 };
const IS_WEBKIT =
  typeof window !== 'undefined' &&
  (/iPad|iPhone|iPod/.test(navigator.userAgent) ||
    /^((?!chrome|android).)*safari/i.test(navigator.userAgent));
const ARTIFICIAL_SCALE = IS_WEBKIT ? 2.2 : 5;

const instances = new Map<string, YouTubePlayerInstance>();
let ytApiReadyPromise: Promise<void> | null = null;

function ensureYouTubeApi(): Promise<void> {
  if (ytApiReadyPromise) return ytApiReadyPromise;

  ytApiReadyPromise = new Promise((resolve) => {
    if (typeof (window as any).YT !== 'undefined' && (window as any).YT.Player) {
      return resolve();
    }
    const tag = document.createElement('script');
    tag.src = 'https://www.youtube.com/iframe_api';
    const firstScriptTag = document.getElementsByTagName('script')[0];
    firstScriptTag?.parentNode?.insertBefore(tag, firstScriptTag);

    (window as any).onYouTubeIframeAPIReady = () => {
      resolve();
    };
  });
  return ytApiReadyPromise;
}

export function calcTrailerStyle(variant: string, customCrop?: number): string {
  const zoomFactor = customCrop ?? DEFAULT_CROP[variant] ?? 1.35;
  const widthPercent = ARTIFICIAL_SCALE * 115;
  const heightPercent = ARTIFICIAL_SCALE * 115;
  const scale = zoomFactor / ARTIFICIAL_SCALE;

  return `position: absolute; top: 50%; left: 50%; width: ${widthPercent}%; height: ${heightPercent}%; transform: translate(-50%, -50%) scale(${scale}); transform-origin: center center; will-change: transform; pointer-events: none;`;
}

export async function createYouTubePlayer(
  containerId: string,
  videoId: string,
  options: {
    startTime?: number;
    mute?: boolean;
    onReady?: () => void;
    onPlay?: () => void;
    onEnded?: () => void;
    onError?: () => void;
  }
): Promise<string> {
  await ensureYouTubeApi();

  const container = document.getElementById(containerId);
  if (!container) return "";

  const handle = "yt_" + Math.random().toString(36).substring(2, 9);
  const startTime = options.startTime ?? 8;

  const player = new (window as any).YT.Player(containerId, {
    videoId,
    width: '100%',
    height: '100%',
    playerVars: {
      autoplay: 1,
      mute: options.mute ? 1 : 0,
      controls: 0,
      modestbranding: 1,
      rel: 0,
      showinfo: 0,
      suggestedQuality: 'small',
      iv_load_policy: 3,
      cc_load_policy: 0,
      enablejsapi: 1,
      playsinline: 1,
      disablekb: 1,
      start: startTime,
      origin: window.location.origin,
      widget_referrer: window.location.origin,
    },
    events: {
      onReady: (event: any) => {
        if (options.mute) event.target.mute();
        options.onReady?.();
      },
      onStateChange: (event: any) => {
        if (event.data === 1) { // YT.PlayerState.PLAYING
          options.onPlay?.();
        } else if (event.data === 0) { // YT.PlayerState.ENDED
          options.onEnded?.();
        }
      },
      onError: () => {
        options.onError?.();
      }
    }
  });

  instances.set(handle, player);
  return handle;
}

export function ytPlay(handle: string): void {
  const p = instances.get(handle);
  if (p && typeof p.playVideo === 'function') {
    p.playVideo();
  }
}

export function ytPause(handle: string): void {
  const p = instances.get(handle);
  if (p && typeof p.pauseVideo === 'function') {
    p.pauseVideo();
  }
}

export function ytMute(handle: string, mute: boolean): void {
  const p = instances.get(handle);
  if (!p) return;
  if (mute && typeof p.mute === 'function') {
    p.mute();
  } else if (!mute && typeof p.unMute === 'function') {
    p.unMute();
  }
}

export function ytSeek(handle: string, seconds: number): void {
  const p = instances.get(handle);
  if (p && typeof p.seekTo === 'function') {
    p.seekTo(seconds, true);
  }
}

export function ytDestroy(handle: string): void {
  const p = instances.get(handle);
  if (p) {
    try {
      p.stopVideo();
      p.destroy();
    } catch {}
    instances.delete(handle);
  }
}
