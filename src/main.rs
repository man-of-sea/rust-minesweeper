mod board;
mod cell;
mod difficulty;
mod app;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Minesweeper")
            .with_inner_size([400.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Minesweeper",
        options,
        Box::new(|_cc| Box::new(app::MinesweeperApp::new()))
    )
}

