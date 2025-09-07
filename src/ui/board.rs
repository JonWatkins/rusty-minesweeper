use crate::game::GameBoard;
use crate::game::{CellContent, CellState};
use crate::game_manager::GameManager;
use crate::theme::Palette;
use crate::utils::is_show_mines_enabled;
use egui::{Color32, Painter, Rect};

pub struct BoardRenderer {
    cell_size: f32,
}

impl BoardRenderer {
    pub fn new() -> Self {
        Self { cell_size: 30.0 }
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        game_state: &mut GameBoard,
        game_manager: &GameManager,
    ) {
        let toolbar_height = 60.0;
        let padding = 40.0;
        let available_width = ui.available_width() - padding;
        let available_height = ui.available_height() - toolbar_height - padding;

        let max_cell_width = available_width / game_state.width as f32;
        let max_cell_height = available_height / game_state.height as f32;
        let responsive_cell_size = max_cell_width.min(max_cell_height).clamp(20.0, 50.0);

        let original_cell_size = self.cell_size;
        self.cell_size = responsive_cell_size;

        let board_width = game_state.width as f32 * self.cell_size;
        let board_height = game_state.height as f32 * self.cell_size;

        let center_x = ui.available_width() / 2.0;
        let center_y = (ui.available_height() + toolbar_height) / 2.0;
        let board_start_x = center_x - board_width / 2.0;
        let board_start_y = center_y - board_height / 2.0;

        let board_rect = egui::Rect::from_min_size(
            egui::pos2(board_start_x, board_start_y),
            egui::vec2(board_width, board_height),
        );

        let response = ui.allocate_rect(board_rect, egui::Sense::click());

        let painter = ui.painter();

        for y in 0..game_state.height {
            for x in 0..game_state.width {
                let cell_x = board_start_x + x as f32 * self.cell_size;
                let cell_y = board_start_y + y as f32 * self.cell_size;

                let cell_rect = egui::Rect::from_min_size(
                    egui::pos2(cell_x, cell_y),
                    egui::vec2(self.cell_size, self.cell_size),
                );

                self.render_cell(painter, game_state, x, y, cell_rect);
            }
        }

        if !game_state.game_over && game_manager.timer.is_running() {
            if let Some(click_pos) = response.interact_pointer_pos() {
                let relative_x = click_pos.x - board_start_x;
                let relative_y = click_pos.y - board_start_y;

                if relative_x >= 0.0 && relative_y >= 0.0 {
                    let cell_x = (relative_x / self.cell_size) as usize;
                    let cell_y = (relative_y / self.cell_size) as usize;

                    if cell_x < game_state.width && cell_y < game_state.height {
                        if response.ctx.input(|i| i.pointer.secondary_clicked()) {
                            game_state.toggle_flag(cell_x, cell_y);
                        } else if response.ctx.input(|i| i.pointer.primary_clicked()) {
                            game_state.reveal_cell(cell_x, cell_y);
                        }
                    }
                }
            }
        }

        self.cell_size = original_cell_size;
    }

    fn render_cell(
        &self,
        painter: &Painter,
        game_state: &GameBoard,
        x: usize,
        y: usize,
        rect: Rect,
    ) {
        let cell = &game_state.board[y][x];
        let palette = Palette::default();
        let show_mines = is_show_mines_enabled();

        let (text, bg_color, text_color) = match cell.state {
            CellState::Hidden => {
                // Show mines in debug mode even when hidden
                if show_mines && matches!(cell.content, CellContent::Mine) {
                    (
                        "💣".to_string(),
                        Color32::from_rgb(255, 100, 100),
                        Color32::WHITE,
                    )
                } else {
                    ("".to_string(), palette.surface_3, palette.text)
                }
            }
            CellState::Flagged => ("🚩".to_string(), palette.surface_3, palette.text),
            CellState::Revealed => match cell.content {
                CellContent::Empty => ("".to_string(), palette.surface_0, palette.text),
                CellContent::Number(n) => {
                    let color = match n {
                        1 => Color32::from_rgb(96, 165, 250),
                        2 => Color32::from_rgb(74, 222, 128),
                        3 => Color32::from_rgb(248, 113, 113),
                        4 => Color32::from_rgb(147, 197, 253),
                        5 => Color32::from_rgb(248, 113, 113),
                        6 => Color32::from_rgb(45, 212, 191),
                        7 => Color32::from_rgb(234, 179, 8),
                        8 => Color32::from_gray(170),
                        _ => palette.text,
                    };
                    (n.to_string(), palette.surface_0, color)
                }
                CellContent::Mine => ("💣".to_string(), palette.danger, Color32::WHITE),
            },
        };

        painter.rect_filled(rect, egui::Rounding::same(2.0), bg_color);

        painter.rect_stroke(
            rect,
            egui::Rounding::same(2.0),
            egui::Stroke::new(1.0, Color32::from_rgba_premultiplied(255, 255, 255, 20)),
        );

        if !text.is_empty() {
            let text_size = self.cell_size * 0.6;
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                text,
                egui::FontId::proportional(text_size),
                text_color,
            );
        }
    }
}
