import * as hls from './hls';
import * as yt from './youtube';
import * as ambient from './ambient';

// Expose global bridges on window for Rust wasm-bindgen extern calls
(window as any).pstream_hls_attach = hls.hlsAttach;
(window as any).pstream_hls_destroy = hls.hlsDestroy;
(window as any).pstream_hls_set_audio_track = hls.hlsSetAudioTrack;
(window as any).pstream_hls_set_quality = hls.hlsSetQuality;
(window as any).pstream_hls_seek = hls.hlsSeek;
(window as any).pstream_hls_current_time = hls.hlsCurrentTime;
(window as any).pstream_hls_duration = hls.hlsDuration;

(window as any).pstream_yt_calc_style = yt.calcTrailerStyle;
(window as any).pstream_yt_create = yt.createYouTubePlayer;
(window as any).pstream_yt_play = yt.ytPlay;
(window as any).pstream_yt_pause = yt.ytPause;
(window as any).pstream_yt_mute = yt.ytMute;
(window as any).pstream_yt_seek = yt.ytSeek;
(window as any).pstream_yt_destroy = yt.ytDestroy;

(window as any).pstream_get_cached_ambient_color = (imageUrl: string) => {
  const res = ambient.getCachedAmbientColor(imageUrl);
  return res ? JSON.stringify(res) : "";
};

(window as any).pstream_get_last_ambient_color = () => {
  const res = ambient.getLastAmbientColor();
  return res ? JSON.stringify(res) : "";
};

(window as any).pstream_extract_ambient_color = (imageUrl: string, callback: (res: string) => void) => {
  ambient.extractAmbientColor(imageUrl).then((res) => {
    if (typeof callback === 'function') {
      callback(res ? JSON.stringify(res) : "");
    }
  });
};

(window as any).pstream_apply_ambient_color = (r: number, g: number, b: number) => {
  try {
    document.documentElement.style.setProperty('--ambient-r', String(r));
    document.documentElement.style.setProperty('--ambient-g', String(g));
    document.documentElement.style.setProperty('--ambient-b', String(b));
    let meta = document.querySelector('meta[name="theme-color"]') as HTMLMetaElement | null;
    if (!meta) {
      meta = document.createElement('meta');
      meta.name = 'theme-color';
      document.head.appendChild(meta);
    }
    const tr = Math.round(r * 0.45);
    const tg = Math.round(g * 0.45);
    const tb = Math.round(b * 0.45);
    meta.content = `rgb(${tr}, ${tg}, ${tb})`;
  } catch {}
};

(window as any).pstream_set_theme_color = (r: number, g: number, b: number) => {
  try {
    let meta = document.querySelector('meta[name="theme-color"]') as HTMLMetaElement | null;
    if (!meta) {
      meta = document.createElement('meta');
      meta.name = 'theme-color';
      document.head.appendChild(meta);
    }
    const tr = Math.round(r * 0.45);
    const tg = Math.round(g * 0.45);
    const tb = Math.round(b * 0.45);
    meta.content = `rgb(${tr}, ${tg}, ${tb})`;
  } catch {}
};

console.log("[PStream] TypeScript interop subsystem active.");

