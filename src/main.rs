mod app;
mod game;
mod game_manager;
mod theme;
mod timer;
mod ui;
mod utils;

use crate::utils::{set_debug_enabled, set_show_mines_enabled};
use app::MinesweeperApp;
use clap::Parser;
use eframe::{NativeOptions, Theme};
use log::{error, info};
use std::sync::atomic::{AtomicBool, Ordering};

static DEBUG_ENABLED: AtomicBool = AtomicBool::new(false);

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    debug: bool,
    #[arg(long)]
    show_mines: bool,
}

fn main() -> Result<(), eframe::Error> {
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("Application panicked!");
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            eprintln!("Panic message: {}", s);
        }
        if let Some(location) = panic_info.location() {
            eprintln!(
                "Panic location: {}:{}:{}",
                location.file(),
                location.line(),
                location.column()
            );
        }
    }));

    let args = Args::parse();

    set_debug_enabled(args.debug);
    set_show_mines_enabled(args.show_mines);
    DEBUG_ENABLED.store(args.debug, Ordering::Relaxed);

    if args.debug {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .init();
        info!("Starting Minesweeper application with debug logging enabled");
    } else {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Off)
            .init();
    }
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_min_inner_size([600.0, 400.0])
            .with_resizable(true)
            .with_title("Minesweeper"),
        default_theme: Theme::Light,
        ..Default::default()
    };

    if DEBUG_ENABLED.load(Ordering::Relaxed) {
        info!("Creating application window");
    }

    let result = eframe::run_native(
        "Minesweeper",
        options,
        Box::new(|_cc| {
            if DEBUG_ENABLED.load(Ordering::Relaxed) {
                info!("Initializing MinesweeperApp");
            }
            Box::new(MinesweeperApp::new())
        }),
    );

    if DEBUG_ENABLED.load(Ordering::Relaxed) {
        match &result {
            Ok(_) => info!("Application exited successfully"),
            Err(e) => error!("Application exited with error: {:?}", e),
        }
    }

    result
}
