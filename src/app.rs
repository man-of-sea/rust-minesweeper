use eframe::egui;
use crate::board::Board;
use crate::difficulty::Difficulty;

enum Screen {
    Menu { selected: usize },
    CustomSetup { rows: String, cols: String, mines: String },
    Playing
}

pub struct MinesweeperApp {
    board: Option<Board>,
    difficulty: Difficulty,
    screen: Screen
}

impl MinesweeperApp {
    pub fn new() -> Self {
        MinesweeperApp {
            board: None,
            difficulty: Difficulty::Beginner,
            screen: Screen::Menu { selected : 0 }
        }
    }
    
    fn draw_menu(&mut self, ui: &mut egui::Ui) {
        let options = [
            Difficulty::Beginner,
            Difficulty::Intermediate,
            Difficulty::Expert,
            Difficulty::Custom { rows: 0, cols: 0, mines: 0 }
        ];

        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            ui.heading("Minesweeper");
            ui.add_space(20.0);

            for option in &options {
                if ui.button(option.label()).clicked() {
                    if matches!(option, Difficulty::Custom { .. }) {
                        self.screen = Screen::CustomSetup {
                            rows:  String::new(),
                            cols:  String::new(),
                            mines: String::new(),
                        }
                    }
                    else {
                        self.start_game(*option);
                    }
                }
            }
        });
    }

    fn start_game(&mut self, difficulty: Difficulty) {
        let (rows, cols, mines) = difficulty.settings();
        self.difficulty = difficulty;
        self.board = Some(Board::new(rows, cols, mines));
        self.screen = Screen::Playing;
    }

    fn draw_custom_setup(&mut self, ui: &mut egui::Ui) {
        ui.label("TODO");
    }

    fn draw_game(&mut self, ui: &mut egui::Ui) {
        ui.label("TODO");
    }
}

impl eframe::App for MinesweeperApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match &self.screen {
                Screen::Menu { .. }         => self.draw_menu(ui),
                Screen::CustomSetup { .. }  => self.draw_custom_setup(ui),
                Screen::Playing             => self.draw_game(ui)
            }
        });
    }
}