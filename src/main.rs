#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use many_time_pad::ui::TemplateApp;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        initial_window_size: Some([400.0, 300.0].into()),
        min_window_size: Some([300.0, 220.0].into()),
        ..Default::default()
    };
    eframe::run_native(
        "Many Time Pad",
        native_options,
        Box::new(|_| Box::<TemplateApp>::default()),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id",
                eframe::WebOptions::default(),
                Box::new(|_| Box::<TemplateApp>::default()),
            )
            .await
            .expect("failed to start application");
    });
}
