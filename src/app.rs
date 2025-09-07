use crate::game::GameBoard;
use crate::game_manager::{Difficulty, GameManager};
use crate::theme::apply_custom_style;
use crate::ui::GameUI;
use crate::utils::is_logging_enabled;
use eframe::egui;
use log::{debug, warn};

pub struct MinesweeperApp {
    game_state: GameBoard,
    game_manager: GameManager,
    ui: GameUI,
}

impl MinesweeperApp {
    pub fn new() -> Self {
        if is_logging_enabled() {
            debug!("Creating new MinesweeperApp");
        }

        let difficulty = Difficulty::Beginner;
        let (width, height, mines) = difficulty.get_dimensions();

        if is_logging_enabled() {
            debug!(
                "Initializing game with difficulty: {:?}, dimensions: {}x{}, mines: {}",
                difficulty, width, height, mines
            );
        }

        let mut app = Self {
            game_state: GameBoard::new(width, height, mines),
            game_manager: GameManager::new(),
            ui: GameUI::new(),
        };

        app.game_manager.current_difficulty = difficulty;
        app
    }
}

impl eframe::App for MinesweeperApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        apply_custom_style(ctx);

        if is_logging_enabled() {
            let keys_just_pressed: Vec<egui::Key> =
                ctx.input(|i| i.keys_down.iter().cloned().collect());
            if !keys_just_pressed.is_empty() {
                debug!("Keys just pressed: {:?}", keys_just_pressed);
            }
        }

        if self.game_state.game_started
            && !self.game_state.game_over
            && ctx.input(|i| i.key_pressed(egui::Key::Escape))
        {
            if is_logging_enabled() {
                debug!("Escape key pressed - toggling pause/resume");
            }
            if self.game_manager.timer.is_running() {
                if is_logging_enabled() {
                    debug!("Pausing game via escape key");
                }
                self.game_manager.pause_game();
            } else {
                if is_logging_enabled() {
                    debug!("Resuming game via escape key");
                }
                self.game_manager.resume_game();
            }
        }

        if self.game_state.game_over && self.game_manager.timer.is_running() {
            if is_logging_enabled() {
                debug!(
                    "Game over detected, ending timer. Won: {}",
                    self.game_state.game_won
                );
            }
            self.game_manager.end_game(self.game_state.game_won);
        }

        if !self.game_state.game_started && self.game_manager.timer.is_running() {
            if is_logging_enabled() {
                warn!("Timer is running but game hasn't started - pausing timer");
            }
            self.game_manager.pause_game();
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(egui::Color32::from_rgb(18, 18, 22)))
            .show(ctx, |ui| {
                self.ui
                    .render(ui, &mut self.game_state, &mut self.game_manager);
            });

        if self.game_manager.timer.is_running()
            && self.game_state.game_started
            && !self.game_state.game_over
        {
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
        }
    }
}
