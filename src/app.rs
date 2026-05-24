use eframe::egui;
use crate::board::Board;
use crate::cell::CellState;
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
            if ui.button("Start").clicked() { start = true };
            if ui.button("Back").clicked()  { back  = true  };
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
        let mut left_click:  Option<(usize, usize)> = None;
        let mut right_click: Option<(usize, usize)> = None;
        let mut restart = false;
        let mut go_menu = false;

        let board = match &self.board {
            Some(b) => b,
            None => return
        };

        let cell_size = 32.0;
        let padding   =  4.0;

        ui.horizontal(|ui| {
            ui.label(format!("mines: {}", board.mines as isize - board.flags_placed() as isize));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Menu").clicked() {
                    go_menu = true;
                }
            });
        });

        ui.separator();

        let board_width  = board.cols as f32 * (cell_size + padding);
        let board_height = board.rows as f32 * (cell_size + padding);
        let (response, painter) = ui.allocate_painter(
            egui::vec2(board_width, board_height),
            egui::Sense::click()
        );

        let origin = response.rect.min;

        for r in 0..board.rows {
            for c in 0..board.cols {
                let cell = &board.cells[r][c];

                let x = origin.x + c as f32 * (cell_size + padding);
                let y = origin.y + r as f32 * (cell_size + padding);
                let rect = egui::Rect::from_min_size(
                    egui::pos2(x, y),
                    egui::vec2(cell_size, cell_size)
                );

                let bg_colour = match cell.state {
                    CellState::Revealed => egui::Color32::from_rgb(200, 200, 200),
                    CellState::Flagged  => egui::Color32::from_rgb(180, 180, 180),
                    CellState::Hidden   => egui::Color32::from_rgb(120, 120, 120)
                };

                painter.rect_filled(rect, 4.0, bg_colour);

                match &cell.state {
                    CellState::Flagged => {
                        painter.text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            "F",
                            egui::FontId::proportional(20.0),
                            egui::Color32::RED
                        );
                    },
                    CellState::Revealed if cell.is_mine => {
                        painter.text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            "*",
                            egui::FontId::proportional(20.0),
                            egui::Color32::BLACK
                        );
                    },
                    CellState::Revealed if cell.adjacent > 0 => {
                        let colour = match cell.adjacent {
                            1 => egui::Color32::BLUE,
                            2 => egui::Color32::GREEN,
                            3 => egui::Color32::RED,
                            4 => egui::Color32::DARK_BLUE,
                            5 => egui::Color32::DARK_RED,
                            6 => egui::Color32::YELLOW,
                            7 => egui::Color32::BLACK,
                            _ => egui::Color32::GRAY
                        };
                        painter.text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            cell.adjacent.to_string(),
                            egui::FontId::proportional(18.0),
                            colour
                        );
                    }
                    _ => {}
                }

            }
        }

        if let Some(pos) = response.interact_pointer_pos() {
            let col = ((pos.x - origin.x) / (cell_size + padding)) as usize;
            let row = ((pos.y - origin.y) / (cell_size + padding)) as usize;

            if row < board.rows && col < board.cols {
                if response.clicked() {
                    left_click = Some((row, col));
                }
                if response.secondary_clicked() {
                    right_click = Some((row, col));
                }
            }
        }

        if board.game_over {
            ui.separator();
            ui.vertical_centered(|ui| {
                ui.colored_label(egui::Color32::RED, "You lost!");
                if ui.button("Play Again").clicked() {
                    restart = true;
                }
                if ui.button("Menu").clicked() {
                    go_menu = true;
                }
            });
        } else if board.won {
            ui.separator();
            ui.vertical_centered(|ui| {
                ui.colored_label(egui::Color32::GREEN, "You won!");
                if ui.button("Play Again").clicked() {
                    restart = true;
                }
                if ui.button("Menu").clicked() {
                    go_menu = true;
                }
            });
        }

        if let Some((r, c)) = left_click {
            if let Some(board) = &mut self.board {
                if !board.game_over && !board.won {
                    board.reveal(r, c);
                }
            }
        }

        if let Some((r, c)) = right_click {
            if let Some(board) = &mut self.board {
                if !board.game_over && !board.won {
                    board.toggle_flag(r, c);
                }
            }
        }

        if restart {
            self.start_game(self.difficulty);
        }

        if go_menu {
            self.screen = Screen::Menu { selected: 0 };
            self.board = None;
        }
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