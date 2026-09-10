//! HLS.js WebAssembly Adapter
use wasm_bindgen::prelude::*;
use web_sys::HtmlVideoElement;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = pstream_hls_attach)]
    pub fn hls_attach(video_id: &str, stream_url: &str) -> String;

    #[wasm_bindgen(js_namespace = window, js_name = pstream_hls_destroy)]
    pub fn hls_destroy(handle: &str);

    #[wasm_bindgen(js_namespace = window, js_name = pstream_hls_set_audio_track)]
    pub fn hls_set_audio_track(handle: &str, track_index: i32);

    #[wasm_bindgen(js_namespace = window, js_name = pstream_hls_set_quality)]
    pub fn hls_set_quality(handle: &str, level: i32);

    #[wasm_bindgen(js_namespace = window, js_name = pstream_hls_seek)]
    pub fn hls_seek(handle: &str, seconds: f64);

    #[wasm_bindgen(js_namespace = window, js_name = pstream_hls_current_time)]
    pub fn hls_current_time(handle: &str) -> f64;

    #[wasm_bindgen(js_namespace = window, js_name = pstream_hls_duration)]
    pub fn hls_duration(handle: &str) -> f64;
}

pub struct HlsPlayer {
    handle: String,
    pub video_element: HtmlVideoElement,
}

impl HlsPlayer {
    pub fn attach(video_element: HtmlVideoElement, stream_url: &str) -> Self {
        let video_id = video_element
            .get_attribute("id")
            .unwrap_or_else(|| "pstream-video".to_string());
        let handle = hls_attach(&video_id, stream_url);
        Self { handle, video_element }
    }

    pub fn play(&self) {
        let _ = self.video_element.play();
    }

    pub fn pause(&self) {
        let _ = self.video_element.pause();
    }

    pub fn seek(&self, seconds: f64) {
        hls_seek(&self.handle, seconds);
    }

    pub fn set_audio_track(&self, index: i32) {
        hls_set_audio_track(&self.handle, index);
    }

    pub fn set_quality(&self, level: i32) {
        hls_set_quality(&self.handle, level);
    }

    pub fn current_time(&self) -> f64 {
        hls_current_time(&self.handle)
    }

    pub fn duration(&self) -> f64 {
        hls_duration(&self.handle)
    }
}

impl Drop for HlsPlayer {
    fn drop(&mut self) {
        hls_destroy(&self.handle);
    }
}
