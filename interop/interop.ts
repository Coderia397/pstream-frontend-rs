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

(window as any).pstream_extract_ambient_color = async (imageUrl: string) => {
  const res = await ambient.extractAmbientColor(imageUrl);
  return res ? JSON.stringify(res) : "";
};

console.log("[PStream] TypeScript interop subsystem active.");
