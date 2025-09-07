use crate::game::GameBoard;
use crate::game_manager::{Difficulty, GameManager};
use crate::theme::Palette;
use egui::{Color32, RichText, Ui};

pub struct WelcomeScreen;

impl WelcomeScreen {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, ui: &mut Ui, game_state: &mut GameBoard, game_manager: &mut GameManager) {
        let palette = Palette::default();
        ui.allocate_ui_with_layout(
            egui::Vec2::new(ui.available_width(), ui.available_height()),
            egui::Layout::top_down(egui::Align::Center),
            |ui| {
                ui.add_space(ui.available_height() * 0.12);

                egui::Frame::default()
                    .fill(palette.surface_1)
                    .stroke(egui::Stroke::new(1.0, Color32::WHITE))
                    .rounding(egui::Rounding::same(12.0))
                    .inner_margin(egui::style::Margin::symmetric(28.0, 24.0))
                    .show(ui, |ui| {
                        ui.set_max_width(560.0);
                        ui.vertical_centered(|ui| {
                            ui.heading(
                                RichText::new("Welcome to Minesweeper!")
                                    .size(32.0)
                                    .color(palette.text),
                            );
                            ui.add_space(16.0);
                            ui.label(
                                RichText::new("Click cells to reveal numbers; avoid the mines.")
                                    .size(16.0)
                                    .color(palette.text_muted),
                            );
                            ui.label(
                                RichText::new("Right-click to flag suspected mines.")
                                    .size(16.0)
                                    .color(palette.text_muted),
                            );

                            ui.add_space(24.0);
                            ui.label(
                                RichText::new("Choose your difficulty")
                                    .size(18.0)
                                    .color(palette.text)
                                    .strong(),
                            );
                            ui.add_space(12.0);

                            ui.horizontal(|ui| {
                                let current_difficulty = game_manager.current_difficulty;

                                let beginner_selected =
                                    matches!(current_difficulty, Difficulty::Beginner);
                                let mut beginner_button = Self::primary_button(
                                    "Beginner\n9×9, 10 mines",
                                    if beginner_selected {
                                        palette.accent
                                    } else {
                                        palette.surface_2
                                    },
                                )
                                .min_size(egui::Vec2::new(160.0, 64.0));
                                if beginner_selected {
                                    beginner_button = beginner_button
                                        .stroke(egui::Stroke::new(1.0, palette.accent_soft));
                                }
                                if ui.add(beginner_button).clicked() {
                                    self.change_difficulty(
                                        game_state,
                                        game_manager,
                                        Difficulty::Beginner,
                                    );
                                }

                                let intermediate_selected =
                                    matches!(current_difficulty, Difficulty::Intermediate);
                                let mut intermediate_button = Self::primary_button(
                                    "Intermediate\n16×16, 40 mines",
                                    if intermediate_selected {
                                        palette.accent
                                    } else {
                                        palette.surface_2
                                    },
                                )
                                .min_size(egui::Vec2::new(180.0, 64.0));
                                if intermediate_selected {
                                    intermediate_button = intermediate_button
                                        .stroke(egui::Stroke::new(1.0, palette.accent_soft));
                                }
                                if ui.add(intermediate_button).clicked() {
                                    self.change_difficulty(
                                        game_state,
                                        game_manager,
                                        Difficulty::Intermediate,
                                    );
                                }

                                let expert_selected =
                                    matches!(current_difficulty, Difficulty::Expert);
                                let mut expert_button = Self::primary_button(
                                    "Expert\n30×16, 99 mines",
                                    if expert_selected {
                                        palette.accent
                                    } else {
                                        palette.surface_2
                                    },
                                )
                                .min_size(egui::Vec2::new(160.0, 64.0));
                                if expert_selected {
                                    expert_button = expert_button
                                        .stroke(egui::Stroke::new(1.0, palette.accent_soft));
                                }
                                if ui.add(expert_button).clicked() {
                                    self.change_difficulty(
                                        game_state,
                                        game_manager,
                                        Difficulty::Expert,
                                    );
                                }
                            });

                            ui.add_space(24.0);
                            let start_button =
                                Self::primary_button("🎮 Start New Game", palette.success)
                                    .min_size(egui::Vec2::new(220.0, 48.0));
                            if ui.add(start_button).clicked() {
                                game_state.start_game();
                                game_manager.start_game(game_manager.current_difficulty);
                            }
                        });
                    });
            },
        );
    }

    fn primary_button(
        label: impl Into<egui::WidgetText> + 'static,
        fill: Color32,
    ) -> egui::Button<'static> {
        let widget_text = label.into();

        // Use white text for dark backgrounds (surface_2), dark text for light backgrounds
        let palette = Palette::default();
        let text_color = if fill == palette.surface_2 {
            Color32::WHITE
        } else {
            Color32::from_rgb(20, 20, 20)
        };

        let rich_text = egui::RichText::new(widget_text.text()).color(text_color);

        egui::Button::new(rich_text)
            .min_size(egui::Vec2::new(120.0, 32.0))
            .rounding(egui::Rounding::same(8.0))
            .fill(fill)
            .stroke(egui::Stroke::new(0.0, Color32::TRANSPARENT))
    }

    fn change_difficulty(
        &self,
        game_state: &mut GameBoard,
        game_manager: &mut GameManager,
        difficulty: Difficulty,
    ) {
        let (width, height, mines) = difficulty.get_dimensions();
        *game_state = GameBoard::new(width, height, mines);
        game_manager.current_difficulty = difficulty;
    }
}
