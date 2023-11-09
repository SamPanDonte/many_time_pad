#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(not(target_arch = "wasm32"))]
use clap::Parser;
use many_time_pad::ui::Application;
#[cfg(not(target_arch = "wasm32"))]
use many_time_pad::{Cipher, Cracker, TextEncoding};
#[cfg(not(target_arch = "wasm32"))]
use std::env::args;
#[cfg(not(target_arch = "wasm32"))]
use std::error::Error;
#[cfg(not(target_arch = "wasm32"))]
use std::num::NonZeroUsize;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to input file
    #[arg(short, long)]
    pub input: String,
    /// Path to output file
    #[arg(short, long)]
    pub output: String,
    /// Encoding
    #[arg(short, long)]
    pub encoding: TextEncoding,
    /// Key length
    #[arg(short, long, default_value = "256")]
    pub length_key: NonZeroUsize,
    /// Path to key output file
    #[arg(short, long)]
    pub key: Option<String>,
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init();

    #[cfg(not(target_arch = "wasm32"))]
    if args().len() > 1 {
        cli_main().expect("Failed to run CLI:");
        return Ok(());
    }

    let native_options = eframe::NativeOptions {
        initial_window_size: Some([600.0, 450.0].into()),
        ..Default::default()
    };
    eframe::run_native(
        "Many Time Pad",
        native_options,
        Box::new(|_| Box::<Application>::default()),
    )
}

#[cfg(not(target_arch = "wasm32"))]
fn cli_main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let input = std::fs::read(args.input)?;
    let cracker = Cracker::new(&args.encoding);
    let key = cracker.crack(&input, args.length_key).get_current_key();
    let message = Cipher::new(key.clone()).decrypt(&input);
    let message = args
        .encoding
        .decode(&message)
        .ok_or("Failed to decode message")?;

    std::fs::write(args.output, message)?;

    if let Some(key_path) = args.key {
        std::fs::write(key_path, key)?;
    }

    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id",
                eframe::WebOptions::default(),
                Box::new(|_| Box::<Application>::default()),
            )
            .await
            .expect("failed to start application");
    });
}
