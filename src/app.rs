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
        let (mut rows, mut cols, mut mines) = 
            if let Screen::CustomSetup { rows, cols, mines } = &self.screen {
                (rows.clone(), cols.clone(), mines.clone())
            } else {
                return;
        };

        
        ui.add_space(40.0);
        ui.vertical_centered(|ui| { ui.heading("Custom Difficulty"); });
        ui.add_space(20.0);

        ui.label("Rows (5 - 30):");
        ui.text_edit_singleline(&mut rows);
        ui.add_space(8.0);

        ui.label("Cols (5 - 50):");
        ui.text_edit_singleline(&mut cols);
        ui.add_space(8.0);

        ui.label("Mines:");
        ui.text_edit_singleline(&mut mines);
        ui.add_space(16.0);

        let mut start = false;
        let mut back  = false;

        ui.horizontal(|ui| {
            if ui.button("Start").clicked { start = true };
            if ui.button("Back").clicked  { back  = true  };
        });

        if let Screen::CustomSetup { rows: r, cols: c, mines: m } = &mut self.screen {
            *r = rows.clone();
            *c = cols.clone();
            *m = mines.clone();
        } 

        if start {
            let parsed_rows = rows.trim().parse::<usize>();
            let parsed_cols = cols.trim().parse::<usize>();
            let parsed_mines = mines.trim().parse::<usize>();

            if let (Ok(r), Ok(c), Ok(m)) = (parsed_rows, parsed_cols, parsed_mines) {
                if r >= 5 && r <= 30 && c >= 5 && c <= 50 && m >= 1 && m < r * c {
                    self.start_game(Difficulty::Custom { rows: r, cols: c, mines: m });
                }
            }
        }

        if back {
            self.screen = Screen::Menu { selected: 0 };
        }
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