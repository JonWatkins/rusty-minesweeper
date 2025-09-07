use crate::game::GameBoard;
use crate::game_manager::{Difficulty, GameManager};
use crate::theme::Palette;
use crate::utils::format_time;
use egui::{Color32, RichText, Ui};

pub struct GameOverOverlay;

impl GameOverOverlay {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, ui: &mut Ui, game_state: &mut GameBoard, game_manager: &mut GameManager) {
        let palette = Palette::default();

        let screen_rect = ui.ctx().screen_rect();
        let painter = ui.painter();
        painter.rect_filled(
            screen_rect,
            egui::Rounding::same(0.0),
            Color32::from_rgba_premultiplied(0, 0, 0, 128),
        );

        let modal_width = 400.0;
        let modal_height = 400.0;
        let center_x = screen_rect.center().x;
        let center_y = screen_rect.center().y;
        let modal_rect = egui::Rect::from_center_size(
            egui::pos2(center_x, center_y),
            egui::vec2(modal_width, modal_height),
        );

        painter.rect_filled(modal_rect, egui::Rounding::same(12.0), palette.surface_1);

        painter.rect_stroke(
            modal_rect,
            egui::Rounding::same(12.0),
            egui::Stroke::new(1.0, palette.border_soft),
        );

        ui.allocate_ui_at_rect(modal_rect, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);

                ui.heading(
                    RichText::new("💥 Game Over!")
                        .size(36.0)
                        .color(palette.text),
                );

                ui.add_space(30.0);

                ui.group(|ui| {
                    ui.style_mut().visuals.panel_fill = palette.surface_2;
                    ui.vertical_centered(|ui| {
                        ui.label(
                            RichText::new("Your Score")
                                .size(20.0)
                                .color(palette.text)
                                .strong(),
                        );
                        ui.add_space(10.0);
                        ui.label(
                            RichText::new(format!(
                                "Time: {}",
                                format_time(game_manager.timer.get_elapsed())
                            ))
                            .size(24.0)
                            .color(palette.accent),
                        );
                        ui.add_space(5.0);
                        ui.label(
                            RichText::new(format!(
                                "Difficulty: {}",
                                self.get_difficulty_name(game_manager.current_difficulty)
                            ))
                            .size(16.0)
                            .color(palette.text_muted),
                        );
                    });
                });

                ui.add_space(40.0);

                ui.label(
                    RichText::new("You hit a mine! Better luck next time.")
                        .size(16.0)
                        .color(palette.text),
                );

                ui.add_space(20.0);

                ui.horizontal_centered(|ui| {
                    ui.add_space(20.0);

                    let play_again_button = Self::primary_button("🔄 Play Again", palette.success)
                        .min_size(egui::Vec2::new(160.0, 48.0));

                    if ui.add(play_again_button).clicked() {
                        game_manager.reset_game(&mut *game_state);
                        game_manager.start_game(game_manager.current_difficulty);
                    }

                    ui.add_space(16.0);

                    let main_menu_button = Self::primary_button("🏠 Main Menu", palette.accent)
                        .min_size(egui::Vec2::new(160.0, 48.0));

                    if ui.add(main_menu_button).clicked() {
                        game_state.reset();
                        game_manager.pause_game();
                    }

                    ui.add_space(20.0);
                });

                ui.add_space(20.0);
            });
        });
    }

    fn primary_button(
        label: impl Into<egui::WidgetText> + 'static,
        fill: Color32,
    ) -> egui::Button<'static> {
        let widget_text = label.into();
        let rich_text =
            egui::RichText::new(widget_text.text()).color(Color32::from_rgb(20, 20, 20));

        egui::Button::new(rich_text)
            .min_size(egui::Vec2::new(120.0, 32.0))
            .rounding(egui::Rounding::same(8.0))
            .fill(fill)
            .stroke(egui::Stroke::new(0.0, Color32::TRANSPARENT))
    }

    fn get_difficulty_name(&self, difficulty: Difficulty) -> &'static str {
        match difficulty {
            Difficulty::Beginner => "Beginner",
            Difficulty::Intermediate => "Intermediate",
            Difficulty::Expert => "Expert",
        }
    }
}

pub struct PauseOverlay;

impl PauseOverlay {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, ui: &mut Ui, game_state: &mut GameBoard, game_manager: &mut GameManager) {
        let palette = Palette::default();

        let screen_rect = ui.ctx().screen_rect();
        let painter = ui.painter();
        painter.rect_filled(
            screen_rect,
            egui::Rounding::same(0.0),
            Color32::from_rgba_premultiplied(0, 0, 0, 128),
        );

        let modal_width = 300.0;
        let modal_height = 280.0;
        let center_x = screen_rect.center().x;
        let center_y = screen_rect.center().y;
        let modal_rect = egui::Rect::from_center_size(
            egui::pos2(center_x, center_y),
            egui::vec2(modal_width, modal_height),
        );

        painter.rect_filled(modal_rect, egui::Rounding::same(12.0), palette.surface_1);

        painter.rect_stroke(
            modal_rect,
            egui::Rounding::same(12.0),
            egui::Stroke::new(1.0, palette.border_soft),
        );

        ui.allocate_ui_at_rect(modal_rect, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.heading(RichText::new("Paused").size(32.0).color(palette.text));
                ui.add_space(15.0);

                ui.vertical_centered(|ui| {
                    let resume = Self::primary_button("Resume", palette.success)
                        .min_size(egui::Vec2::new(200.0, 48.0));
                    if ui.add(resume).clicked() {
                        game_manager.resume_game();
                    }
                    ui.add_space(12.0);

                    let new_game = Self::primary_button("🔄 New Game", palette.accent)
                        .min_size(egui::Vec2::new(200.0, 48.0));
                    if ui.add(new_game).clicked() {
                        game_manager.reset_game(&mut *game_state);
                        game_manager.start_game(game_manager.current_difficulty);
                    }
                    ui.add_space(12.0);

                    let end_game = Self::primary_button("🏁 End Game", palette.danger)
                        .min_size(egui::Vec2::new(200.0, 48.0));
                    if ui.add(end_game).clicked() {
                        game_state.reset();
                        game_manager.pause_game();
                    }
                });
                ui.add_space(20.0);
            });
        });
    }

    fn primary_button(
        label: impl Into<egui::WidgetText> + 'static,
        fill: Color32,
    ) -> egui::Button<'static> {
        let widget_text = label.into();
        let rich_text =
            egui::RichText::new(widget_text.text()).color(Color32::from_rgb(20, 20, 20));

        egui::Button::new(rich_text)
            .min_size(egui::Vec2::new(120.0, 32.0))
            .rounding(egui::Rounding::same(8.0))
            .fill(fill)
            .stroke(egui::Stroke::new(0.0, Color32::TRANSPARENT))
    }
}

pub struct WinOverlay;

impl WinOverlay {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, ui: &mut Ui, game_state: &mut GameBoard, game_manager: &mut GameManager) {
        let palette = Palette::default();

        let screen_rect = ui.ctx().screen_rect();
        let painter = ui.painter();
        painter.rect_filled(
            screen_rect,
            egui::Rounding::same(0.0),
            Color32::from_rgba_premultiplied(0, 0, 0, 128),
        );

        let modal_width = 400.0;
        let modal_height = 400.0;
        let center_x = screen_rect.center().x;
        let center_y = screen_rect.center().y;
        let modal_rect = egui::Rect::from_center_size(
            egui::pos2(center_x, center_y),
            egui::vec2(modal_width, modal_height),
        );

        painter.rect_filled(modal_rect, egui::Rounding::same(12.0), palette.surface_1);

        painter.rect_stroke(
            modal_rect,
            egui::Rounding::same(12.0),
            egui::Stroke::new(1.0, palette.success),
        );

        ui.allocate_ui_at_rect(modal_rect, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);

                ui.heading(
                    RichText::new("🎉 You Win!")
                        .size(36.0)
                        .color(palette.success),
                );

                ui.add_space(30.0);

                ui.group(|ui| {
                    ui.style_mut().visuals.panel_fill = palette.surface_2;
                    ui.vertical_centered(|ui| {
                        ui.label(
                            RichText::new("Your Score")
                                .size(20.0)
                                .color(palette.text)
                                .strong(),
                        );
                        ui.add_space(10.0);
                        ui.label(
                            RichText::new(format!(
                                "Time: {}",
                                format_time(game_manager.timer.get_elapsed())
                            ))
                            .size(24.0)
                            .color(palette.success),
                        );
                        ui.add_space(5.0);
                        ui.label(
                            RichText::new(format!(
                                "Difficulty: {}",
                                self.get_difficulty_name(game_manager.current_difficulty)
                            ))
                            .size(16.0)
                            .color(palette.text_muted),
                        );
                    });
                });

                ui.add_space(40.0);

                ui.label(
                    RichText::new("Congratulations! You cleared all the mines!")
                        .size(16.0)
                        .color(palette.text),
                );

                ui.add_space(20.0);

                ui.horizontal_centered(|ui| {
                    ui.add_space(20.0);

                    let play_again_button = Self::primary_button("🔄 Play Again", palette.success)
                        .min_size(egui::Vec2::new(160.0, 48.0));

                    if ui.add(play_again_button).clicked() {
                        game_manager.reset_game(&mut *game_state);
                        game_manager.start_game(game_manager.current_difficulty);
                    }

                    ui.add_space(16.0);

                    let main_menu_button = Self::primary_button("🏠 Main Menu", palette.accent)
                        .min_size(egui::Vec2::new(160.0, 48.0));

                    if ui.add(main_menu_button).clicked() {
                        game_state.reset();
                        game_manager.pause_game();
                    }

                    ui.add_space(20.0);
                });

                ui.add_space(20.0);
            });
        });
    }

    fn primary_button(
        label: impl Into<egui::WidgetText> + 'static,
        fill: Color32,
    ) -> egui::Button<'static> {
        let widget_text = label.into();
        let rich_text =
            egui::RichText::new(widget_text.text()).color(Color32::from_rgb(20, 20, 20));

        egui::Button::new(rich_text)
            .min_size(egui::Vec2::new(120.0, 32.0))
            .rounding(egui::Rounding::same(8.0))
            .fill(fill)
            .stroke(egui::Stroke::new(0.0, Color32::TRANSPARENT))
    }

    fn get_difficulty_name(&self, difficulty: Difficulty) -> &'static str {
        match difficulty {
            Difficulty::Beginner => "Beginner",
            Difficulty::Intermediate => "Intermediate",
            Difficulty::Expert => "Expert",
        }
    }
}
