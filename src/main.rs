use pstream_frontend_rs::App;

fn main() {
    console_error_panic_hook::set_once();
    wasm_bindgen_futures::spawn_local(async {
        pstream_frontend_rs::i18n::load().await;
        leptos::mount::mount_to_body(App);
        if let Some(boot) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.get_element_by_id("boot")) {
            boot.remove();
        }
    });
}

