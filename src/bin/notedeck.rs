#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use notedeck::Damus;

// Entry point for wasm
//#[cfg(target_arch = "wasm32")]
//use wasm_bindgen::prelude::*;

// Desktop
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let window_builder = Box::new(|builder: egui::ViewportBuilder| {
        builder.with_fullsize_content_view(true)
            .with_titlebar_shown(false)
            .with_title_shown(false)
    });
    let mut native_options = eframe::NativeOptions::default();
    native_options.window_builder = Some(window_builder); 

    let _res = eframe::run_native(
        "Damus NoteDeck",
        native_options,
        Box::new(|cc| Box::new(Damus::new(cc, "."))),
    );
}

#[cfg(target_arch = "wasm32")]
pub fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    wasm_bindgen_futures::spawn_local(async {
        let web_options = eframe::WebOptions::default();
        eframe::start_web(
            "the_canvas_id", // hardcode it
            web_options,
            Box::new(|cc| Box::new(Damus::new(cc, "."))),
        )
        .await
        .expect("failed to start eframe");
    });
}
