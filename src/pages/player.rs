use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_params_map, use_query_map};
use crate::components::video_player::VideoPlayer;
use crate::services::tmdb::fetch_details;

#[component]
pub fn PlayerPage() -> impl IntoView {
    let params = use_params_map();
    let query = use_query_map();
    let navigate = use_navigate();

    let id_str = move || params.read().get("id").unwrap_or_default();
    let season_str = move || query.read().get("season").or_else(|| query.read().get("s"));
    let episode_str = move || query.read().get("episode").or_else(|| query.read().get("e"));

    let media_resource = LocalResource::new(move || {
        let id_val: u32 = id_str().parse().unwrap_or(0);
        async move {
            if id_val > 0 {
                if let Ok(d) = fetch_details(id_val, false).await {
                    Some(d)
                } else if let Ok(d) = fetch_details(id_val, true).await {
                    Some(d)
                } else {
                    None
                }
            } else {
                None
            }
        }
    });

    let display_title = move || {
        let base_title = media_resource.get().flatten().map(|m| m.display_title().to_string()).unwrap_or_else(|| "Playback".to_string());
        if let (Some(s), Some(e)) = (season_str(), episode_str()) {
            format!("{} S{}E{}", base_title, s, e)
        } else {
            base_title
        }
    };

    let stream_url = "https://test-streams.mux.dev/x36xhzz/x36xhzz.m3u8".to_string();

    let on_close = Callback::new(move |_| {
        navigate("/browse", Default::default());
    });

    view! {
        <div class="w-full h-screen bg-black overflow-hidden">
            <VideoPlayer 
                title=display_title() 
                stream_url=stream_url 
                on_close=Some(on_close) 
            />
        </div>
    }
}
