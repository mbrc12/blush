#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let native_opts = eframe::NativeOptions{..eframe::NativeOptions::default()};
    eframe::run_native("blush", native_opts, Box::new(|cc| Box::new(blush::Blush::new(cc).unwrap())));
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    console_log::init_with_level(log::Level::Debug);
    let web_options = eframe::WebOptions::default();
    eframe::start_web(
        "the_canvas_id", // hardcode it
        web_options,
        Box::new(|cc| Box::new(blush::Blush::new(cc).unwrap())),
    )
    .expect("failed to start eframe");
}
