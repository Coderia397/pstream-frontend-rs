// Declaration file defining window.pstream_* functions invoked by Rust WebAssembly

declare global {
  interface Window {
    // HLS.js streaming adapter
    pstream_hls_attach(videoId: string, streamUrl: string): string;
    pstream_hls_destroy(handle: string): void;
    pstream_hls_set_audio_track(handle: string, trackIndex: number): void;
    pstream_hls_set_quality(handle: string, level: number): void;
    pstream_hls_seek(handle: string, seconds: number): void;
    pstream_hls_current_time(handle: string): number;
    pstream_hls_duration(handle: string): number;

    // YouTube trailer player adapter
    pstream_yt_calc_style(variant: string, customCrop?: number): string;
    pstream_yt_create(
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
    ): Promise<string>;
    pstream_yt_play(handle: string): void;
    pstream_yt_pause(handle: string): void;
    pstream_yt_mute(handle: string, mute: boolean): void;
    pstream_yt_seek(handle: string, seconds: number): void;
    pstream_yt_destroy(handle: string): void;

    // Ambient color extractor
    pstream_extract_ambient_color(imageUrl: string): Promise<string>;
  }
}

export {};
