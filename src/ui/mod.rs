pub mod board;
pub mod overlays;
pub mod welcome;

pub use board::BoardRenderer;
pub use overlays::{GameOverOverlay, PauseOverlay, WinOverlay};
pub use welcome::WelcomeScreen;

use crate::game::GameBoard;
use crate::game_manager::GameManager;
use egui::Ui;

pub struct GameUI {
    board_renderer: BoardRenderer,
    welcome_screen: WelcomeScreen,
    game_over_overlay: GameOverOverlay,
    win_overlay: WinOverlay,
    pause_overlay: PauseOverlay,
}

impl GameUI {
    pub fn new() -> Self {
        Self {
            board_renderer: BoardRenderer::new(),
            welcome_screen: WelcomeScreen::new(),
            game_over_overlay: GameOverOverlay::new(),
            win_overlay: WinOverlay::new(),
            pause_overlay: PauseOverlay::new(),
        }
    }

    pub fn render(
        &mut self,
        ui: &mut Ui,
        game_state: &mut GameBoard,
        game_manager: &mut GameManager,
    ) {
        if game_state.game_started {
            egui::TopBottomPanel::top("toolbar")
                .frame(
                    egui::Frame::default()
                        .fill(if game_state.game_over && !game_state.game_won {
                            egui::Color32::from_rgba_premultiplied(30, 32, 38, 200)
                        } else if game_state.game_over && game_state.game_won {
                            egui::Color32::from_rgba_premultiplied(34, 139, 34, 200)
                        } else {
                            egui::Color32::from_rgb(24, 26, 33)
                        })
                        .rounding(egui::Rounding::same(0.0))
                        .inner_margin(egui::style::Margin::symmetric(16.0, 12.0)),
                )
                .show_inside(ui, |ui| {
                    let palette = crate::theme::Palette::default();
                    ui.horizontal(|ui| {
                        ui.heading(
                            egui::RichText::new("💣 Minesweeper")
                                .color(palette.text)
                                .strong()
                                .size(24.0),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                egui::RichText::new(crate::utils::format_time(
                                    game_manager.timer.get_elapsed(),
                                ))
                                .color(palette.text)
                                .strong()
                                .size(18.0),
                            );
                        });
                    });
                });

            self.board_renderer.render(ui, game_state, game_manager);

            if game_state.game_over && !game_state.game_won {
                self.game_over_overlay.render(ui, game_state, game_manager);
            } else if game_state.game_over && game_state.game_won {
                self.win_overlay.render(ui, game_state, game_manager);
            } else if game_state.game_started
                && !game_state.game_over
                && !game_manager.timer.is_running()
            {
                self.pause_overlay.render(ui, game_state, game_manager);
            }
        } else {
            self.welcome_screen.render(ui, game_state, game_manager);
        }
    }
}

impl Default for GameUI {
    fn default() -> Self {
        Self::new()
    }
}
