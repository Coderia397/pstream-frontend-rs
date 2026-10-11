use pstream_frontend_rs::App;

fn main() {
    console_error_panic_hook::set_once();
    wasm_bindgen_futures::spawn_local(async {
        pstream_frontend_rs::i18n::load().await;
        leptos::mount::mount_to_body(App);
    });
}

